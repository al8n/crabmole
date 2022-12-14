/// Error
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    /// Invalid encoder
    InvalidEncoder,

    /// Invalid padding character
    InvalidPadding,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::InvalidEncoder => write!(f, "Base64 alphabet must be 32 bytes long"),
            Error::InvalidPadding => write!(f, "Invalid padding character"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Decode error
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DecodeError(usize);

impl DecodeError {
    /// leak the inner input byte
    #[inline]
    pub const fn into_inner(self) -> usize {
        self.0
    }
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "illegal base64 data at input byte {}", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {}

/// `BASE = 64`
pub const BASE: usize = 64;

const ENCODE_STD: [u8; BASE] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const ENCODE_URL: [u8; BASE] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// No padding
pub const NO_PADDING: Option<char> = None;

/// Standard padding
pub const STD_PADDING: Option<char> = Some('=');

/// The standard base64 encoding, as defined in
/// RFC 4648.
pub const STD_ENCODING: Base64 = Base64::new_unchecked(ENCODE_STD);

/// The alternate base64 encoding defined in RFC 4648.
/// It is typically used in URLs and file names.
pub const URL_ENCODING: Base64 = Base64::new_unchecked(ENCODE_URL);

/// The standard raw, unpadded base64 encoding,
/// as defined in RFC 4648 section 3.2.
/// This is the same as [`STD_ENCODING`] but omits padding characters.
pub const RAW_STD_ENCODING: Base64 = Base64::new_unchecked(ENCODE_STD).with_padding_unchecked(None);

/// The unpadded alternate base64 encoding defined in RFC 4648.
/// It is typically used in URLs and file names.
/// This is the same as [`URL_ENCODING`] but omits padding characters.
pub const RAW_URL_ENCODING: Base64 = Base64::new_unchecked(ENCODE_URL).with_padding_unchecked(None);

const DECODE_MAP_INITIALIZE: [u8; 256] = [255; 256];

/// An Base64 is a radix 64 encoding/decoding scheme, defined by a
/// 64-character alphabet. The most common encoding is the "base64"
/// encoding defined in RFC 4648 and used in MIME (RFC 2045) and PEM
/// (RFC 1421).  RFC 4648 also defines an alternate encoding, which is
/// the standard encoding with - and _ substituted for + and /.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Base64 {
    encode: [u8; BASE],
    decode_map: [u8; 256],
    pad_char: Option<char>,
    strict: bool,
}

impl Default for Base64 {
    fn default() -> Self {
        STD_ENCODING
    }
}

impl Base64 {
    /// Returns a new padded Base64 defined by the given alphabet,
    /// which must be a 64-byte array that does not contain the padding character
    /// or CR / LF ('\r', '\n').
    /// The resulting Base64 uses the default padding character ('='),
    /// which may be changed or disabled via [`Base64::with_padding`].
    #[inline]
    pub const fn new(encoder: [u8; BASE]) -> Result<Self, Error> {
        const CH: char = '=';
        let mut decode_map = DECODE_MAP_INITIALIZE;
        let mut idx = 0;
        while idx < BASE {
            if encoder[idx] == b'\n' || encoder[idx] == b'\r' || encoder[idx] == CH as u8 {
                return Err(Error::InvalidEncoder);
            }
            decode_map[encoder[idx] as usize] = idx as u8;
            idx += 1;
        }

        Ok(Self {
            encode: encoder,
            decode_map,
            pad_char: Some('='),
            strict: false,
        })
    }

    /// Returns a new padded Base64 defined by the given alphabet,
    /// which must be a 64-byte array that does not contain the padding character
    /// or CR / LF ('\r', '\n').
    /// The resulting Base64 uses the default padding character ('='),
    /// which may be changed or disabled via [`Base64::with_padding_unchecked`].
    ///
    /// # Panic
    /// 64-byte array that contains the padding character ('=')
    /// or CR / LF ('\r', '\n').
    #[inline]
    pub const fn new_unchecked(encoder: [u8; BASE]) -> Self {
        const CH: char = '=';
        let mut decode_map = DECODE_MAP_INITIALIZE;
        let mut idx = 0;
        while idx < BASE {
            if encoder[idx] == b'\n' || encoder[idx] == b'\r' || encoder[idx] == CH as u8 {
                panic!("encoding alphabet contains newline character or padding character");
            }
            decode_map[encoder[idx] as usize] = idx as u8;
            idx += 1;
        }

        Self {
            encode: encoder,
            decode_map,
            pad_char: Some('='),
            strict: false,
        }
    }

    /// Creates a new encoding identical to enc except
    /// with a specified padding character, or NoPadding to disable padding.
    /// The padding character must not be '\r' or '\n', must not
    /// be contained in the encoding's alphabet and must be a rune equal or
    /// below '\xff'.
    #[inline]
    pub const fn with_padding(self, pad: Option<char>) -> Result<Self, Error> {
        let Self {
            encode: encoder,
            mut decode_map,
            pad_char: _,
            strict,
        } = self;

        match pad {
            Some(ch) => {
                let mut idx = 0;
                while idx < BASE {
                    if encoder[idx] == b'\n' || encoder[idx] == b'\r' || encoder[idx] == ch as u8 {
                        return Err(Error::InvalidPadding);
                    }
                    decode_map[encoder[idx] as usize] = idx as u8;
                    idx += 1;
                }
            }
            None => {
                let mut idx = 0;
                while idx < BASE {
                    if encoder[idx] == b'\n' || encoder[idx] == b'\r' {
                        return Err(Error::InvalidPadding);
                    }
                    decode_map[encoder[idx] as usize] = idx as u8;
                    idx += 1;
                }
            }
        }

        Ok(Self {
            encode: encoder,
            decode_map,
            pad_char: pad,
            strict,
        })
    }

    /// Creates a new encoding identical to enc except
    /// with a specified padding character, or [`NO_PADDING`] to disable padding.
    /// The padding character must not be '\r' or '\n', must not
    /// be contained in the encoding's alphabet and must be a rune equal or
    /// below '\xff'.
    ///
    /// # Panic
    /// 64-byte array that contains the padding character
    /// or CR / LF ('\r', '\n').
    #[inline]
    pub const fn with_padding_unchecked(self, pad: Option<char>) -> Self {
        let Self {
            encode: encoder,
            mut decode_map,
            pad_char: _,
            strict,
        } = self;

        match pad {
            Some(ch) => {
                let mut idx = 0;
                while idx < BASE {
                    if encoder[idx] == b'\n' || encoder[idx] == b'\r' || encoder[idx] == ch as u8 {
                        panic!("encoding alphabet contains newline character or padding character");
                    }
                    decode_map[encoder[idx] as usize] = idx as u8;
                    idx += 1;
                }
            }
            None => {
                let mut idx = 0;
                while idx < BASE {
                    if encoder[idx] == b'\n' || encoder[idx] == b'\r' {
                        panic!("encoding alphabet contains newline character or padding character");
                    }
                    decode_map[encoder[idx] as usize] = idx as u8;
                    idx += 1;
                }
            }
        }

        Self {
            encode: encoder,
            decode_map,
            pad_char: pad,
            strict,
        }
    }

    /// Creates a new encoding identical to enc except with
    /// strict decoding enabled. In this mode, the decoder requires that
    /// trailing padding bits are zero, as described in RFC 4648 section 3.5.
    ///
    /// Note that the input is still malleable, as new line characters
    /// (CR and LF) are still ignored.
    #[inline]
    pub const fn with_strict(mut self) -> Self {
        self.strict = true;
        self
    }

    /// Returns the length in bytes of the base64 encoding
    /// of an input buffer of length n.
    #[inline]
    pub const fn encoded_len(&self, n: usize) -> usize {
        if self.pad_char.is_none() {
            return (n * 8 + 5) / 6;
        }
        (n + 2) / 3 * 4
    }

    /// Returns a base64 encoder.
    #[inline]
    pub const fn encoder<W: std::io::Write>(self, w: W) -> Encoder<W> {
        Encoder::new(self, w)
    }

    /// Encodes src using the encoding enc, writing
    /// EncodedLen(len(src)) bytes to dst.
    ///
    /// The encoding pads the output to a multiple of 4 bytes,
    /// so Encode is not appropriate for use on individual blocks
    /// of a large data stream. Use NewEncoder() instead.
    pub fn encode(&self, src: &[u8], dst: &mut [u8]) {
        if src.is_empty() {
            return;
        }

        let (mut di, mut si) = (0, 0);
        let n = (src.len() / 3) * 3;
        while si < n {
            // Convert 3x 8bit source bytes into 4 bytes
            let val =
                ((src[si] as usize) << 16) | ((src[si + 1] as usize) << 8) | (src[si + 2] as usize);

            dst[di] = self.encode[(val >> 18) & 0x3f];
            dst[di + 1] = self.encode[(val >> 12) & 0x3f];
            dst[di + 2] = self.encode[(val >> 6) & 0x3f];
            dst[di + 3] = self.encode[val & 0x3f];

            si += 3;
            di += 4;
        }

        let remain = src.len() - si;
        if remain == 0 {
            return;
        }

        // Add the remaining small block
        let mut val = (src[si] as usize) << 16;
        if remain == 2 {
            val |= (src[si + 1] as usize) << 8;
        }

        dst[di] = self.encode[(val >> 18) & 0x3f];
        dst[di + 1] = self.encode[(val >> 12) & 0x3f];

        match remain {
            2 => {
                dst[di + 2] = self.encode[(val >> 6) & 0x3f];
                if let Some(ch) = self.pad_char {
                    dst[di + 3] = ch as u8;
                }
            }
            1 => {
                if let Some(ch) = self.pad_char {
                    dst[di + 2] = ch as u8;
                    dst[di + 3] = ch as u8;
                }
            }
            _ => {}
        }
    }

    /// Returns the base64 encoding of src.
    #[cfg(feature = "alloc")]
    pub fn encode_to_vec(&self, src: &[u8]) -> alloc::vec::Vec<u8> {
        let mut buf = alloc::vec![0; self.encoded_len(src.len())];
        self.encode(src, &mut buf);
        buf
    }

    /// Returns the base64 decoder
    #[cfg(feature = "std")]
    pub const fn decoder<R: std::io::Read>(self, r: R) -> Decoder<R> {
        Decoder::new(self, r)
    }

    /// Decodes src using the encoding enc. It writes at most
    /// `self.decoded_len(src.len())` bytes to dst and returns the number of bytes
    /// written. If src contains invalid base64 data, it will return the
    /// number of bytes successfully written and DecodeError.
    /// New line characters (\r and \n) are ignored.
    pub fn decode(&self, src: &[u8], dst: &mut [u8]) -> Result<usize, DecodeError> {
        if src.is_empty() {
            return Ok(0);
        }

        let mut n = 0;
        let mut si = 0;
        while usize::BITS >= 64 && src.len() - si >= 8 && dst.len() - n >= 8 {
            let src2 = &src[si..si + 8];
            let (dn, ok) = assemble_64(
                self.decode_map[src2[0] as usize],
                self.decode_map[src2[1] as usize],
                self.decode_map[src2[2] as usize],
                self.decode_map[src2[3] as usize],
                self.decode_map[src2[4] as usize],
                self.decode_map[src2[5] as usize],
                self.decode_map[src2[6] as usize],
                self.decode_map[src2[7] as usize],
            );

            if ok {
                dst[n..n + core::mem::size_of::<u64>()].copy_from_slice(&dn.to_be_bytes());
                n += 6;
                si += 8;
            } else {
                let (si1, ninc) = self.decode_quantum(src, &mut dst[n..], si)?;
                si = si1;
                n += ninc;
            }
        }

        while src.len() - si >= 4 && dst.len() - n >= 4 {
            let src2 = &src[si..si + 4];
            let (dn, ok) = assemble_32(
                self.decode_map[src2[0] as usize],
                self.decode_map[src2[1] as usize],
                self.decode_map[src2[2] as usize],
                self.decode_map[src2[3] as usize],
            );
            if ok {
                dst[n..n + core::mem::size_of::<u32>()].copy_from_slice(&dn.to_be_bytes());
                n += 3;
                si += 4;
            } else {
                let (si1, ninc) = self.decode_quantum(src, &mut dst[n..], si)?;
                si = si1;
                n += ninc;
            }
        }

        while si < src.len() {
            let (si1, ninc) = self.decode_quantum(src, &mut dst[n..], si)?;
            si = si1;
            n += ninc;
        }
        Ok(n)
    }

    /// Returns the bytes represented by the base64 vec s.
    #[cfg(feature = "alloc")]
    pub fn decode_to_vec(&self, src: &[u8]) -> Result<alloc::vec::Vec<u8>, DecodeError> {
        let mut buf = alloc::vec![0; self.decoded_len(src.len())];
        let n = self.decode(src, &mut buf)?;
        buf.truncate(n);
        Ok(buf)
    }

    /// Decodes up to 4 base64 bytes. The received parameters are
    /// the destination buffer dst, the source buffer src and an index in the
    /// source buffer si.
    /// It returns the number of bytes read from src, the number of bytes written
    /// to dst, and an error, if any.
    #[inline]
    fn decode_quantum(
        self,
        src: &[u8],
        dst: &mut [u8],
        mut si: usize,
    ) -> Result<(usize, usize), DecodeError> {
        let mut dbuf = [0; 4];
        let mut dlen = 4;
        let mut j = 0;
        while j < dbuf.len() {
            if src.len() == si {
                match () {
                    () if j == 0 => {
                        return Ok((si, 0));
                    }
                    () if j == 1 || self.pad_char.is_some() => {
                        return Err(DecodeError(si - j));
                    }
                    _ => {}
                }
                dlen = j;
                break;
            }
            let in_ = src[si];
            si += 1;

            let out = self.decode_map[in_ as usize];
            if out != 0xff {
                dbuf[j] = out;
                j += 1;
                continue;
            }

            if in_ == b'\n' || in_ == b'\r' {
                continue;
            }

            if let Some(ch) = self.pad_char {
                if (in_ as char) != ch {
                    return Err(DecodeError(si - 1));
                }
            }
            // We've reached the end and there's padding
            match j {
                0 | 1 => {
                    // incorrect padding
                    return Err(DecodeError(si - 1));
                }
                2 => {
                    // "==" is expected, the first "=" is already consumed.
                    // skip over newlines
                    while si < src.len() && (src[si] == b'\n' || src[si] == b'\r') {
                        si += 1;
                    }
                    if si == src.len() {
                        // not enough padding
                        return Err(DecodeError(src.len()));
                    }
                    if let Some(ch) = self.pad_char {
                        if (src[si] as char) != ch {
                            return Err(DecodeError(si - 1));
                        }
                    }
                    si += 1;
                }
                _ => {}
            }
            // skip over newlines
            while si < src.len() && (src[si] == b'\n' || src[si] == b'\r') {
                si += 1;
            }
            if si < src.len() {
                // trailing garbage
                return Err(DecodeError(si));
            }
            dlen = j;
            break;
        }

        // Convert 4x 6bit source bytes into 3 bytes
        let val = ((dbuf[0] as usize) << 18)
            | ((dbuf[1] as usize) << 12)
            | ((dbuf[2] as usize) << 6)
            | (dbuf[3] as usize);
        dbuf[2] = val as u8;
        dbuf[1] = (val >> 8) as u8;
        dbuf[0] = (val >> 16) as u8;

        match dlen {
            4 => {
                dst[2] = dbuf[2];
                dbuf[2] = 0;
                dst[1] = dbuf[1];
                if self.strict && dbuf[2] != 0 {
                    return Err(DecodeError(si - 1));
                }
                dbuf[1] = 0;
                dst[0] = dbuf[0];
                if self.strict && (dbuf[1] != 0 || dbuf[2] != 0) {
                    return Err(DecodeError(si - 2));
                }
            }
            3 => {
                dst[1] = dbuf[1];
                if self.strict && dbuf[2] != 0 {
                    return Err(DecodeError(si - 1));
                }
                dbuf[1] = 0;
                dst[0] = dbuf[0];
                if self.strict && (dbuf[1] != 0 || dbuf[2] != 0) {
                    return Err(DecodeError(si - 2));
                }
            }
            2 => {
                dst[0] = dbuf[0];
                if self.strict && (dbuf[1] != 0 || dbuf[2] != 0) {
                    return Err(DecodeError(si - 2));
                }
            }
            _ => {}
        }
        Ok((si, dlen - 1))
    }

    /// Returns the maximum length in bytes of the decoded data
    /// corresponding to n bytes of base64-encoded
    #[inline]
    pub const fn decoded_len(&self, n: usize) -> usize {
        if self.pad_char.is_none() {
            return n * 6 / 8;
        }
        n / 4 * 3
    }
}

/// Base64 encoder
pub struct Encoder<W> {
    enc: Base64,
    w: W,
    buf: [u8; 3],
    nbuf: usize,
    out: [u8; 1024],
}

impl<W> Encoder<W> {
    /// Returns a new encoder based on the given encoding
    #[inline]
    pub const fn new(enc: Base64, w: W) -> Self {
        Self {
            enc,
            w,
            buf: [0; 3],
            nbuf: 0,
            out: [0; 1024],
        }
    }
}

#[cfg(feature = "std")]
impl<W: std::io::Write> std::io::Write for Encoder<W> {
    #[inline]
    fn write(&mut self, mut buf: &[u8]) -> std::io::Result<usize> {
        let mut n = 0;
        // Leading fringe.
        if self.nbuf > 0 {
            let mut i = 0;
            while i < buf.len() && self.nbuf < 3 {
                self.buf[self.nbuf] = buf[i];
                self.nbuf += 1;
                i += 1;
            }
            n += i;
            buf = &buf[i..];
            if self.nbuf < 3 {
                return Ok(n);
            }

            self.enc.encode(&self.buf, &mut self.out);
            self.w.write_all(&self.out[..4])?;
            self.nbuf = 0;
        }

        // Large interior chunks.
        while buf.len() >= 3 {
            let mut nn = self.out.len() / 4 * 3;
            if nn > buf.len() {
                nn = buf.len();
                nn -= nn % 3;
            }
            self.enc.encode(&buf[..nn], &mut self.out);
            self.w.write_all(&self.out[..nn / 3 * 4])?;
            n += nn;
            buf = &buf[nn..];
        }

        // Trailing fringe.
        crate::copy(buf, &mut self.buf);
        self.nbuf = buf.len();
        n += buf.len();
        Ok(n)
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        if self.nbuf > 0 {
            self.enc.encode(&self.buf[..self.nbuf], &mut self.out);
            self.w
                .write_all(&self.out[..self.enc.encoded_len(self.nbuf)])?;
            self.nbuf = 0;
        }
        Ok(())
    }
}

#[cfg(all(feature = "std", feature = "io"))]
impl<W: std::io::Write> crate::io::Closer for Encoder<W> {
    fn close(&mut self) -> std::io::Result<()> {
        use std::io::Write;
        self.flush()
    }
}

struct NewLineFilteringReader<R> {
    wrapped: R,
}

#[cfg(feature = "std")]
impl<R: std::io::Read> std::io::Read for NewLineFilteringReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut n = self.wrapped.read(buf)?;
        while n > 0 {
            let mut offset = 0;
            for i in 0..n {
                if buf[i] != b'\r' && buf[i] != b'\n' {
                    if i != offset {
                        buf[offset] = buf[i];
                    }
                    offset += 1;
                }
            }
            if offset > 0 {
                return Ok(offset);
            }
            // Previous buffer entirely whitespace, read again
            n = self.wrapped.read(buf)?;
        }
        Ok(n)
    }
}

/// Base64 decoder
#[cfg(feature = "alloc")]
pub struct Decoder<R> {
    eof: bool,
    r: NewLineFilteringReader<R>,
    enc: Base64,
    buf: [u8; 1024],
    nbuf: usize,
    out: alloc::vec::Vec<u8>,
    outbuf: [u8; 1024 / 4 * 3],
}

#[cfg(feature = "alloc")]
impl<R> Decoder<R> {
    /// Create a new decoder
    #[inline]
    pub const fn new(enc: Base64, r: R) -> Decoder<R> {
        Decoder {
            eof: false,
            r: NewLineFilteringReader { wrapped: r },
            enc,
            buf: [0; 1024],
            nbuf: 0,
            out: alloc::vec::Vec::new(),
            outbuf: [0; 1024 / 4 * 3],
        }
    }
}

#[cfg(feature = "std")]
impl<R: std::io::Read> std::io::Read for Decoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // Use leftover decoded output from last read.
        if !self.out.is_empty() {
            let n = crate::copy(&self.out, buf);
            self.out.drain(..n);
            return Ok(n);
        }

        // This code assumes that d.r strips supported whitespace ('\r' and '\n').
        let mut n = 0;

        // Refill buffer.
        while self.nbuf < 4 && !self.eof {
            let mut nn = buf.len() / 3 * 4;
            if nn < 4 {
                nn = 4;
            }
            if nn > self.buf.len() {
                nn = self.buf.len();
            }
            nn = self.r.read(&mut self.buf[self.nbuf..nn])?;
            if nn == 0 {
                self.eof = true;
                break;
            }
            self.nbuf += nn;
        }

        if self.nbuf < 4 {
            if self.enc.pad_char.is_none() && self.nbuf > 0 {
                // Decode final fragment, without padding.
                let nw = self
                    .enc
                    .decode(&self.buf[..self.nbuf], &mut self.outbuf)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                self.nbuf = 0;
                self.out.resize(nw, 0);
                self.out[..nw].copy_from_slice(&self.outbuf[..nw]);
                n = crate::copy(&self.out, buf);
                self.out.drain(..n);
                if n > 0 || buf.is_empty() && !self.out.is_empty() {
                    return Ok(n);
                }
            }

            if n == 0 && self.nbuf > 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "base64 decoder: unexpected EOF",
                ));
            }
        }

        // Decode chunk into p, or d.out and then p if p is too small.
        let (nr, mut nw) = (self.nbuf / 4 * 4, self.nbuf / 4 * 3);
        if nw > buf.len() {
            nw = self
                .enc
                .decode(&self.buf[..nr], &mut self.outbuf)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            self.out.resize(nw, 0);
            self.out[..nw].copy_from_slice(&self.outbuf[..nw]);
            n = crate::copy(&self.out, buf);
            self.out.drain(..n);
        } else {
            n = self
                .enc
                .decode(&self.buf[..nr], buf)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        }
        self.nbuf -= nr;
        self.buf.copy_within(nr..nr + self.nbuf, 0);

        Ok(n)
    }
}

/// Assembles 4 base64 digits into 3 bytes.
/// Each digit comes from the decode map, and will be 0xff
/// if it came from an invalid character.
#[inline]
const fn assemble_32(n1: u8, n2: u8, n3: u8, n4: u8) -> (u32, bool) {
    // Check that all the digits are valid. If any of them was 0xff, their
    // bitwise OR will be 0xff.
    if n1 | n2 | n3 | n4 == 0xff {
        return (0, false);
    }
    (
        ((n1 as u32) << 26) | ((n2 as u32) << 20) | ((n3 as u32) << 14) | ((n4 as u32) << 8),
        true,
    )
}

/// Assembles 8 base64 digits into 6 bytes.
/// Each digit comes from the decode map, and will be 0xff
/// if it came from an invalid character.
#[inline]
#[allow(clippy::too_many_arguments)]
const fn assemble_64(
    n1: u8,
    n2: u8,
    n3: u8,
    n4: u8,
    n5: u8,
    n6: u8,
    n7: u8,
    n8: u8,
) -> (u64, bool) {
    // Check that all the digits are valid. If any of them was 0xff, their
    // bitwise OR will be 0xff.
    if n1 | n2 | n3 | n4 | n5 | n6 | n7 | n8 == 0xff {
        return (0, false);
    }
    (
        ((n1 as u64) << 58)
            | ((n2 as u64) << 52)
            | ((n3 as u64) << 46)
            | ((n4 as u64) << 40)
            | ((n5 as u64) << 34)
            | ((n6 as u64) << 28)
            | ((n7 as u64) << 22)
            | ((n8 as u64) << 16),
        true,
    )
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use crate::io::Closer;

    use super::*;

    fn big_test() -> TestPair {
        TestPair {
            decoded: b"Twas brillig, and the slithy toves".to_vec(),
            encoded: b"VHdhcyBicmlsbGlnLCBhbmQgdGhlIHNsaXRoeSB0b3Zlcw==".to_vec(),
        }
    }

    struct TestPair {
        decoded: Vec<u8>,
        encoded: Vec<u8>,
    }

    fn pairs() -> Vec<TestPair> {
        vec![
            // RFC 3548 examples
            TestPair {
                decoded: vec![20, 251, 156, 3, 217, 126],
                encoded: b"FPucA9l+".to_vec(),
            },
            TestPair {
                decoded: vec![20, 251, 156, 3, 217],
                encoded: b"FPucA9k=".to_vec(),
            },
            TestPair {
                decoded: vec![20, 251, 156, 3],
                encoded: b"FPucAw==".to_vec(),
            },
            // RFC 4648 examples
            TestPair {
                decoded: b"".to_vec(),
                encoded: b"".to_vec(),
            },
            TestPair {
                decoded: b"f".to_vec(),
                encoded: b"Zg==".to_vec(),
            },
            TestPair {
                decoded: b"fo".to_vec(),
                encoded: b"Zm8=".to_vec(),
            },
            TestPair {
                decoded: b"foo".to_vec(),
                encoded: b"Zm9v".to_vec(),
            },
            TestPair {
                decoded: b"foob".to_vec(),
                encoded: b"Zm9vYg==".to_vec(),
            },
            TestPair {
                decoded: b"fooba".to_vec(),
                encoded: b"Zm9vYmE=".to_vec(),
            },
            TestPair {
                decoded: b"foobar".to_vec(),
                encoded: b"Zm9vYmFy".to_vec(),
            },
            // Wikipedia examples
            TestPair {
                decoded: b"sure.".to_vec(),
                encoded: b"c3VyZS4=".to_vec(),
            },
            TestPair {
                decoded: b"sure".to_vec(),
                encoded: b"c3VyZQ==".to_vec(),
            },
            TestPair {
                decoded: b"sur".to_vec(),
                encoded: b"c3Vy".to_vec(),
            },
            TestPair {
                decoded: b"su".to_vec(),
                encoded: b"c3U=".to_vec(),
            },
            TestPair {
                decoded: b"leasure.".to_vec(),
                encoded: b"bGVhc3VyZS4=".to_vec(),
            },
            TestPair {
                decoded: b"easure.".to_vec(),
                encoded: b"ZWFzdXJlLg==".to_vec(),
            },
            TestPair {
                decoded: b"asure.".to_vec(),
                encoded: b"YXN1cmUu".to_vec(),
            },
            TestPair {
                decoded: b"sure.".to_vec(),
                encoded: b"c3VyZS4=".to_vec(),
            },
        ]
    }

    struct EncodingTest {
        enc: Base64,
        conv: Box<dyn Fn(String) -> String>,
    }

    fn std_ref(r: String) -> String {
        r
    }

    fn url_ref(r: String) -> String {
        r.replace('+', "-").replace('/', "_")
    }

    fn raw_ref(r: String) -> String {
        r.trim_end_matches('=').to_string()
    }

    fn raw_url_ref(r: String) -> String {
        raw_ref(url_ref(r))
    }

    const FUNNY_ENCODING: Base64 = STD_ENCODING.with_padding_unchecked(Some('@'));

    fn funny_ref(r: String) -> String {
        r.replace('=', "@")
    }

    fn encoding_tests() -> Vec<EncodingTest> {
        vec![
            EncodingTest {
                enc: STD_ENCODING,
                conv: Box::new(std_ref),
            },
            EncodingTest {
                enc: URL_ENCODING,
                conv: Box::new(url_ref),
            },
            EncodingTest {
                enc: RAW_STD_ENCODING,
                conv: Box::new(raw_ref),
            },
            EncodingTest {
                enc: RAW_URL_ENCODING,
                conv: Box::new(raw_url_ref),
            },
            EncodingTest {
                enc: FUNNY_ENCODING,
                conv: Box::new(funny_ref),
            },
            EncodingTest {
                enc: STD_ENCODING.with_strict(),
                conv: Box::new(std_ref),
            },
            EncodingTest {
                enc: URL_ENCODING.with_strict(),
                conv: Box::new(url_ref),
            },
            EncodingTest {
                enc: RAW_STD_ENCODING.with_strict(),
                conv: Box::new(raw_ref),
            },
            EncodingTest {
                enc: RAW_URL_ENCODING.with_strict(),
                conv: Box::new(raw_url_ref),
            },
            EncodingTest {
                enc: FUNNY_ENCODING.with_strict(),
                conv: Box::new(funny_ref),
            },
        ]
    }

    #[test]
    fn test_encode() {
        for p in pairs() {
            for tt in encoding_tests() {
                let got = tt.enc.encode_to_vec(&p.decoded);
                assert_eq!(
                    got,
                    (tt.conv)(String::from_utf8_lossy(&p.encoded).to_string()).as_bytes()
                );
            }
        }
    }

    #[test]
    fn test_encoder() {
        for p in pairs() {
            let mut bb = vec![];
            let mut encoder = STD_ENCODING.encoder(&mut bb);
            encoder.write_all(&p.decoded).unwrap();
            encoder.close().unwrap();
            assert_eq!(bb, p.encoded);
        }
    }

    #[test]
    fn test_encoder_buffering() {
        let input = big_test().decoded;
        for bs in 1..=12 {
            let mut bb = vec![];
            let mut encoder = STD_ENCODING.encoder(&mut bb);
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
            assert_eq!(bb, big_test().encoded);
        }
    }

    #[test]
    fn test_decode() {
        for p in pairs() {
            for tt in encoding_tests() {
                let encoded = (tt.conv)(String::from_utf8_lossy(p.encoded.as_slice()).to_string());
                let mut dbuf = vec![0; tt.enc.decoded_len(encoded.len())];
                let count = tt.enc.decode(encoded.as_bytes(), &mut dbuf).unwrap();
                assert_eq!(count, p.decoded.len());
                assert_eq!(&dbuf[..count], &p.decoded);

                let dbuf = tt.enc.decode_to_vec(encoded.as_bytes()).unwrap();
                assert_eq!(dbuf, p.decoded);
            }
        }
    }

    #[test]
    fn test_decoder() {
        for p in pairs() {
            let mut dbuf = vec![0; STD_ENCODING.decoded_len(p.encoded.len())];
            let mut decoder = STD_ENCODING.decoder(std::io::Cursor::new(&p.encoded));
            let count = decoder.read(&mut dbuf).unwrap();
            assert_eq!(count, p.decoded.len());
            assert_eq!(&dbuf[..count], &p.decoded);
        }
    }

    #[test]
    fn test_decoder_buffering() {
        let input = big_test();
        for bs in 1..=12 {
            let mut decoder = STD_ENCODING.decoder(std::io::Cursor::new(&input.encoded));
            let mut buf = vec![0; input.decoded.len() + 12];
            let mut total = 0;
            while total < input.decoded.len() {
                total += decoder.read(&mut buf[total..total + bs]).unwrap();
            }
            assert_eq!(&buf[..total], &input.decoded);
        }
    }

    #[test]
    fn test_decode_corrupt() {
        struct TestCase {
            input: Vec<u8>,
            offset: isize,
        }

        let test_cases = vec![
            TestCase {
                input: b"".to_vec(),
                offset: -1,
            },
            TestCase {
                input: b"\n".to_vec(),
                offset: -1,
            },
            TestCase {
                input: b"AAA=\n".to_vec(),
                offset: -1,
            },
            TestCase {
                input: b"AAAA\n".to_vec(),
                offset: -1,
            },
            TestCase {
                input: b"!!!!".to_vec(),
                offset: 0,
            },
            TestCase {
                input: b"====".to_vec(),
                offset: 0,
            },
            TestCase {
                input: b"x===".to_vec(),
                offset: 1,
            },
            TestCase {
                input: b"=AAA".to_vec(),
                offset: 0,
            },
            TestCase {
                input: b"A=AA".to_vec(),
                offset: 1,
            },
            TestCase {
                input: b"AA=A".to_vec(),
                offset: 2,
            },
            TestCase {
                input: b"AA==A".to_vec(),
                offset: 4,
            },
            TestCase {
                input: b"AAA=AAAA".to_vec(),
                offset: 4,
            },
            TestCase {
                input: b"AAAAA".to_vec(),
                offset: 4,
            },
            TestCase {
                input: b"AAAAAA".to_vec(),
                offset: 4,
            },
            TestCase {
                input: b"A=".to_vec(),
                offset: 1,
            },
            TestCase {
                input: b"A==".to_vec(),
                offset: 1,
            },
            TestCase {
                input: b"AA=".to_vec(),
                offset: 3,
            },
            TestCase {
                input: b"AA==".to_vec(),
                offset: -1,
            },
            TestCase {
                input: b"AAA=".to_vec(),
                offset: -1,
            },
            TestCase {
                input: b"AAAA".to_vec(),
                offset: -1,
            },
            TestCase {
                input: b"AAAAAA=".to_vec(),
                offset: 7,
            },
            TestCase {
                input: b"YWJjZA=====".to_vec(),
                offset: 8,
            },
            TestCase {
                input: b"A!\n".to_vec(),
                offset: 1,
            },
            TestCase {
                input: b"A=\n".to_vec(),
                offset: 1,
            },
        ];

        for tc in test_cases {
            let mut dbuf = vec![0; STD_ENCODING.decoded_len(tc.input.len())];
            if tc.offset == -1 {
                let _ = STD_ENCODING.decode(&tc.input, &mut dbuf).unwrap();
                continue;
            }

            let n = STD_ENCODING
                .decode(&tc.input, &mut dbuf)
                .unwrap_err()
                .into_inner();
            assert_eq!(n, tc.offset as usize);
        }
    }

    #[test]
    fn test_decode_bounds() {
        let mut buf = [0; 32];
        let s = STD_ENCODING.encode_to_vec(&buf);
        let n = STD_ENCODING.decode(&s, &mut buf).unwrap();
        assert_eq!(n, buf.len());
    }

    struct Test {
        enc: Base64,
        n: usize,
        want: usize,
    }

    #[test]
    fn test_encoded_len() {
        for tt in vec![
            Test {
                enc: RAW_STD_ENCODING,
                n: 0,
                want: 0,
            },
            Test {
                enc: RAW_STD_ENCODING,
                n: 1,
                want: 2,
            },
            Test {
                enc: RAW_STD_ENCODING,
                n: 2,
                want: 3,
            },
            Test {
                enc: RAW_STD_ENCODING,
                n: 3,
                want: 4,
            },
            Test {
                enc: RAW_STD_ENCODING,
                n: 7,
                want: 10,
            },
            Test {
                enc: STD_ENCODING,
                n: 0,
                want: 0,
            },
            Test {
                enc: STD_ENCODING,
                n: 1,
                want: 4,
            },
            Test {
                enc: STD_ENCODING,
                n: 2,
                want: 4,
            },
            Test {
                enc: STD_ENCODING,
                n: 3,
                want: 4,
            },
            Test {
                enc: STD_ENCODING,
                n: 4,
                want: 8,
            },
            Test {
                enc: STD_ENCODING,
                n: 7,
                want: 12,
            },
        ] {
            assert_eq!(tt.enc.encoded_len(tt.n), tt.want, "encoded_len({})", tt.n);
        }
    }

    #[test]
    fn test_decoded_len() {
        for tt in vec![
            Test {
                enc: RAW_STD_ENCODING,
                n: 0,
                want: 0,
            },
            Test {
                enc: RAW_STD_ENCODING,
                n: 2,
                want: 1,
            },
            Test {
                enc: RAW_STD_ENCODING,
                n: 3,
                want: 2,
            },
            Test {
                enc: RAW_STD_ENCODING,
                n: 4,
                want: 3,
            },
            Test {
                enc: RAW_STD_ENCODING,
                n: 10,
                want: 7,
            },
            Test {
                enc: STD_ENCODING,
                n: 0,
                want: 0,
            },
            Test {
                enc: STD_ENCODING,
                n: 4,
                want: 3,
            },
            Test {
                enc: STD_ENCODING,
                n: 8,
                want: 6,
            },
        ] {
            let got = tt.enc.decoded_len(tt.n);
            assert_eq!(got, tt.want);
        }
    }

    #[test]
    fn test_big() {
        const N: usize = 3 * 1000 + 1;
        let mut raw = [0; N];
        const ALPHA: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        for i in 0..N {
            raw[i] = ALPHA[i % ALPHA.len()];
        }

        let mut encoded = vec![];
        let mut w = STD_ENCODING.encoder(&mut encoded);
        let nn = w.write(&raw).unwrap();
        assert_eq!(nn, N);

        w.close().unwrap();
        let mut dbuf = vec![];
        let mut decoded = STD_ENCODING.decoder(std::io::Cursor::new(&encoded));
        decoded.read_to_end(&mut dbuf).unwrap();
        assert_eq!(dbuf, raw);
    }

    #[test]
    fn test_new_line_characters() {
        const EXPECTED: &str = "sure";
        let examples = vec![
            "c3VyZQ==",
            "c3VyZQ==\r",
            "c3VyZQ==\n",
            "c3VyZQ==\r\n",
            "c3VyZ\r\nQ==",
            "c3V\ryZ\nQ==",
            "c3V\nyZ\rQ==",
            "c3VyZ\nQ==",
            "c3VyZQ\n==",
            "c3VyZQ=\n=",
            "c3VyZQ=\r\n\r\n=",
        ];

        for e in examples {
            let buf = STD_ENCODING.decode_to_vec(e.as_bytes()).unwrap();
            assert_eq!(EXPECTED, std::str::from_utf8(&buf).unwrap());
        }
    }

    #[test]
    fn test_decoder_issue_3577() {
        // TODO: implement this test case
    }

    #[test]
    fn test_decoder_issue_4779() {
        let encoded = r#"CP/EAT8AAAEF
AQEBAQEBAAAAAAAAAAMAAQIEBQYHCAkKCwEAAQUBAQEBAQEAAAAAAAAAAQACAwQFBgcICQoLEAAB
BAEDAgQCBQcGCAUDDDMBAAIRAwQhEjEFQVFhEyJxgTIGFJGhsUIjJBVSwWIzNHKC0UMHJZJT8OHx
Y3M1FqKygyZEk1RkRcKjdDYX0lXiZfKzhMPTdePzRieUpIW0lcTU5PSltcXV5fVWZnaGlqa2xtbm
9jdHV2d3h5ent8fX5/cRAAICAQIEBAMEBQYHBwYFNQEAAhEDITESBEFRYXEiEwUygZEUobFCI8FS
0fAzJGLhcoKSQ1MVY3M08SUGFqKygwcmNcLSRJNUoxdkRVU2dGXi8rOEw9N14/NGlKSFtJXE1OT0
pbXF1eX1VmZ2hpamtsbW5vYnN0dXZ3eHl6e3x//aAAwDAQACEQMRAD8A9VSSSSUpJJJJSkkkJ+Tj
1kiy1jCJJDnAcCTykpKkuQ6p/jN6FgmxlNduXawwAzaGH+V6jn/R/wCt71zdn+N/qL3kVYFNYB4N
ji6PDVjWpKp9TSXnvTf8bFNjg3qOEa2n6VlLpj/rT/pf567DpX1i6L1hs9Py67X8mqdtg/rUWbbf
+gkp0kkkklKSSSSUpJJJJT//0PVUkkklKVLq3WMDpGI7KzrNjADtYNXvI/Mqr/Pd/q9W3vaxjnvM
NaCXE9gNSvGPrf8AWS3qmba5jjsJhoB0DAf0NDf6sevf+/lf8Hj0JJATfWT6/dV6oXU1uOLQeKKn
EQP+Hubtfe/+R7Mf/g7f5xcocp++Z11JMCJPgFBxOg7/AOuqDx8I/ikpkXkmSdU8mJIJA/O8EMAy
j+mSARB/17pKVXYWHXjsj7yIex0PadzXMO1zT5KHoNA3HT8ietoGhgjsfA+CSnvvqh/jJtqsrwOv
2b6NGNzXfTYexzJ+nU7/ALkf4P8Awv6P9KvTQQ4AgyDqCF85Pho3CTB7eHwXoH+LT65uZbX9X+o2
bqbPb06551Y4
"#;

        let encoded_stort = encoded.replace('\n', "");
        let mut buf = vec![];
        let mut decoder = STD_ENCODING.decoder(std::io::Cursor::new(encoded));
        decoder.read_to_end(&mut buf).unwrap();

        let mut buf1 = vec![];
        let mut decoder1 = STD_ENCODING.decoder(std::io::Cursor::new(encoded_stort));
        decoder1.read_to_end(&mut buf1).unwrap();
        assert_eq!(buf, buf1);
    }

    #[test]
    fn test_decode_issue_7733() {
        let err = STD_ENCODING
            .decode_to_vec(b"YWJjZA=====")
            .unwrap_err()
            .into_inner();
        assert_eq!(err, 8);
    }

    #[test]
    fn test_decode_issue_15656() {
        let err = STD_ENCODING
            .with_strict()
            .decode_to_vec(b"WvLTlMrX9NpYDQlEIFlnDB==")
            .unwrap_err()
            .into_inner();
        assert_eq!(err, 22);
        STD_ENCODING
            .with_strict()
            .decode_to_vec(b"WvLTlMrX9NpYDQlEIFlnDA==")
            .unwrap();
        STD_ENCODING
            .decode_to_vec(b"WvLTlMrX9NpYDQlEIFlnDB==")
            .unwrap();
    }
}
