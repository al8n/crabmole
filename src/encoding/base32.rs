/// `BASE = 32`
pub const BASE: usize = 32;
const ENCODE_STD: [u8; BASE] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const ENCODE_HEX: [u8; BASE] = *b"0123456789ABCDEFGHIJKLMNOPQRSTUV";
const DECODE_MAP_INITIALIZE: [u8; 256] = [255; 256];

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
            Error::InvalidEncoder => write!(f, "Base32 alphabet must be 32 bytes long"),
            Error::InvalidPadding => write!(f, "Invalid padding character"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// No padding
pub const NO_PADDING: Option<char> = None;

/// Standard padding
pub const STD_PADDING: Option<char> = Some('=');

/// The standard base32 encoding, as defined in
/// RFC 4648.
pub const STD_ENCODING: Base32 = Base32::new_unchecked(ENCODE_STD);

/// The standard raw, unpadded base32 encoding.
/// This is the same as [`STD_ENCODING`] but omits padding characters.
pub const RAW_STD_ENCODING: Base32 = Base32::new_unchecked(ENCODE_STD).with_padding_unchecked(None);

/// The `Extended Hex Alphabet` defined in RFC 4648.
/// It is typically used in DNS.
pub const HEX_ENCODING: Base32 = Base32::new_unchecked(ENCODE_HEX);

/// The `Extended Hex Alphabet` defined in RFC 4648.
/// It is typically used in DNS.
/// This is the same as [`HEX_ENCODING`] but omits padding characters.
pub const RAW_HEX_ENCODING: Base32 = Base32::new_unchecked(ENCODE_HEX).with_padding_unchecked(None);

/// An Base32 is a radix 32 encoding/decoding scheme, defined by a
/// 32-character alphabet. The most common is the "base32" encoding
/// introduced for SASL GSSAPI and standardized in RFC 4648.
/// The alternate "base32hex" encoding is used in DNSSEC.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Base32 {
    encode: [u8; BASE],
    decode_map: [u8; 256],
    pad_char: Option<char>,
}

impl Default for Base32 {
    fn default() -> Self {
        STD_ENCODING
    }
}

impl Base32 {
    /// Returns a new Base32 defined by the given alphabet,
    /// which must be a 32-byte array that does not contain the padding character
    /// or CR / LF ('\r', '\n').
    /// The resulting [`Base32`] uses the default padding character ('='),
    /// which may be changed or disabled via [`Base32::with_padding`].
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
        })
    }

    /// Returns a new padded Base32 defined by the given alphabet,
    /// which must be a 64-byte array that does not contain the padding character
    /// or CR / LF ('\r', '\n').
    /// The resulting Base32 uses the default padding character ('='),
    /// which may be changed or disabled via [`Base32::with_padding_unchecked`].
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
        }
    }

    /// Creates a new encoding identical to enc except
    /// with a specified padding character, or NoPadding to disable padding.
    /// The padding character must not be '\r' or '\n', must not
    /// be contained in the encoding's alphabet and must be a rune equal or
    /// below '\xff'.
    #[inline]
    pub const fn with_padding(self, padding: Option<char>) -> Result<Self, Error> {
        let Self {
            encode: encoder,
            mut decode_map,
            pad_char: _,
        } = self;

        match padding {
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
            pad_char: padding,
        })
    }

    /// Creates a new encoding identical to enc except
    /// with a specified padding character, or [`NO_PADDING`] to disable padding.
    /// The padding character must not be '\r' or '\n', must not
    /// be contained in the encoding's alphabet and must be a rune equal or
    /// below '\xff'.
    ///
    /// # Panic
    /// 32-byte array that contains the padding character
    /// or CR / LF ('\r', '\n').
    #[inline]
    pub const fn with_padding_unchecked(self, pad: Option<char>) -> Self {
        let Self {
            encode: encoder,
            mut decode_map,
            pad_char: _,
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
        }
    }

    /// Returns an encoder
    #[cfg(feature = "std")]
    #[inline]
    pub const fn encoder<W: std::io::Write>(&self, writer: W) -> Encoder<W> {
        Encoder::new(*self, writer)
    }

    /// Returns a base32 decoder
    #[cfg(feature = "std")]
    #[inline]
    pub const fn decoder<R: std::io::Read>(&self, reader: R) -> Decoder<R> {
        Decoder::new(*self, reader)
    }

    /// Returns the length in bytes of the base32 encoding
    /// of source slice.
    #[inline]
    pub const fn encoded_len(&self, n: usize) -> usize {
        if self.pad_char.is_none() {
            return (n * 8 + 4) / 5;
        }

        (n + 4) / 5 * 8
    }

    /// Encodes `src` using the encoding enc, writing
    /// `encoded_len(src)` bytes to dst.
    ///
    /// The encoding pads the output to a multiple of 8 bytes,
    /// so Encode is not appropriate for use on individual blocks
    /// of a large data stream. Use NewEncoder() instead.
    #[inline]
    pub fn encode(&self, mut src: &[u8], mut dst: &mut [u8]) {
        while !src.is_empty() {
            let mut b = [0; 8];

            // Unpack 8x 5-bit source blocks into a 5 byte
            // destination quantum
            match src.len() {
                4 => {
                    b[6] |= (src[3] << 3) & 0x1F;
                    b[5] = (src[3] >> 2) & 0x1F;
                    b[4] = src[3] >> 7;
                    b[4] |= (src[2] << 1) & 0x1F;
                    b[3] = (src[2] >> 4) & 0x1F;
                    b[3] |= (src[1] << 4) & 0x1F;
                    b[2] = (src[1] >> 1) & 0x1F;
                    b[1] = (src[1] >> 6) & 0x1F;
                    b[1] |= (src[0] << 2) & 0x1F;
                    b[0] = src[0] >> 3;
                }
                3 => {
                    b[4] |= (src[2] << 1) & 0x1F;
                    b[3] = (src[2] >> 4) & 0x1F;
                    b[3] |= (src[1] << 4) & 0x1F;
                    b[2] = (src[1] >> 1) & 0x1F;
                    b[1] = (src[1] >> 6) & 0x1F;
                    b[1] |= (src[0] << 2) & 0x1F;
                    b[0] = src[0] >> 3;
                }
                2 => {
                    b[3] |= (src[1] << 4) & 0x1F;
                    b[2] = (src[1] >> 1) & 0x1F;
                    b[1] = (src[1] >> 6) & 0x1F;
                    b[1] |= (src[0] << 2) & 0x1F;
                    b[0] = src[0] >> 3;
                }
                1 => {
                    b[1] |= (src[0] << 2) & 0x1F;
                    b[0] = src[0] >> 3;
                }
                _ => {
                    b[7] = src[4] & 0x1F;
                    b[6] = src[4] >> 5;
                    b[6] |= (src[3] << 3) & 0x1F;
                    b[5] = (src[3] >> 2) & 0x1F;
                    b[4] = src[3] >> 7;
                    b[4] |= (src[2] << 1) & 0x1F;
                    b[3] = (src[2] >> 4) & 0x1F;
                    b[3] |= (src[1] << 4) & 0x1F;
                    b[2] = (src[1] >> 1) & 0x1F;
                    b[1] = (src[1] >> 6) & 0x1F;
                    b[1] |= (src[0] << 2) & 0x1F;
                    b[0] = src[0] >> 3;
                }
            }

            // Encode 5-bit blocks using the base32 alphabet
            let size = dst.len();
            if size >= 8 {
                // Common case, unrolled for extra performance
                dst[0] = self.encode[b[0] as usize & 31];
                dst[1] = self.encode[b[1] as usize & 31];
                dst[2] = self.encode[b[2] as usize & 31];
                dst[3] = self.encode[b[3] as usize & 31];
                dst[4] = self.encode[b[4] as usize & 31];
                dst[5] = self.encode[b[5] as usize & 31];
                dst[6] = self.encode[b[6] as usize & 31];
                dst[7] = self.encode[b[7] as usize & 31];
            } else {
                for i in 0..size {
                    dst[i] = self.encode[b[i] as usize & 31];
                }
            }

            // Pad the final quantum
            if src.len() < 5 {
                if let Some(padding) = self.pad_char {
                    dst[7] = padding as u8;
                    if src.len() < 4 {
                        dst[6] = padding as u8;
                        dst[5] = padding as u8;
                        if src.len() < 3 {
                            dst[4] = padding as u8;
                            if src.len() < 2 {
                                dst[3] = padding as u8;
                                dst[2] = padding as u8;
                            }
                        }
                    }
                } else {
                    break;
                }

                break;
            }

            src = &src[5..];
            dst = &mut dst[8..];
        }
    }

    /// Returns the base32 encoding of src.
    #[cfg(feature = "alloc")]
    pub fn encode_to_vec(&self, src: &[u8]) -> alloc::vec::Vec<u8> {
        let mut dst = alloc::vec![0; self.encoded_len(src.len())];
        self.encode(src, &mut dst);
        dst
    }

    /// Decodes src using the encoding enc. It writes at most
    /// DecodedLen(len(src)) bytes to dst and returns the number of bytes
    /// written. If src contains invalid base32 data, it will return the
    /// number of bytes successfully written and DecodeError.
    /// New line characters (`\r` and `\n`) are ignored.
    #[inline]
    #[cfg(feature = "alloc")]
    pub fn decode(&self, src: &[u8], dst: &mut [u8]) -> Result<usize, DecodeError> {
        let mut buf = src.to_vec();
        let l = strip_new_lines_inplace(&mut buf);
        self.decode_in(&buf[..l], dst).map(|(n, _)| n)
    }

    /// Returns the bytes represented by the base32 slice (ignore newline characters (`\r` and `\n`)).
    #[inline]
    #[cfg(feature = "alloc")]
    pub fn decode_to_vec(&self, src: &[u8]) -> Result<alloc::vec::Vec<u8>, DecodeError> {
        let mut buf = src.to_vec();
        let l = strip_new_lines_inplace(&mut buf);
        self.decode_inplace(&mut buf, l).map(|(n, _)| {
            buf.truncate(n);
            buf
        })
    }

    #[inline]
    fn decode_inplace(&self, dst: &mut [u8], src_len: usize) -> Result<(usize, bool), DecodeError> {
        let mut n = 0;
        let mut end = false;
        let mut dsti = 0;
        let src = dst;
        let olen = src_len;

        let mut src_start = 0;
        let src_end = src_len;
        while !src[src_start..src_end].is_empty() && !end {
            // Decode quantum using the base32 alphabet
            let mut dbuf = [0; 8];
            let mut dlen = 8;

            let mut j = 0;
            while j < 8 {
                if src[src_start..src_end].is_empty() {
                    if self.pad_char.is_some() {
                        // We have reached the end and are missing padding
                        return Err(DecodeError(olen - (src_end - src_start) - j));
                    }
                    // We have reached the end and are not expecting any padding
                    (dlen, end) = (j, true);
                    break;
                }

                let in_ = src[src_start];
                // src = &mut src[src_start + 1..];
                src_start += 1;

                match self.pad_char {
                    Some(padding)
                        if padding == (in_ as char) && j >= 2 && (src_end - src_start) < 8 =>
                    {
                        // We've reached the end and there's padding
                        if (src_end - src_start) + j < 8 - 1 {
                            // not enough padding
                            return Err(DecodeError(olen));
                        }

                        let mut k = 0;
                        while k < 8 - 1 - j {
                            if (src_end - src_start) > k && (src[src_start + k] as char) != padding
                            {
                                // incorrect padding
                                return Err(DecodeError(olen - (src_end - src_start)));
                            }
                            k += 1;
                        }

                        (dlen, end) = (j, true);

                        // 7, 5 and 2 are not valid padding lengths, and so 1, 3 and 6 are not
                        // valid dlen values. See RFC 4648 Section 6 "Base 32 Base32" listing
                        // the five valid padding lengths, and Section 9 "Illustrations and
                        // Examples" for an illustration for how the 1st, 3rd and 6th base32
                        // src bytes do not yield enough information to decode a dst byte.
                        if dlen == 1 || dlen == 3 || dlen == 6 {
                            return Err(DecodeError(olen - (src_end - src_start) - 1));
                        }
                        break;
                    }
                    _ => {
                        dbuf[j] = self.decode_map[in_ as usize];
                        if dbuf[j] == 0xFF {
                            return Err(DecodeError(olen - (src_end - src_start) - 1));
                        }
                        j += 1;
                    }
                }
            }

            // Pack 8x 5-bit source blocks into 5 byte destination
            // quantum
            match dlen {
                8 => {
                    src[dsti + 4] = dbuf[6] << 5 | dbuf[7];
                    n += 1;
                    src[dsti + 3] = dbuf[4] << 7 | dbuf[5] << 2 | dbuf[6] >> 3;
                    n += 1;
                    src[dsti + 2] = dbuf[3] << 4 | dbuf[4] >> 1;
                    n += 1;
                    src[dsti + 1] = dbuf[1] << 6 | dbuf[2] << 1 | dbuf[3] >> 4;
                    n += 1;
                    src[dsti] = dbuf[0] << 3 | dbuf[1] >> 2;
                    n += 1;
                }
                7 => {
                    src[dsti + 3] = dbuf[4] << 7 | dbuf[5] << 2 | dbuf[6] >> 3;
                    n += 1;
                    src[dsti + 2] = dbuf[3] << 4 | dbuf[4] >> 1;
                    n += 1;
                    src[dsti + 1] = dbuf[1] << 6 | dbuf[2] << 1 | dbuf[3] >> 4;
                    n += 1;
                    src[dsti] = dbuf[0] << 3 | dbuf[1] >> 2;
                    n += 1;
                }
                5 => {
                    src[dsti + 2] = dbuf[3] << 4 | dbuf[4] >> 1;
                    n += 1;
                    src[dsti + 1] = dbuf[1] << 6 | dbuf[2] << 1 | dbuf[3] >> 4;
                    n += 1;
                    src[dsti] = dbuf[0] << 3 | dbuf[1] >> 2;
                    n += 1;
                }
                4 => {
                    src[dsti + 1] = dbuf[1] << 6 | dbuf[2] << 1 | dbuf[3] >> 4;
                    n += 1;
                    src[dsti] = dbuf[0] << 3 | dbuf[1] >> 2;
                    n += 1;
                }
                2 => {
                    src[dsti] = dbuf[0] << 3 | dbuf[1] >> 2;
                    n += 1;
                }
                _ => {}
            }
            dsti += 5;
        }
        Ok((n, end))
    }

    /// Returns the maximum length in bytes of the decoded data
    /// corresponding to n bytes of base32-encoded data.
    #[inline]
    pub const fn decode_len(&self, n: usize) -> usize {
        if self.pad_char.is_none() {
            return n * 5 / 8;
        }
        n / 8 * 5
    }

    /// Returns an additional 'end' value, which
    /// indicates if end-of-message padding was encountered and thus any
    /// additional data is an error. This method assumes that src has been
    /// stripped of all supported whitespace ('\r' and '\n').
    #[inline]
    fn decode_in(&self, mut src: &[u8], dst: &mut [u8]) -> Result<(usize, bool), DecodeError> {
        let mut n = 0;
        let mut end = false;
        let mut dsti = 0;
        let olen = src.len();

        while !src.is_empty() && !end {
            // Decode quantum using the base32 alphabet
            let mut dbuf = [0; 8];
            let mut dlen = 8;

            let mut j = 0;
            while j < 8 {
                if src.is_empty() {
                    if self.pad_char.is_some() {
                        // We have reached the end and are missing padding
                        return Err(DecodeError(olen - src.len() - j));
                    }
                    // We have reached the end and are not expecting any padding
                    (dlen, end) = (j, true);
                    break;
                }

                let in_ = src[0];
                src = &src[1..];

                match self.pad_char {
                    Some(padding) if padding == (in_ as char) && j >= 2 && src.len() < 8 => {
                        // We've reached the end and there's padding
                        if src.len() + j < 8 - 1 {
                            // not enough padding
                            return Err(DecodeError(olen));
                        }

                        let mut k = 0;
                        while k < 8 - 1 - j {
                            if src.len() > k && (src[k] as char) != padding {
                                // incorrect padding
                                return Err(DecodeError(olen - src.len()));
                            }
                            k += 1;
                        }

                        (dlen, end) = (j, true);

                        // 7, 5 and 2 are not valid padding lengths, and so 1, 3 and 6 are not
                        // valid dlen values. See RFC 4648 Section 6 "Base 32 Base32" listing
                        // the five valid padding lengths, and Section 9 "Illustrations and
                        // Examples" for an illustration for how the 1st, 3rd and 6th base32
                        // src bytes do not yield enough information to decode a dst byte.
                        if dlen == 1 || dlen == 3 || dlen == 6 {
                            return Err(DecodeError(olen - src.len() - 1));
                        }
                        break;
                    }
                    _ => {
                        dbuf[j] = self.decode_map[in_ as usize];
                        if dbuf[j] == 0xFF {
                            return Err(DecodeError(olen - src.len() - 1));
                        }
                        j += 1;
                    }
                }
            }

            // Pack 8x 5-bit source blocks into 5 byte destination
            // quantum
            match dlen {
                8 => {
                    dst[dsti + 4] = dbuf[6] << 5 | dbuf[7];
                    n += 1;
                    dst[dsti + 3] = dbuf[4] << 7 | dbuf[5] << 2 | dbuf[6] >> 3;
                    n += 1;
                    dst[dsti + 2] = dbuf[3] << 4 | dbuf[4] >> 1;
                    n += 1;
                    dst[dsti + 1] = dbuf[1] << 6 | dbuf[2] << 1 | dbuf[3] >> 4;
                    n += 1;
                    dst[dsti] = dbuf[0] << 3 | dbuf[1] >> 2;
                    n += 1;
                }
                7 => {
                    dst[dsti + 3] = dbuf[4] << 7 | dbuf[5] << 2 | dbuf[6] >> 3;
                    n += 1;
                    dst[dsti + 2] = dbuf[3] << 4 | dbuf[4] >> 1;
                    n += 1;
                    dst[dsti + 1] = dbuf[1] << 6 | dbuf[2] << 1 | dbuf[3] >> 4;
                    n += 1;
                    dst[dsti] = dbuf[0] << 3 | dbuf[1] >> 2;
                    n += 1;
                }
                5 => {
                    dst[dsti + 2] = dbuf[3] << 4 | dbuf[4] >> 1;
                    n += 1;
                    dst[dsti + 1] = dbuf[1] << 6 | dbuf[2] << 1 | dbuf[3] >> 4;
                    n += 1;
                    dst[dsti] = dbuf[0] << 3 | dbuf[1] >> 2;
                    n += 1;
                }
                4 => {
                    dst[dsti + 1] = dbuf[1] << 6 | dbuf[2] << 1 | dbuf[3] >> 4;
                    n += 1;
                    dst[dsti] = dbuf[0] << 3 | dbuf[1] >> 2;
                    n += 1;
                }
                2 => {
                    dst[dsti] = dbuf[0] << 3 | dbuf[1] >> 2;
                    n += 1;
                }
                _ => {}
            }
            dsti += 5;
        }
        Ok((n, end))
    }
}

/// Removes newline characters and returns the number
/// of non-newline characters copied to dst.
#[inline]
fn strip_new_lines_inplace(dst: &mut [u8]) -> usize {
    let mut offset = 0;
    for i in 0..dst.len() {
        let b = dst[i];
        if b.eq(&b'\r') || b.eq(&b'\n') {
            continue;
        }
        dst[offset] = b;
        offset += 1;
    }
    offset
}

/// Wrong input for decoding
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DecodeError(usize);

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "illegal base32 data at input byte {}", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {}

/// Base32 encoder
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Encoder<W> {
    enc: Base32,
    writer: W,
    buf: [u8; 5],
    nbuf: usize,
    out: [u8; 1024],
}

impl<W> Encoder<W> {
    /// Returns a new base32 stream encoder. Data written to
    /// the returned writer will be encoded using enc and then written to w.
    /// Base32 encodings operate in 5-byte blocks; when finished
    /// writing, the caller must **Flush** the returned encoder to flush any
    /// partially written blocks.
    #[inline]
    pub const fn new(enc: Base32, writer: W) -> Self {
        Self {
            enc,
            writer,
            buf: [0; 5],
            nbuf: 0,
            out: [0; 1024],
        }
    }
}

#[cfg(feature = "std")]
impl<W: std::io::Write> std::io::Write for Encoder<W> {
    fn write(&mut self, mut buf: &[u8]) -> std::io::Result<usize> {
        let mut n = 0;

        // leading fringe.
        if self.nbuf > 0 {
            let mut i = 0;
            while i < buf.len() && self.nbuf < 5 {
                self.buf[self.nbuf] = buf[i];
                self.nbuf += 1;
                i += 1;
            }
            n += i;
            buf = &buf[i..];
            if self.nbuf < 5 {
                return Ok(n);
            }
            self.enc.encode(&self.buf, &mut self.out);
            self.writer.write_all(&self.out[..8])?;
            self.nbuf = 0;
        }

        // Large interior chunks.
        while buf.len() >= 5 {
            let mut nn = self.out.len() / 8 * 5;
            if nn > buf.len() {
                nn = buf.len();
                nn -= nn % 5;
            }

            self.enc.encode(&buf[..nn], &mut self.out);
            self.writer.write_all(&self.out[..nn / 5 * 8])?;
            n += nn;
            buf = &buf[nn..];
        }

        // Trailing fringe.
        // Safety: buf.len() always less of equal to self.buf.len()
        crate::copy(buf, &mut self.buf);
        self.nbuf = buf.len();
        n += buf.len();
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if self.nbuf > 0 {
            self.enc.encode(&self.buf[..self.nbuf], &mut self.out);
            let encoded_len = self.enc.encoded_len(self.nbuf);
            self.nbuf = 0;
            self.writer.write_all(&self.out[..encoded_len])?;
        }
        self.writer.flush()
    }
}

#[cfg(all(feature = "std", feature = "io"))]
impl<W: std::io::Write> crate::io::Closer for Encoder<W> {
    fn close(&mut self) -> std::io::Result<()> {
        use std::io::Write;
        self.flush()
    }
}

/// A reader wrapper can filter newline characters when decoding base32 stream.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct NewLineFilteringReader<R> {
    wrapped: R,
}

impl<R> NewLineFilteringReader<R> {
    /// Creates a `NewlineFilteringReader` that wraps the given reader.
    #[inline]
    const fn new(reader: R) -> NewLineFilteringReader<R> {
        NewLineFilteringReader { wrapped: reader }
    }
}

#[cfg(feature = "std")]
impl<R: std::io::Read> From<R> for NewLineFilteringReader<R> {
    fn from(value: R) -> Self {
        Self { wrapped: value }
    }
}

impl<R: std::io::Read> std::io::Read for NewLineFilteringReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self.wrapped.read(buf) {
            Ok(mut n) => {
                while n > 0 {
                    let s = &mut buf[..n];
                    let offset = strip_new_lines_inplace(s);
                    if offset > 0 {
                        return Ok(offset);
                    }

                    // Previous buffer entirely whitespace, read again
                    n = self.wrapped.read(buf)?;
                }
                Ok(n)
            }
            Err(e) => Err(e),
        }
    }
}

#[inline]
fn read_encoded_data(
    r: &mut impl std::io::Read,
    buf: &mut [u8],
    min: usize,
    expects_padding: bool,
) -> std::io::Result<usize> {
    let mut n = 0;
    while n < min {
        n += r.read(&mut buf[n..])?;
        if n == 0 {
            break;
        }
    }

    // data was read, less than min bytes could be read
    if n < min && n > 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "base32 decoder: data was read, less than min bytes could be read",
        ));
    }

    // no data was read, the buffer already contains some data
    // when padding is disabled this is not an error, as the message can be of
    // any length
    if expects_padding && min < 8 && n == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "base32 decoder: no data was read, the buffer already contains some data",
        ));
    }
    Ok(n)
}

/// Base32 decoder
#[cfg(feature = "std")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Decoder<R> {
    enc: Base32,
    reader: NewLineFilteringReader<R>,
    end: bool,
    buf: [u8; 1024],
    nbuf: usize,
    out: std::vec::Vec<u8>,
    out_buf: [u8; 1024 / 8 * 5],
}

#[cfg(feature = "std")]
impl<R> Decoder<R> {
    /// Constructs a new base32 stream decoder.
    #[inline]
    pub const fn new(enc: Base32, reader: R) -> Self {
        Self {
            enc,
            reader: NewLineFilteringReader::new(reader),
            end: false,
            buf: [0; 1024],
            nbuf: 0,
            out: std::vec::Vec::new(),
            out_buf: [0; 1024 / 8 * 5],
        }
    }
}

#[cfg(feature = "std")]
impl<R: std::io::Read> std::io::Read for Decoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n;
        // Use leftover decoded output from last read.
        if !self.out.is_empty() {
            n = crate::copy(&self.out, buf);
            self.out.drain(..self.out.len().min(n));
            return Ok(n);
        }

        // Read a chunk
        let mut nn = (buf.len() / 5 * 8).clamp(8, 1024);

        // Minimum amount of bytes that needs to be read each cycle
        let (min, expects_padding) = match self.enc.pad_char {
            Some(_) => (8 - self.nbuf, true),
            None => (1, false),
        };

        nn = read_encoded_data(
            &mut self.reader,
            &mut self.buf[self.nbuf..nn],
            min,
            expects_padding,
        )?;
        self.nbuf += nn;
        if self.nbuf < min {
            return Ok(0);
        }

        if nn > 0 && self.end {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                DecodeError(0),
            ));
        }

        // Decode chunk into p, or d.out and then p if p is too small.
        let nr = match self.enc.pad_char {
            Some(_) => self.nbuf / 8 * 8,
            None => self.nbuf,
        };

        let mut nw = self.enc.decode_len(self.nbuf);
        if nw > buf.len() {
            (nw, self.end) = self
                .enc
                .decode_in(&self.buf[..nr], &mut self.out_buf)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            self.out = self.out_buf[..nw].to_vec();
            n = crate::copy(&self.out, buf);
            self.out.drain(..n);
        } else {
            (n, self.end) = self
                .enc
                .decode_in(&self.buf[..nr], buf)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        }
        self.nbuf -= nr;
        for i in 0..self.nbuf {
            self.buf[i] = self.buf[i + nr];
        }

        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};

    struct TestPair {
        decoded: String,
        encoded: String,
    }

    fn pairs() -> Vec<TestPair> {
        vec![
            // RFC 4648 examples
            TestPair {
                decoded: Default::default(),
                encoded: Default::default(),
            },
            TestPair {
                decoded: "f".to_string(),
                encoded: "MY======".to_string(),
            },
            TestPair {
                decoded: "fo".to_string(),
                encoded: "MZXQ====".to_string(),
            },
            TestPair {
                decoded: "foo".to_string(),
                encoded: "MZXW6===".to_string(),
            },
            TestPair {
                decoded: "foob".to_string(),
                encoded: "MZXW6YQ=".to_string(),
            },
            TestPair {
                decoded: "fooba".to_string(),
                encoded: "MZXW6YTB".to_string(),
            },
            TestPair {
                decoded: "foobar".to_string(),
                encoded: "MZXW6YTBOI======".to_string(),
            },
            // Wikipedia examples, converted to base32
            TestPair {
                decoded: "sure.".to_string(),
                encoded: "ON2XEZJO".to_string(),
            },
            TestPair {
                decoded: "sure".to_string(),
                encoded: "ON2XEZI=".to_string(),
            },
            TestPair {
                decoded: "sur".to_string(),
                encoded: "ON2XE===".to_string(),
            },
            TestPair {
                decoded: "su".to_string(),
                encoded: "ON2Q====".to_string(),
            },
            TestPair {
                decoded: "leasure.".to_string(),
                encoded: "NRSWC43VOJSS4===".to_string(),
            },
            TestPair {
                decoded: "easure.".to_string(),
                encoded: "MVQXG5LSMUXA====".to_string(),
            },
            TestPair {
                decoded: "asure.".to_string(),
                encoded: "MFZXK4TFFY======".to_string(),
            },
            TestPair {
                decoded: "sure.".to_string(),
                encoded: "ON2XEZJO".to_string(),
            },
        ]
    }

    fn big_test() -> TestPair {
        TestPair {
            decoded: "Twas brillig, and the slithy toves".to_string(),
            encoded: "KR3WC4ZAMJZGS3DMNFTSYIDBNZSCA5DIMUQHG3DJORUHSIDUN53GK4Y=".to_string(),
        }
    }

    #[test]
    fn test_encode() {
        for p in pairs() {
            let got = STD_ENCODING.encode_to_vec(p.decoded.as_bytes());
            assert_eq!(got, p.encoded.as_bytes());
        }
    }

    #[test]
    fn test_encoder() {
        for p in pairs() {
            let mut got = Vec::new();
            let mut encoder = STD_ENCODING.encoder(&mut got);
            let _ = encoder.write(p.decoded.as_bytes()).unwrap();
            encoder.flush().unwrap();
            assert_eq!(got, p.encoded.as_bytes());
        }
    }

    #[test]
    fn test_encoder_buffering() {
        let input = big_test();
        for bs in 1..=12 {
            let mut bb = Vec::new();
            let mut encoder = STD_ENCODING.encoder(&mut bb);
            let mut pos = 0;
            while pos < input.decoded.len() {
                let mut end = pos + bs;
                if end > input.decoded.len() {
                    end = input.decoded.len();
                }

                let n = encoder.write(&input.decoded.as_bytes()[pos..end]).unwrap();
                assert_eq!(n, end - pos);
                pos += bs;
            }

            encoder.flush().unwrap();
            assert_eq!(bb, input.encoded.as_bytes());
        }
    }

    #[test]
    fn test_decode() {
        for p in pairs() {
            let mut dbuf = vec![0; STD_ENCODING.decode_len(p.encoded.len())];
            let (n, end) = STD_ENCODING
                .decode_in(p.encoded.as_bytes(), &mut dbuf)
                .unwrap();
            assert_eq!(n, p.decoded.len());
            if !p.encoded.is_empty() {
                assert_eq!(end, p.encoded.as_bytes()[p.encoded.len() - 1] == b'=');
            }
            assert_eq!(&dbuf[..n], p.decoded.as_bytes());

            let dbuf = STD_ENCODING.decode_to_vec(p.encoded.as_bytes()).unwrap();
            assert_eq!(dbuf, p.decoded.as_bytes());
        }
    }

    #[test]
    fn test_decoder() {
        for p in pairs() {
            let mut reader = p.encoded.as_bytes().to_vec();
            let mut decoder =
                Base32::new_unchecked(ENCODE_STD).decoder(std::io::Cursor::new(&mut reader));
            let mut dbuf = vec![0; STD_ENCODING.decode_len(p.encoded.len())];
            match decoder.read(&mut dbuf) {
                Ok(n) => {
                    assert_eq!(n, p.decoded.len());
                    assert_eq!(&dbuf[..n], p.decoded.as_bytes());
                }
                Err(e) => {
                    assert_eq!(e.kind(), std::io::ErrorKind::UnexpectedEof);
                }
            }
        }
    }

    #[test]
    fn test_decoder_buffering() {
        let big = big_test();
        for bs in 1..=12 {
            let mut decoder = STD_ENCODING.decoder(std::io::Cursor::new(big.encoded.as_bytes()));
            let mut buf = vec![0; big.decoded.len() + 12];
            let mut total = 0;
            while total < big.decoded.len() {
                match decoder.read(&mut buf[total..total + bs]) {
                    Ok(nn) => {
                        total += nn;
                    }
                    Err(e) => {
                        if e.kind() != std::io::ErrorKind::UnexpectedEof {
                            panic!("unexpected error: {}", e);
                        }
                        break;
                    }
                }
            }
            assert_eq!(big.decoded.as_bytes(), &buf[..total]);
        }
    }

    #[test]
    fn test_decode_corrupt() {
        struct Case {
            input: &'static str,
            offset: isize, // -1 means no corruption.
        }

        let cases = vec![
            Case {
                input: "",
                offset: -1,
            },
            Case {
                input: "!!!!",
                offset: 0,
            },
            Case {
                input: "x===",
                offset: 0,
            },
            Case {
                input: "AA=A====",
                offset: 2,
            },
            Case {
                input: "AAA=AAAA",
                offset: 3,
            },
            Case {
                input: "MMMMMMMMM",
                offset: 8,
            },
            Case {
                input: "MMMMMM",
                offset: 0,
            },
            Case {
                input: "A=",
                offset: 1,
            },
            Case {
                input: "AA=",
                offset: 3,
            },
            Case {
                input: "AA==",
                offset: 4,
            },
            Case {
                input: "AA===",
                offset: 5,
            },
            Case {
                input: "AAAA=",
                offset: 5,
            },
            Case {
                input: "AAAA==",
                offset: 6,
            },
            Case {
                input: "AAAAA=",
                offset: 6,
            },
            Case {
                input: "AAAAA==",
                offset: 7,
            },
            Case {
                input: "A=======",
                offset: 1,
            },
            Case {
                input: "AA======",
                offset: -1,
            },
            Case {
                input: "AAA=====",
                offset: 3,
            },
            Case {
                input: "AAAA====",
                offset: -1,
            },
            Case {
                input: "AAAAA===",
                offset: -1,
            },
            Case {
                input: "AAAAAA==",
                offset: 6,
            },
            Case {
                input: "AAAAAAA=",
                offset: -1,
            },
            Case {
                input: "AAAAAAAA",
                offset: -1,
            },
        ];

        for c in cases {
            let mut dbuf = vec![0; STD_ENCODING.decode_len(c.input.len())];
            let x = STD_ENCODING.decode(c.input.as_bytes(), &mut dbuf);
            if c.offset == -1 {
                assert!(x.is_ok());
                continue;
            }

            assert!(x.is_err());
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
        let mut w = STD_ENCODING.encoder(&mut encoded);
        let _ = w.write(&raw).unwrap();
        w.flush().unwrap();
        eprintln!("encoded: {}", encoded.len());

        let mut dst = String::new();
        STD_ENCODING
            .decoder(std::io::Cursor::new(&encoded))
            .read_to_string(&mut dst)
            .unwrap();
        assert_eq!(raw, dst.as_bytes());
    }

    #[test]
    fn test_new_line_characters() {
        let test_string_encode = |expected: &str, examples: &[&str]| {
            for e in examples {
                match STD_ENCODING.decode_to_vec(e.as_bytes()) {
                    Ok(buf) => {
                        assert_eq!(expected.as_bytes(), &buf);
                    }
                    Err(err) => {
                        eprintln!("Decode {} failed: {}", e, err);
                        continue;
                    }
                }
            }
        };

        // Each of these should decode to the string "sure", without errors.
        const EXAMPLES: &[&str] = &[
            "ON2XEZI=",
            "ON2XEZI=\r",
            "ON2XEZI=\n",
            "ON2XEZI=\r\n",
            "ON2XEZ\r\nI=",
            "ON2X\rEZ\nI=",
            "ON2X\nEZ\rI=",
            "ON2XEZ\nI=",
            "ON2XEZI\n=",
        ];
        test_string_encode("sure", EXAMPLES);

        // Each of these should decode to the string "foobar", without errors.
        const EXAMPLES1: &[&str] = &["MZXW6YTBOI======", "MZXW6YTBOI=\r\n====="];
        test_string_encode("foobar", EXAMPLES1);
    }

    #[test]
    fn test_decoder_without_newlines() {
        const ENCODED: &str = r"JRXXEZLNEBUXA43VNUQGI33MN5ZCA43JOQQGC3LFOQWCAY3PNZZWKY3UMV2HK4
RAMFSGS4DJONUWG2LOM4QGK3DJOQWCA43FMQQGI3YKMVUXK43NN5SCA5DFNVYG64RANFXGG2LENFSH
K3TUEB2XIIDMMFRG64TFEBSXIIDEN5WG64TFEBWWCZ3OMEQGC3DJOF2WCLRAKV2CAZLONFWQUYLEEB
WWS3TJNUQHMZLONFQW2LBAOF2WS4ZANZXXG5DSOVSCAZLYMVZGG2LUMF2GS33OEB2WY3DBNVRW6IDM
MFRG64TJOMQG42LTNEQHK5AKMFWGS4LVNFYCAZLYEBSWCIDDN5WW233EN4QGG33OONSXC5LBOQXCAR
DVNFZSAYLVORSSA2LSOVZGKIDEN5WG64RANFXAU4TFOBZGK2DFNZSGK4TJOQQGS3RAOZXWY5LQORQX
IZJAOZSWY2LUEBSXG43FEBRWS3DMOVWSAZDPNRXXEZJAMV2SAZTVM5UWC5BANZ2WY3DBBJYGC4TJMF
2HK4ROEBCXQY3FOB2GK5LSEBZWS3TUEBXWGY3BMVRWC5BAMN2XA2LEMF2GC5BANZXW4IDQOJXWSZDF
NZ2CYIDTOVXHIIDJNYFGG5LMOBQSA4LVNEQG6ZTGNFRWSYJAMRSXGZLSOVXHIIDNN5WGY2LUEBQW42
LNEBUWIIDFON2CA3DBMJXXE5LNFY==
====";

        let encoded_start = ENCODED.replace('\n', "");
        let mut dec = STD_ENCODING.decoder(std::io::Cursor::new(&ENCODED));

        let mut res1 = String::new();
        dec.read_to_string(&mut res1).unwrap();

        let mut dec = STD_ENCODING.decoder(std::io::Cursor::new(&encoded_start));
        let mut res2 = String::new();
        dec.read_to_string(&mut res2).unwrap();
        assert_eq!(res1, res2);
    }

    #[test]
    fn test_with_custom_padding() {
        for case in pairs() {
            let default_padding = STD_ENCODING.encode_to_vec(case.decoded.as_bytes());
            let custom_padding = STD_ENCODING
                .with_padding_unchecked(Some('@'))
                .encode_to_vec(case.decoded.as_bytes());
            let expected = String::from_utf8_lossy(&default_padding).replace('=', "@");

            assert_eq!(expected, String::from_utf8_lossy(&custom_padding));
            assert_eq!(case.encoded.as_bytes(), default_padding);
        }
    }

    #[test]
    fn test_without_padding() {
        for case in pairs() {
            let default_padding = STD_ENCODING.encode_to_vec(case.decoded.as_bytes());
            let custom_padding = STD_ENCODING
                .with_padding_unchecked(None)
                .encode_to_vec(case.decoded.as_bytes());
            let expected = String::from_utf8_lossy(&default_padding)
                .trim_end_matches('=')
                .to_owned();

            assert_eq!(expected, String::from_utf8_lossy(&custom_padding));
            assert_eq!(case.encoded.as_bytes(), default_padding);
        }
    }

    #[test]
    fn test_decode_with_padding() {
        let encodings = [
            STD_ENCODING,
            STD_ENCODING.with_padding_unchecked(Some('-')),
            STD_ENCODING.with_padding_unchecked(None),
        ];

        for enc in encodings {
            for case in pairs() {
                let input = case.decoded.as_bytes();
                let encoded = enc.encode_to_vec(input);

                let decoded = enc.decode_to_vec(&encoded).unwrap();
                assert_eq!(input, &decoded);
            }
        }
    }

    #[test]
    fn test_decode_with_wrong_padding() {
        let encoded = STD_ENCODING.encode_to_vec(b"foobar");

        let _ = STD_ENCODING
            .with_padding_unchecked(Some('-'))
            .decode_to_vec(&encoded)
            .unwrap_err();

        let _ = STD_ENCODING
            .with_padding_unchecked(None)
            .decode_to_vec(&encoded)
            .unwrap_err();
    }

    // TODO: implement this
    #[test]
    fn test_buffered_decoding_padding() {}

    // TODO: implement this
    #[test]
    fn test_buffered_decoding_same_error() {}

    #[test]
    fn test_encoded_decoded_len() {
        struct TestCase {
            in_: usize,
            want_enc: usize,
            want_dec: usize,
        }

        struct Test {
            enc: Base32,
            cases: &'static [TestCase],
        }

        let tests = [
            Test {
                enc: STD_ENCODING,
                cases: &[
                    TestCase {
                        in_: 0,
                        want_enc: 0,
                        want_dec: 0,
                    },
                    TestCase {
                        in_: 1,
                        want_enc: 8,
                        want_dec: 5,
                    },
                    TestCase {
                        in_: 5,
                        want_enc: 8,
                        want_dec: 5,
                    },
                    TestCase {
                        in_: 6,
                        want_enc: 16,
                        want_dec: 10,
                    },
                    TestCase {
                        in_: 10,
                        want_enc: 16,
                        want_dec: 10,
                    },
                ],
            },
            Test {
                enc: STD_ENCODING.with_padding_unchecked(None),
                cases: &[
                    TestCase {
                        in_: 0,
                        want_enc: 0,
                        want_dec: 0,
                    },
                    TestCase {
                        in_: 1,
                        want_enc: 2,
                        want_dec: 1,
                    },
                    TestCase {
                        in_: 2,
                        want_enc: 4,
                        want_dec: 2,
                    },
                    TestCase {
                        in_: 5,
                        want_enc: 8,
                        want_dec: 5,
                    },
                    TestCase {
                        in_: 6,
                        want_enc: 10,
                        want_dec: 6,
                    },
                    TestCase {
                        in_: 7,
                        want_enc: 12,
                        want_dec: 7,
                    },
                    TestCase {
                        in_: 10,
                        want_enc: 16,
                        want_dec: 10,
                    },
                    TestCase {
                        in_: 11,
                        want_enc: 18,
                        want_dec: 11,
                    },
                ],
            },
        ];

        let data = vec![b'x'; 100];

        for test in tests {
            for tc in test.cases {
                let enc_len = test.enc.encoded_len(tc.in_);
                let dec_len = test.enc.decode_len(enc_len);
                let enc = test.enc.encode_to_vec(&data[..tc.in_]);
                assert_eq!(enc_len, enc.len());
                assert_eq!(enc_len, tc.want_enc);
                assert_eq!(dec_len, tc.want_dec);
            }
        }
    }

    #[test]
    fn test_without_padding_close() {
        let encodings = [STD_ENCODING, STD_ENCODING.with_padding_unchecked(None)];

        for enc in encodings {
            for case in pairs() {
                let mut buf = Vec::new();
                let mut encoder = enc.encoder(&mut buf);
                let _ = encoder.write(case.decoded.as_bytes()).unwrap();
                encoder.flush().unwrap();

                let mut expected = case.encoded;
                if enc.pad_char.is_none() {
                    expected = expected.replace('=', "");
                }
                assert_eq!(expected.as_bytes(), &buf);
            }
        }
    }

    #[test]
    fn test_decode_read_all() {
        let encodings = [STD_ENCODING, STD_ENCODING.with_padding_unchecked(None)];

        for pair in pairs() {
            for enc in encodings {
                let mut encoded = pair.encoded.clone();
                if enc.pad_char.is_none() {
                    encoded = encoded.replace('=', "");
                }

                let mut dec = String::new();
                enc.decoder(std::io::Cursor::new(encoded))
                    .read_to_string(&mut dec)
                    .unwrap();

                assert_eq!(pair.decoded, dec);
            }
        }
    }

    #[test]
    fn test_decode_small_buffer() {
        let encodings = [STD_ENCODING, STD_ENCODING.with_padding_unchecked(None)];

        for buffer_size in 1..200 {
            for pair in pairs() {
                for enc in encodings {
                    let mut encoded = pair.encoded.clone();
                    if enc.pad_char.is_none() {
                        encoded = encoded.replace('=', "");
                    }

                    let mut decoder = enc.decoder(std::io::Cursor::new(encoded));
                    let mut all_read = Vec::new();
                    loop {
                        let mut buf = vec![0; buffer_size];
                        let n = decoder.read(&mut buf).unwrap();
                        if n == 0 {
                            break;
                        }
                        all_read.extend_from_slice(&buf[..n]);
                    }

                    assert_eq!(pair.decoded.as_bytes(), all_read);
                }
            }
        }
    }

    // struct BadReader {
    //     data: Vec<u8>,
    //     called: usize,
    //     errs: Vec<Option<std::io::Error>>,
    //     limit: usize,
    // }

    // impl std::io::Read for BadReader {
    //     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    //         let mut lim = buf.len();
    //         if self.limit != 0 && self.limit < lim {
    //             lim = self.limit;
    //         }
    //         if self.data.len() < lim {
    //             lim = self.data.len();
    //         }
    //         let tmp = buf[..lim].to_vec();
    //         for i in tmp {
    //             buf[i as usize] = self.data[i as usize];
    //         }

    //         self.data.drain(..lim);
    //         let mut err = Some(std::io::Error::new(
    //             std::io::ErrorKind::UnexpectedEof,
    //             "bad reader eof",
    //         ));
    //         if self.called < self.errs.len() {
    //             err = self.errs[self.called]
    //                 .as_ref()
    //                 .map(|e| std::io::Error::new(e.kind(), "bad reader"));
    //         }
    //         self.called += 1;
    //         match err {
    //             Some(e) => Err(e),
    //             None => Ok(lim),
    //         }
    //     }
    // }

    // #[test]
    // fn test_decoder_error() {
    //     for err in [
    //         Some(std::io::Error::new(
    //             std::io::ErrorKind::UnexpectedEof,
    //             "test decoder error",
    //         )),
    //         None,
    //     ] {
    //         let input = "MZXW6YTb".to_string();
    //         let mut dbuf = vec![0; STD_ENCODING.decode_len(input.len())];
    //         let mut br = BadReader {
    //             data: input.as_bytes().to_vec(),
    //             called: 0,
    //             errs: vec![err],
    //             limit: 0,
    //         };
    //         let mut decoder = STD_ENCODING.decoder(&mut br);
    //         let n = decoder.read(&mut dbuf).unwrap();
    //         assert_eq!(n, 0);
    //     }
    // }
}
