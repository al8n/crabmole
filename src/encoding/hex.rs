const HEX_TABLE: [u8; 16] = *b"0123456789abcdef";

const REVERSE_HASH_TABLE: [u8; 256] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255, 255, 255,
    255, 255, 255, 255, 10, 11, 12, 13, 14, 15, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 10, 11, 12, 13,
    14, 15, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255,
];

/// Hex related errors
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    /// Reports an attempt to decode an odd-length input
    /// using Decode or DecodeString.
    Length,
    /// Values describe errors resulting from an invalid byte in a hex string.
    InvalidByte(u8),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::Length => write!(f, "encoding/hex: odd length hex string"),
            Error::InvalidByte(byte) => write!(f, "encoding/hex: invalid byte {}", byte),
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
pub const fn decoded_len(n: usize) -> usize {
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
            return Err(Error::InvalidByte(p));
        }

        if b > 0x0f {
            return Err(Error::InvalidByte(q));
        }

        dst[i] = (a << 4) | b;
        i += 1;
        j += 2;
    }

    if src.len() % 2 == 1 {
        // Check for invalid char before reporting bad length,
        // since the invalid char (if present) is an earlier problem.
        if REVERSE_HASH_TABLE[src[j - 1] as usize] > 0x0f {
            return Err(Error::InvalidByte(src[j - 1]));
        }
        return Err(Error::Length);
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
    let mut dst = alloc::vec![0; decoded_len(src.len())];
    decode(src, &mut dst).map(|u| {
        dst.truncate(u);
        dst
    })
}

/// Returns a [`Vec<u8>`] that contains a hex dump of the given data. The format
/// of the hex dump matches the output of `hexdump -C` on the command line.
#[cfg(all(feature = "std", feature = "io"))]
#[inline]
pub fn dump(src: &[u8]) -> std::io::Result<alloc::vec::Vec<u8>> {
    use std::io::Write;

    if src.is_empty() {
        return Ok(alloc::vec![]);
    }

    // Dumper will write 79 bytes per complete 16 byte chunk, and at least
    // 64 bytes for whatever remains. Round the allocation up, since only a
    // maximum of 15 bytes will be wasted.
    let mut dst = alloc::vec::Vec::with_capacity((1 + ((src.len() - 1) / 16)) * 79);

    let mut dumper = Dumper::new(&mut dst);
    dumper
        .write_all(src)
        .and_then(|_| dumper.close_in())
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
            self.in_.resize(num_copy + num_read, 0);
            self.in_.copy_from_slice(&self.arr[..num_copy + num_read]);
            if num_read == 0 && self.in_.len() % 2 != 0 {
                let a = REVERSE_HASH_TABLE[self.in_[self.in_.len() - 1] as usize];
                if a > 0x0f {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        Error::InvalidByte(self.in_[self.in_.len() - 1]),
                    ));
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "encoding/hex: unexpected end of input",
                    ));
                }
            }
        }

        // Decode internal buffer into output buffer
        let num_avail = self.in_.len() / 2;
        if dst.len() > num_avail {
            dst = &mut dst[..num_avail];
        }

        let num_dec = decode(&self.in_[..dst.len() * 2], dst).map_err(|e| match e {
            Error::InvalidByte(byte) => {
                std::io::Error::new(std::io::ErrorKind::InvalidData, Error::InvalidByte(byte))
            }
            Error::Length => std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "encoding/hex: unexpected end of input",
            ),
        })?;

        self.in_.drain(..num_dec * 2);
        if self.in_.len() < 2 {
            if self.in_.len() == 1 && num_dec == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "encoding/hex: unexpected end of input",
                ));
            } else {
                return Ok(num_dec);
            }
        }

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

#[cfg(feature = "std")]
impl<W: std::io::Write> Dumper<W> {
    #[inline]
    fn close_in(&mut self) -> std::io::Result<()> {
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
        for i in 0..buf.len() {
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
            encode(&buf[i..i + 1], &mut self.buf);
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
            self.right_chars[self.used] = to_char(buf[i]);
            self.used += 1;
            self.n += 1;
            if self.used == 16 {
                self.right_chars[16] = b'|';
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

#[cfg(all(feature = "std", feature = "io"))]
impl<W: std::io::Write> crate::io::Closer for Dumper<W> {
    fn close(&mut self) -> std::io::Result<()> {
        self.close_in()
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use crate::io::Closer;

    use super::*;

    struct TestPair {
        enc: String,
        dec: Vec<u8>,
    }

    fn pairs() -> Vec<TestPair> {
        vec![
            TestPair {
                enc: String::new(),
                dec: Vec::new(),
            },
            TestPair {
                enc: "0001020304050607".to_string(),
                dec: vec![0, 1, 2, 3, 4, 5, 6, 7],
            },
            TestPair {
                enc: "08090a0b0c0d0e0f".to_string(),
                dec: vec![8, 9, 10, 11, 12, 13, 14, 15],
            },
            TestPair {
                enc: "f0f1f2f3f4f5f6f7".to_string(),
                dec: vec![0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7],
            },
            TestPair {
                enc: "f8f9fafbfcfdfeff".to_string(),
                dec: vec![0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff],
            },
            TestPair {
                enc: String::from("67"),
                dec: vec![b'g'],
            },
            TestPair {
                enc: String::from("e3a1"),
                dec: vec![0xe3, 0xa1],
            },
        ]
    }

    #[test]
    fn test_encode() {
        for p in pairs() {
            let mut dst = vec![0; encoded_len(p.dec.len())];
            let n = encode(&p.dec, &mut dst);
            assert_eq!(n, dst.len());
            assert_eq!(p.enc, String::from_utf8(dst).unwrap());
        }
    }

    #[test]
    fn test_decode() {
        // Case for decoding uppercase hex characters, since
        // Encode always uses lowercase.
        let mut pairs = pairs();
        pairs.push(TestPair {
            enc: String::from("F8F9FAFBFCFDFEFF"),
            dec: vec![0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff],
        });

        for p in pairs {
            let mut dst = vec![0; decoded_len(p.enc.len())];
            let n = decode(p.enc.as_bytes(), &mut dst).unwrap();
            assert_eq!(n, dst.len());
            assert_eq!(p.dec, dst);
        }
    }

    #[test]
    fn test_encode_to_vec() {
        for p in pairs() {
            let dst = encode_to_vec(&p.dec);
            assert_eq!(p.enc, String::from_utf8(dst).unwrap());
        }
    }

    #[test]
    fn test_decode_to_vec() {
        for p in pairs() {
            let dst = decode_to_vec(p.enc.as_bytes()).unwrap();
            assert_eq!(p.dec, dst);
        }
    }

    struct ErrorTestPair {
        in_: String,
        out: Vec<u8>,
        err: Option<Error>,
    }

    fn err_pairs() -> Vec<ErrorTestPair> {
        vec![
            ErrorTestPair {
                in_: String::from(""),
                out: vec![],
                err: None,
            },
            ErrorTestPair {
                in_: String::from("0"),
                out: vec![],
                err: Some(Error::Length),
            },
            ErrorTestPair {
                in_: String::from("zd4aa"),
                out: vec![],
                err: Some(Error::InvalidByte(b'z')),
            },
            ErrorTestPair {
                in_: String::from("d4aaz"),
                out: vec![212, 170],
                err: Some(Error::InvalidByte(b'z')),
            },
            ErrorTestPair {
                in_: String::from("30313"),
                out: String::from("01").into_bytes(),
                err: Some(Error::Length),
            },
            ErrorTestPair {
                in_: String::from("0g"),
                out: vec![],
                err: Some(Error::InvalidByte(b'g')),
            },
            ErrorTestPair {
                in_: String::from("00gg"),
                out: String::from("\x00").into_bytes(),
                err: Some(Error::InvalidByte(b'g')),
            },
            ErrorTestPair {
                in_: String::from("0\x01"),
                out: vec![],
                err: Some(Error::InvalidByte(b'\x01')),
            },
            ErrorTestPair {
                in_: String::from("ffeed"),
                out: vec![255, 238],
                err: Some(Error::Length),
            },
        ]
    }

    #[test]
    fn test_decode_err() {
        for p in err_pairs() {
            let mut dst = vec![0; p.in_.len() + 10];
            if p.err.is_some() {
                let err = decode(p.in_.as_bytes(), &mut dst).unwrap_err();
                assert_eq!(err, p.err.unwrap());
            } else {
                let n = decode(p.in_.as_bytes(), &mut dst).unwrap();
                assert_eq!(&dst[..n], &p.out);
            }
        }
    }

    // TODO: implement this after implement `io.CopyBuffer`
    #[test]
    fn test_encoder_decoder() {
        // for multiplier in [1, 128, 192] {
        //     for p in pairs() {
        //         let mut input = vec![p.dec; multiplier];
        //         let mut output = vec![p.enc; multiplier];

        //         let mut buf = vec![];
        //         let mut encoder = Encoder::new(&mut buf);

        //     }
        // }
    }

    #[test]
    fn test_decoder_error() {
        for p in err_pairs() {
            let mut r = std::io::Cursor::new(p.in_.as_bytes());
            let mut dec = Decoder::new(&mut r);
            let mut out = String::new();
            if p.err.is_some() {
                let err = dec.read_to_string(&mut out).unwrap_err();
                // assert_eq!(err, p.err.unwrap());
                let p_err = p.err.unwrap();
                match p_err {
                    Error::InvalidByte(byte) => {
                        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
                        let x = *err.into_inner().unwrap().downcast_ref::<Error>().unwrap();
                        assert_eq!(x, Error::InvalidByte(byte));
                    }
                    Error::Length => {
                        assert_eq!(err.kind(), std::io::ErrorKind::UnexpectedEof);
                    }
                }
            } else {
                dec.read_to_string(&mut out).unwrap();
                assert_eq!(out.as_bytes(), p.out);
            }
        }
    }

    const EXPECTED_HEX_DUMP: &[u8] = &[
        48, 48, 48, 48, 48, 48, 48, 48, 32, 32, 49, 101, 32, 49, 102, 32, 50, 48, 32, 50, 49, 32,
        50, 50, 32, 50, 51, 32, 50, 52, 32, 50, 53, 32, 32, 50, 54, 32, 50, 55, 32, 50, 56, 32, 50,
        57, 32, 50, 97, 32, 50, 98, 32, 50, 99, 32, 50, 100, 32, 32, 124, 46, 46, 32, 33, 34, 35,
        36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 124, 10, 48, 48, 48, 48, 48, 48, 49, 48, 32, 32,
        50, 101, 32, 50, 102, 32, 51, 48, 32, 51, 49, 32, 51, 50, 32, 51, 51, 32, 51, 52, 32, 51,
        53, 32, 32, 51, 54, 32, 51, 55, 32, 51, 56, 32, 51, 57, 32, 51, 97, 32, 51, 98, 32, 51, 99,
        32, 51, 100, 32, 32, 124, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
        124, 10, 48, 48, 48, 48, 48, 48, 50, 48, 32, 32, 51, 101, 32, 51, 102, 32, 52, 48, 32, 52,
        49, 32, 52, 50, 32, 52, 51, 32, 52, 52, 32, 52, 53, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 124, 62, 63, 64, 65,
        66, 67, 68, 69, 124, 10,
    ];

    #[test]
    fn test_dumper() {
        let mut in_ = [0; 40];
        for (i, b) in in_.iter_mut().enumerate() {
            *b = (i + 30) as u8;
        }

        for stride in 1..in_.len() {
            let mut out = vec![];
            let mut dumper = Dumper::new(&mut out);
            let mut done = 0;
            while done < in_.len() {
                let mut todo = done + stride;
                if todo > in_.len() {
                    todo = in_.len();
                }
                dumper.write_all(&in_[done..todo]).unwrap();
                done = todo;
            }

            dumper.close().unwrap();
            assert_eq!(out, EXPECTED_HEX_DUMP);
        }
    }

    #[test]
    fn test_dumper_double_close() {
        let mut out = Vec::new();
        let mut dumper = Dumper::new(&mut out);

        dumper.write_all(b"rustacean").unwrap();
        dumper.close().unwrap();
        dumper.close().unwrap();
        dumper.write_all(b"rustacean").unwrap_err();
        dumper.close().unwrap();

        const EXPECTED: &[u8] =
            b"00000000  72 75 73 74 61 63 65 61  6e                       |rustacean|\n";
        assert_eq!(out, EXPECTED);
    }

    #[test]
    fn test_dumper_early_close() {
        let mut out = vec![];
        let mut dumper = Dumper::new(&mut out);
        dumper.close().unwrap();
        dumper.write_all(b"rustacean").unwrap_err();

        assert_eq!(out, &[]);
    }

    #[test]
    fn test_dump() {
        let mut in_ = [0; 40];
        for (i, b) in in_.iter_mut().enumerate() {
            *b = (i + 30) as u8;
        }

        let out = dump(&in_).unwrap();
        assert_eq!(out, EXPECTED_HEX_DUMP);
    }
}
