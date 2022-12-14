/// DecodeError contains the position of illegal ascii85 data at input byte
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DecodeError(usize);

impl DecodeError {
    /// Consumes self and returns position of illegal byte of the source slice.
    pub const fn into_inner(&self) -> usize {
        self.0
    }
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Illegal ascii85 data at input byte {}", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {}

/// Encodes src into at most MaxEncodedLen(len(src))
/// bytes of dst, returning the actual number of bytes written.
///
/// The encoding handles 4-byte chunks, using a special encoding
/// for the last fragment, so Encode is not appropriate for use on
/// individual blocks of a large data stream. Use NewEncoder() instead.
///
/// Often, ascii85-encoded data is wrapped in <~ and ~> symbols.
/// Encode does not add these.
#[inline]
pub fn encode(mut src: &[u8], mut dst: &mut [u8]) -> usize {
    const EMPTY: &[u8] = &[];

    if src.is_empty() {
        return 0;
    }

    let mut n = 0;
    while !src.is_empty() {
        dst[0] = 0;
        dst[1] = 0;
        dst[2] = 0;
        dst[3] = 0;
        dst[4] = 0;

        // Unpack 4 bytes into uint32 to repack into base 85 5-byte.
        let mut v = 0u32;
        match src.len() {
            1 => {
                v |= (src[0] as u32) << 24;
            }
            2 => {
                v |= (src[1] as u32) << 16;
                v |= (src[0] as u32) << 24;
            }
            3 => {
                v |= (src[2] as u32) << 8;
                v |= (src[1] as u32) << 16;
                v |= (src[0] as u32) << 24;
            }
            _ => {
                v |= src[3] as u32;
                v |= (src[2] as u32) << 8;
                v |= (src[1] as u32) << 16;
                v |= (src[0] as u32) << 24;
            }
        }

        // Special case: zero (!!!!!) shortens to z.
        if v == 0 && src.len() >= 4 {
            dst[0] = b'z';
            dst = &mut dst[1..];
            src = &src[4..];
            n += 1;
            continue;
        }

        // Otherwise, 5 base 85 digits starting at !.
        for i in (0..5).rev() {
            dst[i] = b'!' + (v % 85) as u8;
            v /= 85;
        }

        // If src was short, discard the low destination bytes.
        let mut m = 5;
        if src.len() < 4 {
            m -= 4 - src.len();
            src = EMPTY;
        } else {
            src = &src[4..];
        }

        dst = &mut dst[m..];
        n += m;
    }
    n
}

/// Returns the maximum length of an encoding of n source bytes.
#[inline]
pub const fn max_encoded_len(n: usize) -> usize {
    (n + 3) / 4 * 5
}

/// Ascii85 encoder
pub struct Encoder<W> {
    w: W,
    buf: [u8; 4],
    nbuf: usize,
    out: [u8; 1024],
}

impl<W> Encoder<W> {
    /// Returns an Ascii85 encoder
    #[inline]
    pub const fn new(w: W) -> Self {
        Self {
            w,
            buf: [0; 4],
            nbuf: 0,
            out: [0; 1024],
        }
    }
}

#[cfg(feature = "std")]
impl<W: std::io::Write> std::io::Write for Encoder<W> {
    fn write(&mut self, mut buf: &[u8]) -> std::io::Result<usize> {
        let mut n = 0;
        // Leading fringe.
        if self.nbuf > 0 {
            let mut i = 0;
            while i < buf.len() && self.nbuf < 4 {
                self.buf[self.nbuf] = buf[i];
                self.nbuf += 1;
                i += 1;
            }
            n += i;
            buf = &buf[i..];
            if self.nbuf < 4 {
                return Ok(n);
            }
            let nout = encode(&self.buf, &mut self.out);
            self.w.write_all(&self.out[..nout])?;
            self.nbuf = 0;
        }

        // Large interior chunks.
        while buf.len() >= 4 {
            let mut nn = self.out.len() / 5 * 4;
            if nn > buf.len() {
                nn = buf.len();
            }
            nn -= nn % 4;
            if nn > 0 {
                let nout = encode(&buf[..nn], &mut self.out);
                self.w.write_all(&self.out[..nout])?;
            }
            n += nn;
            buf = &buf[nn..];
        }

        crate::copy(buf, &mut self.buf);
        self.nbuf = buf.len();
        n += buf.len();
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if self.nbuf > 0 {
            let nout = encode(&self.buf[..self.nbuf], &mut self.out);
            self.nbuf = 0;
            self.w.write_all(&self.out[..nout])?;
        }
        self.w.flush()
    }
}

#[cfg(all(feature = "std", feature = "io"))]
impl<W: std::io::Write> crate::io::Closer for Encoder<W> {
    fn close(&mut self) -> std::io::Result<()> {
        use std::io::Write;
        self.flush()
    }
}

/// Decodes src into dst, returning both the number
/// of bytes written to dst and the number consumed from src.
/// If src contains invalid ascii85 data, Decode will return a [`DecodeError`].
/// Decode ignores space and control characters in src.
/// Often, ascii85-encoded data is wrapped in <~ and ~> symbols.
/// Decode expects these to have been stripped by the caller.
#[inline]
pub fn decode(src: &[u8], dst: &mut [u8]) -> Result<(usize, usize), DecodeError> {
    decode_in(src, dst, false)
}

/// Decodes src into dst, returning both the number
/// of bytes written to dst and the number consumed from src.
/// If src contains invalid ascii85 data, Decode will return the
/// a [`DecodeError`].
/// Decode ignores space and control characters in src.
/// Often, ascii85-encoded data is wrapped in <~ and ~> symbols.
/// Decode expects these to have been stripped by the caller.
///
/// Decode assumes that src represents the
/// end of the input stream and processes it completely rather
/// than wait for the completion of another 32-bit block.
#[inline]
pub fn decode_with_flush(src: &[u8], dst: &mut [u8]) -> Result<(usize, usize), DecodeError> {
    decode_in(src, dst, true)
}

#[inline]
fn decode_in(src: &[u8], dst: &mut [u8], flush: bool) -> Result<(usize, usize), DecodeError> {
    let (mut ndst, mut nsrc) = (0, 0);
    let mut v = 0u32;
    let mut nb = 0;

    for (i, b) in src.iter().enumerate() {
        let b = *b;
        if dst.len() - ndst < 4 {
            return Ok((ndst, nsrc));
        }

        match () {
            () if b <= b' ' => {
                continue;
            }
            () if b == b'z' && nb == 0 => {
                nb = 5;
                v = 0;
            }
            () if (b'!'..=b'u').contains(&b) => {
                v = v * 85 + (b - b'!') as u32;
                nb += 1;
            }
            _ => {
                return Err(DecodeError(i));
            }
        }

        if nb == 5 {
            nsrc = i + 1;
            dst[ndst] = (v >> 24) as u8;
            dst[ndst + 1] = (v >> 16) as u8;
            dst[ndst + 2] = (v >> 8) as u8;
            dst[ndst + 3] = v as u8;
            ndst += 4;
            nb = 0;
            v = 0;
        }
    }

    if flush {
        nsrc = src.len();
        if nb > 0 {
            // The number of output bytes in the last fragment
            // is the number of leftover input bytes - 1:
            // the extra byte provides enough bits to cover
            // the inefficiency of the encoding for the block.
            if nb == 1 {
                return Err(DecodeError(src.len()));
            }

            let mut i = nb;
            while i < 5 {
                // The short encoding truncated the output value.
                // We have to assume the worst case values (digit 84)
                // in order to ensure that the top bits are correct.
                v = v * 85 + 84;
                i += 1;
            }

            for _ in 0..nb - 1 {
                dst[ndst] = (v >> 24) as u8;
                v <<= 8;
                ndst += 1;
            }
        }
    }

    Ok((ndst, nsrc))
}

/// The Ascii85 decoder
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg(feature = "alloc")]
pub struct Decoder<R> {
    r: R,
    buf: [u8; 1024],
    nbuf: usize,
    out: alloc::vec::Vec<u8>,
    outbuf: [u8; 1024],
    eof: bool,
}

#[cfg(feature = "alloc")]
impl<R> Decoder<R> {
    /// Creates a new decoder that reads from the given reader.
    pub const fn new(r: R) -> Self {
        Self {
            r,
            buf: [0; 1024],
            nbuf: 0,
            out: alloc::vec::Vec::new(),
            outbuf: [0; 1024],
            eof: false,
        }
    }
}

#[cfg(feature = "std")]
impl<R: std::io::Read> std::io::Read for Decoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut n = 0;
        if buf.is_empty() {
            return Ok(n);
        }

        loop {
            // Copy leftover output from last decode.
            if !self.out.is_empty() {
                n = crate::copy(&self.out, buf);
                self.out.drain(..n);
                break;
            }

            // Decode leftover input from last read.

            if self.nbuf > 0 {
                let (ndst, nsrc) = decode_in(&self.buf[..self.nbuf], &mut self.outbuf, self.eof)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                if ndst > 0 {
                    self.out.resize(ndst, 0);
                    self.out.copy_from_slice(&self.outbuf[0..ndst]);
                    self.buf.copy_within(nsrc..self.nbuf, 0);
                    self.nbuf -= nsrc;
                    continue;
                }
                if ndst == 0 {
                    // Special case: input buffer is mostly filled with non-data bytes.
                    // Filter out such bytes to make room for more input.
                    let mut off = 0;
                    for i in 0..self.nbuf {
                        if self.buf[i] > b' ' {
                            self.buf[off] = self.buf[i];
                            off += 1;
                        }
                    }
                    self.nbuf = off;
                }
            }

            // Read more data.
            let nread = self.r.read(&mut self.buf[self.nbuf..])?;
            if nread == 0 {
                self.eof = true;
                if self.nbuf == 0 {
                    break;
                }
            }
            self.nbuf += nread;
        }
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use crate::io::Closer;

    use super::*;

    struct TestPair {
        encoded: String,
        decoded: String,
    }

    fn big_test() -> TestPair {
        TestPair {
            encoded: concat!(
                "9jqo^BlbD-BleB1DJ+*+F(f,q/0JhKF<GL>Cj@.4Gp$d7F!,L7@<6@)/0JDEF<G%<+EV:2F!,\n",
                "O<DJ+*.@<*K0@<6L(Df-\\0Ec5e;DffZ(EZee.Bl.9pF\"AGXBPCsi+DGm>@3BB/F*&OCAfu2/AKY\n",
                "i(DIb:@FD,*)+C]U=@3BN#EcYf8ATD3s@q?d$AftVqCh[NqF<G:8+EV:.+Cf>-FD5W8ARlolDIa\n",
                "l(DId<j@<?3r@:F%a+D58'ATD4$Bl@l3De:,-DJs`8ARoFb/0JMK@qB4^F!,R<AKZ&-DfTqBG%G\n",
                ">uD.RTpAKYo'+CT/5+Cei#DII?(E,9)oF*2M7/c\n"
            )
            .to_string(),
            decoded: concat!(
                "Man is distinguished, not only by his reason, but by this singular passion from ",
                "other animals, which is a lust of the mind, that by a perseverance of delight in ",
                "the continued and indefatigable generation of knowledge, exceeds the short ",
                "vehemence of any carnal pleasure."
            )
            .to_string(),
        }
    }

    fn pairs() -> Vec<TestPair> {
        vec![
            TestPair {
                encoded: String::new(),
                decoded: String::new(),
            },
            big_test(),
            TestPair {
                decoded: "\u{000}\u{000}\u{000}\u{000}".to_string(),
                encoded: "z".to_string(),
            },
        ]
    }

    fn strip85(s: &[u8]) -> Vec<u8> {
        let mut t = vec![0; s.len()];
        let mut w = 0;
        for r in s {
            let c = *r;
            if c > b' ' {
                t[w] = c;
                w += 1;
            }
        }
        t.drain(..w).collect()
    }

    #[test]
    fn test_encode() {
        for p in pairs() {
            let mut buf = vec![0; max_encoded_len(p.decoded.len())];
            let n = encode(p.decoded.as_bytes(), &mut buf);
            buf = buf[0..n].to_vec();
            assert_eq!(strip85(&buf), strip85(p.encoded.as_bytes()));
        }
    }

    #[test]
    fn test_encoder() {
        for p in pairs() {
            let mut bb = vec![];
            let mut encoder = Encoder::new(&mut bb);
            encoder.write_all(p.decoded.as_bytes()).unwrap();
            encoder.close().unwrap();
            assert_eq!(strip85(&bb), strip85(p.encoded.as_bytes()));
        }
    }

    #[test]
    fn test_encoder_buffering() {
        let input = big_test().decoded.as_bytes().to_vec();
        for bs in 1..=12 {
            let mut bb = vec![];
            let mut encoder = Encoder::new(&mut bb);
            let mut pos = 0;
            while pos < input.len() {
                let mut end = pos + bs;
                if end > input.len() {
                    end = input.len();
                }
                let n = encoder.write(&input[pos..end]).unwrap();
                assert_eq!(n, end - pos);

                pos += bs;
            }
            encoder.close().unwrap();
            assert_eq!(strip85(&bb), strip85(big_test().encoded.as_bytes()));
        }
    }

    #[test]
    fn test_decode() {
        for p in pairs() {
            let mut dbuf = vec![0; 4 * p.encoded.len()];
            let (ndst, nsrc) = decode_with_flush(p.encoded.as_bytes(), &mut dbuf).unwrap();
            assert_eq!(nsrc, p.encoded.len());
            assert_eq!(ndst, p.decoded.len());
            assert_eq!(&dbuf[0..ndst], p.decoded.as_bytes());
        }
    }

    #[test]
    fn test_decoder() {
        for p in pairs() {
            eprintln!("{}", p.decoded);
            let mut decoder = Decoder::new(std::io::Cursor::new(p.encoded.as_bytes()));

            let mut dbuf = String::new();
            let _n = decoder.read_to_string(&mut dbuf).unwrap();
            assert_eq!(dbuf.len(), p.decoded.len());
            assert_eq!(dbuf, p.decoded);
        }
    }

    #[test]
    fn test_decoder_buffering() {
        let big_test = big_test();
        for bs in 1..=12 {
            let mut decoder = Decoder::new(big_test.encoded.as_bytes());
            let mut buf = vec![0; big_test.decoded.len() + 12];
            let mut total = 0;
            while total < big_test.decoded.len() {
                let n = decoder.read(&mut buf[total..total + bs]).unwrap();
                total += n;
            }
            assert_eq!(&buf[..total], big_test.decoded.as_bytes());
        }
    }

    #[test]
    fn test_decode_corrupt() {
        struct Corrupt {
            e: String,
            p: usize,
        }

        let examples = vec![
            Corrupt {
                e: "v".to_string(),
                p: 0,
            },
            Corrupt {
                e: "!z!!!!!!!!!".to_string(),
                p: 1,
            },
        ];

        for e in examples {
            let mut dbuf = vec![0; 4 * e.e.len()];
            let p = decode_with_flush(e.e.as_bytes(), &mut dbuf)
                .unwrap_err()
                .into_inner();
            assert_eq!(p, e.p);
        }
    }

    #[test]
    fn test_big() {
        let n = 3 * 1000 + 1;
        let mut raw = vec![0; n];
        const ALPHA: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        for i in 0..n {
            raw[i] = ALPHA[i % ALPHA.len()];
        }

        let mut encoded = vec![];
        let mut encoder = Encoder::new(&mut encoded);
        let nn = encoder.write(&raw).unwrap();
        assert_eq!(nn, n);
        encoder.close().unwrap();
        let mut decoder = Decoder::new(std::io::Cursor::new(&mut encoded));
        let mut decoded = String::new();
        let _n = decoder.read_to_string(&mut decoded).unwrap();
        assert_eq!(decoded.as_bytes(), raw);
    }

    #[test]
    fn test_decoder_internal_whitespace() {
        let mut s = vec![b' '; 2048];
        s.push(b'z');
        let mut decoder = Decoder::new(std::io::Cursor::new(&mut s));
        let mut decoded = String::new();
        decoder.read_to_string(&mut decoded).unwrap();
        assert_eq!(decoded, "\u{000}\u{000}\u{000}\u{000}");
    }
}
