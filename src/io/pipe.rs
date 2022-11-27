use super::PipeError;
use std::sync::Once;

use alloc::sync::Arc;
use crossbeam_channel::{bounded, select, unbounded, Receiver, Sender};
use parking_lot::Mutex;

#[derive(Debug)]
#[repr(transparent)]
struct OnceError {
    err: Mutex<Option<PipeError>>,
}

impl OnceError {
    #[inline]
    const fn new() -> Self {
        Self {
            err: Mutex::new(None),
        }
    }

    #[inline]
    fn store(&self, e: PipeError) {
        let mut err = self.err.lock();
        if err.is_none() {
            *err = Some(e);
        }
    }

    #[inline]
    fn load(&self) -> Option<PipeError> {
        let err = self.err.lock();
        *err
    }
}

/// A [`PipeReader`] is the read half of a pipe.
pub struct PipeReader {
    wr_rx: Receiver<Vec<u8>>,
    rd_tx: Sender<usize>,
    inner: Arc<Inner>,
}

impl PipeReader {
    /// Read implements the standard Read interface:
    /// it reads data from the pipe, blocking until a writer
    /// arrives or the write end is closed.
    /// If the write end is closed with an error, that error is
    /// returned as err; otherwise err is EOF.
    pub fn read(&self, buf: &mut [u8]) -> Result<usize, PipeError> {
        select! {
            recv(self.inner.done_rx) -> _ => Err(self.read_close_error()),
            default => {
                select! {
                    recv(self.wr_rx) -> msg => {
                        match msg {
                            Ok(mut msg) => {
                                let n = crate::copy(buf, &mut msg);
                                self.rd_tx.send(n).unwrap();
                                Ok(n)
                            }
                            Err(_) => {
                                Err(PipeError::Closed)
                            }
                        }
                    }
                    recv(self.inner.done_rx) -> _ => {
                        Err(self.read_close_error())
                    }
                }
            }
        }
    }

    #[inline]
    fn read_close_error(&self) -> PipeError {
        let rerr = self.inner.rerr.load();
        let werr = self.inner.werr.load();
        match (rerr, werr) {
            (None, Some(err)) => err,
            _ => PipeError::Closed,
        }
    }

    #[inline]
    fn close_read(&self) {
        self.inner.rerr.store(PipeError::Closed);
        self.inner.once.call_once(|| {
            // unwrap safe here because PipeReader also holds a receiver channel.
            self.inner.done_tx.send(()).unwrap();
        });
    }
}

impl Drop for PipeReader {
    fn drop(&mut self) {
        self.close_read();
    }
}

/// A [`PipeWriter`] is the write half of a pipe.
#[derive(Debug)]
pub struct PipeWriter {
    mu: Mutex<()>,
    wr_tx: Sender<Vec<u8>>,
    rd_rx: Receiver<usize>,
    inner: Arc<Inner>,
}

impl PipeWriter {
    /// Implements the standard Write interface:
    /// it writes data to the pipe, blocking until one or more readers
    /// have consumed all the data or the read end is closed.
    /// If the read end is closed with an error, that err is
    /// returned as err; otherwise err is ErrClosedPipe.
    pub fn write(&self, mut buf: &[u8]) -> Result<usize, PipeError> {
        select! {
            recv(self.inner.done_rx) -> _ => {
                Err(self.write_close_error())
            }
            default => {
                let _mu = self.mu.lock();
                let mut n = 0;
                let mut once = true;
                while once || !buf.is_empty() {
                    select! {
                        recv(self.inner.done_rx) -> _ => {
                            return Err(self.write_close_error());
                        }
                        send(self.wr_tx, buf.to_vec()) -> _ => {
                            match self.rd_rx.recv() {
                                Ok(nw) => {
                                    buf = &buf[nw..];
                                    n += nw;
                                },
                                Err(_) => {
                                    return Err(PipeError::Closed);
                                }
                            }
                        }
                    }
                    once = false;
                }
                Ok(n)
            }
        }
    }

    #[inline]
    fn close_write(&self) {
        self.inner.werr.store(PipeError::Eof);
        self.inner.once.call_once(|| {
            // unwrap safe here because PipeWriter also holds a receiver channel.
            self.inner.done_tx.send(()).unwrap();
        });
    }

    #[inline]
    fn write_close_error(&self) -> PipeError {
        let rerr = self.inner.rerr.load();
        let werr = self.inner.werr.load();
        match (rerr, werr) {
            (Some(err), _) => err,
            _ => PipeError::Closed,
        }
    }
}

impl Drop for PipeWriter {
    fn drop(&mut self) {
        self.close_write();
    }
}

#[derive(Debug)]
struct Inner {
    done_tx: Sender<()>,
    done_rx: Receiver<()>,
    once: Once,
    rerr: OnceError,
    werr: OnceError,
}

/// Creates a synchronous in-memory pipe.
/// It can be used to connect code expecting an io.Reader
/// with code expecting an io.Writer.
///
/// Reads and Writes on the pipe are matched one to one
/// except when multiple Reads are needed to consume a single Write.
/// That is, each Write to the PipeWriter blocks until it has satisfied
/// one or more Reads from the PipeReader that fully consume
/// the written data.
/// The data is copied directly from the Write to the corresponding
/// Read (or Reads); there is no internal buffering.
///
/// It is safe to call `read` and `write` in parallel with each other or with `close`.
/// Parallel calls to Read and parallel calls to Write are also safe:
/// the individual calls will be gated sequentially.
pub fn pipe() -> (PipeReader, PipeWriter) {
    let (wr_tx, wr_rx) = unbounded();
    let (rd_tx, rd_rx) = unbounded();
    let (done_tx, done_rx) = bounded(1);
    let inner = Arc::new(Inner {
        done_tx,
        done_rx,
        once: Once::new(),
        rerr: OnceError::new(),
        werr: OnceError::new(),
    });
    (
        PipeReader {
            wr_rx,
            rd_tx,
            inner: inner.clone(),
        },
        PipeWriter {
            mu: Mutex::new(()),
            wr_tx,
            rd_rx,
            inner,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_write(w: PipeWriter, data: Vec<u8>, c: Sender<usize>) {
        let n = w.write(&data).unwrap();
        assert_eq!(n, data.len(), "short write: {} != {}", n, data.len());
        c.send(0).unwrap();
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_pipe1() {
        let (tx, rx) = bounded(1);
        let (r, w) = pipe();
        let mut buf = vec![0; 64];
        std::thread::spawn(|| {
            check_write(w, b"hello, world".to_vec(), tx);
        });

        let n = r.read(&mut buf).unwrap();
        assert_eq!(n, 12, "bad read: got {}", String::from_utf8_lossy(&buf));
        rx.recv().unwrap();
    }

    fn reader(r: PipeReader, c: Sender<usize>) {
        let mut buf = vec![0; 64];
        loop {
            match r.read(&mut buf) {
                Ok(n) => {
                    c.send(n).unwrap();
                }
                Err(e) => {
                    if e.is_eof() {
                        c.send(0).unwrap();
                        break;
                    }
                }
            }
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn test_pipe2() {
        let (tx, rx) = bounded(1);
        let (r, w) = pipe();
        let buf = vec![0; 64];
        std::thread::spawn(|| {
            reader(r, tx);
        });

        for i in 0..5 {
            let p = &buf[..5 + i * 10];
            let n = w.write(p).unwrap();
            assert_eq!(n, p.len(), "wrote {}, got {}", p.len(), n);
            let nn = rx.recv().unwrap();
            assert_eq!(nn, n, "wrote {}, read got {}", n, nn);
        }

        drop(w);
        let nn = rx.recv().unwrap();
        assert_eq!(nn, 0, "final read got {}", nn);
    }
}
