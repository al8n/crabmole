const HEX_TABLE: [u8; 16] = *b"0123456789abcdef";

const REVERSE_HASH_TABLE: [u8; 256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255, 255, 255,
    255, 255, 255, 255, 10, 11, 12, 13, 14, 15, 7, 8, 9, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 10, 11, 12, 13, 14,
    15, 7, 8, 9, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255,
];

/// Hex related errors
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    /// Reports an attempt to decode an odd-length input
    /// using Decode or DecodeString.
    Length,
    /// Values describe errors resulting from an invalid byte in a hex string.
    InvalidByte {
        /// The position of the `src`
        index: usize,
        /// The invalid byte
        byte: u8,
    },
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::Length => write!(f, "encoding/hex: odd length hex string"),
            Error::InvalidByte { index, byte } => write!(
                f,
                "encoding/hex: invalid byte {} at position {}",
                byte, index
            ),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Returns the length of an encoding of n source bytes.
/// Specifically, it returns n * 2.
#[inline]
pub const fn encoded_len(n: usize) -> usize {
    n * 2
}

/// Encodes src into `encoded_len(src.len())`
/// bytes of dst. As a convenience, it returns the number
/// of bytes written to dst, but this value is always `encoded_len(src.len())`.
/// Encode implements hexadecimal encoding.
#[inline]
pub fn encode(src: &[u8], dst: &mut [u8]) -> usize {
    let mut j = 0;
    for &byte in src {
        dst[j] = HEX_TABLE[(byte >> 4) as usize];
        dst[j + 1] = HEX_TABLE[(byte & 0x0f) as usize];
        j += 2;
    }
    src.len() * 2
}

/// Returns the length of a decoding of x source bytes.
/// Specifically, it returns x / 2.
#[inline]
pub const fn decode_len(n: usize) -> usize {
    n / 2
}

/// Decodes src into DecodedLen(len(src)) bytes,
/// returning the actual number of bytes written to dst.
///
/// Decode expects that src contains only hexadecimal
/// characters and that src has even length.
/// If the input is malformed, Decode returns the number
/// of bytes decoded before the error.
#[inline]
pub fn decode(src: &[u8], dst: &mut [u8]) -> Result<usize, Error> {
    let (mut i, mut j) = (0, 1);
    while j < src.len() {
        let p = src[j - 1];
        let q = src[j];

        let a = REVERSE_HASH_TABLE[p as usize];
        let b = REVERSE_HASH_TABLE[q as usize];

        if a > 0x0f {
            return Err(Error::InvalidByte { index: i, byte: p });
        }

        if b > 0x0f {
            return Err(Error::InvalidByte { index: i, byte: q });
        }

        dst[i] = (a << 4) | b;
        i += 1;
        j += 2;
    }

    if src.len() % 2 == 1 &&
    // Check for invalid char before reporting bad length,
	// since the invalid char (if present) is an earlier problem.
    REVERSE_HASH_TABLE[src[j-1] as usize] > 0x0f
    {
        return Err(Error::InvalidByte {
            index: i,
            byte: src[j - 1],
        });
    }

    Ok(i)
}

/// Returns the hexadecimal encoding of src.
#[cfg(feature = "alloc")]
#[inline]
pub fn encode_to_vec(src: &[u8]) -> alloc::vec::Vec<u8> {
    let mut dst = alloc::vec![0; encoded_len(src.len())];
    encode(src, &mut dst);
    dst
}

/// Returns the bytes represented by the hexadecimal `src`.
///
/// [`decode_to_vec`] expects that src contains only hexadecimal
/// characters and that src has even length.
/// If the input is malformed, [`decode_to_vec`] returns
/// the bytes decoded before the error.
#[cfg(feature = "alloc")]
#[inline]
pub fn decode_to_vec(src: &[u8]) -> Result<alloc::vec::Vec<u8>, Error> {
    let mut dst = alloc::vec![0; decode_len(src.len())];
    decode(src, &mut dst).map(|u| {
        dst.truncate(u);
        dst
    })
}

/// Returns a [`Vec<u8>`] that contains a hex dump of the given data. The format
/// of the hex dump matches the output of `hexdump -C` on the command line.
#[cfg(feature = "std")]
#[inline]
pub fn dump(src: &[u8]) -> std::io::Result<alloc::vec::Vec<u8>> {
    use crate::io::Closer;
    use std::io::Write;

    if src.is_empty() {
        return Ok(alloc::vec![]);
    }

    // Dumper will write 79 bytes per complete 16 byte chunk, and at least
    // 64 bytes for whatever remains. Round the allocation up, since only a
    // maximum of 15 bytes will be wasted.
    let mut dst = alloc::vec![0; (1 + ((src.len() - 1) / 16)) * 79];

    let mut dumper = Dumper::new(&mut dst);
    dumper
        .write_all(src)
        .and_then(|_| dumper.close())
        .map(|_| dst)
}

const BUFFER_SIZE: usize = 1024;

/// Hex encoder
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Encoder<W> {
    w: W,
    out: [u8; BUFFER_SIZE],
}

impl<W> Encoder<W> {
    /// Returns an encoder that writes lowercase hexadecimal characters to w.
    #[inline]
    pub const fn new(w: W) -> Self {
        Self {
            w,
            out: [0; BUFFER_SIZE],
        }
    }
}

#[cfg(feature = "std")]
impl<W: std::io::Write> std::io::Write for Encoder<W> {
    #[inline]
    fn write(&mut self, mut src: &[u8]) -> std::io::Result<usize> {
        let mut n = 0;
        while !src.is_empty() {
            let chunk_size = (BUFFER_SIZE / 2).min(src.len());
            let encoded = encode(&src[..chunk_size], &mut self.out);
            let written = self.w.write(&self.out[..encoded])?;
            n += written / 2;
            src = &src[chunk_size..];
        }
        Ok(n)
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        self.w.flush()
    }
}

/// Hex decoder
#[cfg(feature = "alloc")]
pub struct Decoder<R> {
    r: R,
    in_: alloc::vec::Vec<u8>,
    arr: [u8; BUFFER_SIZE],
}

#[cfg(feature = "alloc")]
impl<R> Decoder<R> {
    /// Returns an decoder that reads hexadecimal characters from r.
    #[inline]
    pub const fn new(r: R) -> Self {
        Self {
            r,
            in_: alloc::vec::Vec::new(),
            arr: [0; BUFFER_SIZE],
        }
    }
}

#[cfg(feature = "std")]
impl<R: std::io::Read> std::io::Read for Decoder<R> {
    #[inline]
    fn read(&mut self, mut dst: &mut [u8]) -> std::io::Result<usize> {
        if self.in_.len() < 2 {
            let num_copy = crate::copy(&self.in_, &mut self.arr);
            let num_read = self.r.read(&mut self.arr[num_copy..])?;
            // TODO: optimize allocation here, we do not need to allocate full copy+read every time, reuse the old self.in_
            self.in_ = self.arr[..num_copy + num_read].to_vec();
        }

        // Decode internal buffer into output buffer
        let num_avail = self.in_.len() / 2;
        if dst.len() > num_avail {
            dst = &mut dst[..num_avail];
        }
        let num_dec = decode(&self.in_[..dst.len() / 2], dst)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        Ok(num_dec)
    }
}

/// Hex dumper
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dumper<W> {
    w: W,
    right_chars: [u8; 18],
    buf: [u8; 14],
    used: usize,
    n: usize,
    closed: bool,
}

impl<W> Dumper<W> {
    /// Returns a dumper that writes a hex dump of all written data to
    /// w. The format of the dump matches the output of `hexdump -C` on the command
    #[inline]
    pub const fn new(w: W) -> Self {
        Self {
            w,
            right_chars: [0; 18],
            buf: [0; 14],
            used: 0,
            n: 0,
            closed: false,
        }
    }
}

#[inline]
const fn to_char(b: u8) -> u8 {
    if b < 32 || b > 126 {
        b'.'
    } else {
        b
    }
}

#[cfg(feature = "std")]
impl<W: std::io::Write> std::io::Write for Dumper<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.closed {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "encoding/hex: dumper closed",
            ));
        }

        // Output lines look like:
        // 00000010  2e 2f 30 31 32 33 34 35  36 37 38 39 3a 3b 3c 3d  |./0123456789:;<=|
        // ^ offset                          ^ extra space              ^ ASCII of line.
        let mut n = 0;
        for b in buf {
            if self.used == 0 {
                // At the beginning of a line we print the current
                // offset in hex.
                self.buf[0] = (self.n >> 24) as u8;
                self.buf[1] = (self.n >> 16) as u8;
                self.buf[2] = (self.n >> 8) as u8;
                self.buf[3] = self.n as u8;

                let mut jj = 0;
                for idx in 0..4 {
                    self.buf[4 + jj] = HEX_TABLE[(self.buf[idx] as usize) >> 4];
                    self.buf[4 + jj + 1] = HEX_TABLE[(self.buf[idx] as usize) & 0x0f];
                    jj += 2;
                }

                self.buf[12] = b' ';
                self.buf[13] = b' ';
                self.w.write_all(&self.buf[4..])?;
            }
            let bu = (*b) as usize;
            encode(&buf[bu..bu + 1], &mut self.buf);
            self.buf[2] = b' ';
            let mut l = 3;
            if self.used == 7 {
                self.buf[3] = b' ';
                l = 4;
            } else if self.used == 15 {
                self.buf[3] = b' ';
                self.buf[4] = b'|';
                l = 5;
            }

            self.w.write_all(&self.buf[..l])?;
            n += 1;
            self.right_chars[self.used] = to_char(buf[bu]);
            self.used += 1;
            self.n += 1;
            if self.used == 16 {
                self.right_chars[16] = b' ';
                self.right_chars[17] = b'\n';
                self.w.write_all(&self.right_chars)?;
                self.used = 0;
            }
        }

        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.w.flush()
    }
}

#[cfg(feature = "std")]
impl<W: std::io::Write> crate::io::Closer for Dumper<W> {
    fn close(&mut self) -> std::io::Result<()> {
        if self.closed {
            return Ok(());
        }

        self.closed = true;
        if self.used == 0 {
            return Ok(());
        }

        self.buf[0] = b' ';
        self.buf[1] = b' ';
        self.buf[2] = b' ';
        self.buf[3] = b' ';
        self.buf[4] = b'|';
        let n_bytes = self.used;
        while self.used < 16 {
            let l = if self.used == 7 {
                4
            } else if self.used == 15 {
                5
            } else {
                3
            };
            self.w.write_all(&self.buf[..l])?;
            self.used += 1;
        }
        self.right_chars[n_bytes] = b'|';
        self.right_chars[n_bytes + 1] = b'\n';
        self.w.write_all(&self.right_chars[..n_bytes + 2])?;
        Ok(())
    }
}
