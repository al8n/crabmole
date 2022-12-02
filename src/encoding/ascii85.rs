/// CorruptInputError contains the position of illegal ascii85 data at input byte
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CorruptInputError(usize);

impl core::fmt::Display for CorruptInputError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Illegal ascii85 data at input byte {}", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CorruptInputError {}

/// Encodes src into at most MaxEncodedLen(len(src))
/// bytes of dst, returning the actual number of bytes written.
///
/// The encoding handles 4-byte chunks, using a special encoding
/// for the last fragment, so Encode is not appropriate for use on
/// individual blocks of a large data stream. Use NewEncoder() instead.
///
/// Often, ascii85-encoded data is wrapped in <~ and ~> symbols.
/// Encode does not add these.
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
                v |= src[3] as u32;
                v |= (src[2] as u32) << 8;
                v |= (src[1] as u32) << 16;
                v |= (src[0] as u32) << 24;
            }
            2 => {
                v |= src[3] as u32;
                v |= (src[2] as u32) << 8;
                v |= (src[1] as u32) << 16;
            }
            3 => {
                v |= src[3] as u32;
                v |= (src[2] as u32) << 8;
            }
            _ => {
                v |= src[3] as u32;
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

#[cfg(feature = "std")]
impl<W: std::io::Write> crate::io::Closer for Encoder<W> {
    fn close(&mut self) -> std::io::Result<()> {
        use std::io::Write;
        self.flush()
    }
}

/// Decodes src into dst, returning both the number
/// of bytes written to dst and the number consumed from src.
/// If src contains invalid ascii85 data, Decode will return a [`CorruptInputError`].
/// Decode ignores space and control characters in src.
/// Often, ascii85-encoded data is wrapped in <~ and ~> symbols.
/// Decode expects these to have been stripped by the caller.
pub fn decode(src: &[u8], dst: &mut [u8]) -> Result<(usize, usize), CorruptInputError> {
    decode_in(src, dst, false)
}

/// Decodes src into dst, returning both the number
/// of bytes written to dst and the number consumed from src.
/// If src contains invalid ascii85 data, Decode will return the
/// a [`CorruptInputError`].
/// Decode ignores space and control characters in src.
/// Often, ascii85-encoded data is wrapped in <~ and ~> symbols.
/// Decode expects these to have been stripped by the caller.
///
/// Decode assumes that src represents the
/// end of the input stream and processes it completely rather
/// than wait for the completion of another 32-bit block.
pub fn decode_with_flush(src: &[u8], dst: &mut [u8]) -> Result<(usize, usize), CorruptInputError> {
    decode_in(src, dst, true)
}

#[inline]
fn decode_in(src: &[u8], dst: &mut [u8], flush: bool) -> Result<(usize, usize), CorruptInputError> {
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
                return Err(CorruptInputError(i));
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
                return Err(CorruptInputError(src.len()));
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
                let (ndst, nsrc) = decode(&self.buf[..self.nbuf], &mut self.outbuf)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                if ndst > 0 {
                    self.out.drain(ndst..);
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
            self.nbuf += self.r.read(&mut self.buf[self.nbuf..])?;
        }
        Ok(n)
    }
}
