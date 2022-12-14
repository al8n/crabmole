/// Pipe
#[cfg(feature = "pipe")]
#[cfg_attr(docsrs, doc(cfg(feature = "pipe")))]
pub mod pipe;

/// Pipe
#[cfg(feature = "async-pipe")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-pipe")))]
pub mod async_pipe;

/// Error for pipe
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg(any(feature = "pipe", feature = "async-pipe"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "pipe", feature = "async-pipe"))))]
pub enum PipeError {
    /// Read/Write on closed pipe
    Closed,
    /// EOF
    Eof,
}

impl PipeError {
    /// Returns true if the error is [`PipeError::Eof`]
    #[inline]
    pub const fn is_eof(&self) -> bool {
        matches!(self, Self::Eof)
    }

    /// Returns true if the error is [`PipeError::Closed`]
    #[inline]
    pub const fn is_closed(&self) -> bool {
        matches!(self, Self::Closed)
    }
}

impl core::fmt::Display for PipeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PipeError::Closed => write!(f, "Pipe: read/write on closed pipe"),
            PipeError::Eof => write!(f, "Pipe: EOF"),
        }
    }
}

impl std::error::Error for PipeError {}

/// The trait that groups the basic [`std::io::Read`] and [`std::io::Write`].
pub trait ReadWriter: std::io::Write + std::io::Read {}

impl<T: std::io::Read + std::io::Write> ReadWriter for T {}

/// The trait that groups the basic [`std::io::Read`] and [`Closer`].
pub trait ReadCloser: std::io::Read + Closer {}

impl<T: std::io::Read + Closer> ReadCloser for T {}

/// The trait that groups the basic [`std::io::Write`] and [`Closer`].
pub trait WriteCloser: std::io::Write + Closer {}

impl<T: std::io::Write + Closer> WriteCloser for T {}

/// The trait that groups the basic [`std::io::Read`], [`std::io::Write`] and [`Closer`].
pub trait ReadWriteCloser: std::io::Read + Closer {}

/// Closer is the trait that wraps the basic `close` method.
///
/// The behavior of `close` after the first call is undefined.
/// Specific implementations may document their own behavior.
pub trait Closer {
    /// Close
    fn close(&mut self) -> std::io::Result<()>;
}
