

macro_rules! assign_32 {
    ($src: ident, $dst: ident) => {
        $dst[0] = $src[0];
        $dst[1] = $src[1];
        $dst[2] = $src[2];
        $dst[3] = $src[3];
        $dst[4] = $src[4];
        $dst[5] = $src[5];
        $dst[6] = $src[6];
        $dst[7] = $src[7];
        $dst[8] = $src[8];
        $dst[9] = $src[9];
        $dst[10] = $src[10];
        $dst[11] = $src[11];
        $dst[12] = $src[12];
        $dst[13] = $src[13];
        $dst[14] = $src[14];
        $dst[15] = $src[15];
        $dst[16] = $src[16];
        $dst[17] = $src[17];
        $dst[18] = $src[18];
        $dst[19] = $src[19];
        $dst[20] = $src[20];
        $dst[21] = $src[21];
        $dst[22] = $src[22];
        $dst[23] = $src[23];
        $dst[24] = $src[24];
        $dst[25] = $src[25];
        $dst[26] = $src[26];
        $dst[27] = $src[27];
        $dst[28] = $src[28];
        $dst[29] = $src[29];
        $dst[30] = $src[30];
        $dst[31] = $src[31];
    };
}

macro_rules! check_32 {
    ($arr: ident, $char: ident, $invalid: expr) => {
        if $char == $arr[0] as char {
            $invalid;
        }

        if $char == $arr[1] as char {
            $invalid;
        }

        if $char == $arr[2] as char {
            $invalid;
        }

        if $char == $arr[3] as char {
            $invalid;
        }

        if $char == $arr[4] as char {
            $invalid;
        }

        if $char == $arr[5] as char {
            $invalid;
        }

        if $char == $arr[6] as char {
            $invalid;
        }

        if $char == $arr[7] as char {
            $invalid;
        }

        if $char == $arr[8] as char {
            $invalid;
        }

        if $char == $arr[9] as char {
            $invalid;
        }

        if $char == $arr[10] as char {
            $invalid;
        }

        if $char == $arr[11] as char {
            $invalid;
        }

        if $char == $arr[12] as char {
            $invalid;
        }

        if $char == $arr[13] as char {
            $invalid;
        }

        if $char == $arr[14] as char {
            $invalid;
        }

        if $char == $arr[15] as char {
            $invalid;
        }

        if $char == $arr[16] as char {
            $invalid;
        }

        if $char == $arr[17] as char {
            $invalid;
        }

        if $char == $arr[18] as char {
            $invalid;
        }

        if $char == $arr[19] as char {
            $invalid;
        }

        if $char == $arr[20] as char {
            $invalid;
        }

        if $char == $arr[21] as char {
            $invalid;
        }

        if $char == $arr[22] as char {
            $invalid;
        }

        if $char == $arr[23] as char {
            $invalid;
        }

        if $char == $arr[24] as char {
            $invalid;
        }

        if $char == $arr[25] as char {
            $invalid;
        }

        if $char == $arr[26] as char {
            $invalid;
        }

        if $char == $arr[27] as char {
            $invalid;
        }

        if $char == $arr[28] as char {
            $invalid;
        }

        if $char == $arr[29] as char {
            $invalid;
        }

        if $char == $arr[30] as char {
            $invalid;
        }

        if $char == $arr[31] as char {
            $invalid;
        }
    };
}

const BASE: usize = 32;

const ENCODE_STD: [u8; BASE] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const ENCODE_HEX: [u8; BASE] = *b"0123456789ABCDEFGHIJKLMNOPQRSTUV";

/// Error
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    /// Invalid encoder
    InvalidEncoder,

    /// Invalid padding character
    InvalidPadding,

    /// Padding character is contained in the alphabet
    PaddingContainedInAlphabet(char),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::InvalidEncoder => write!(f, "Encoding alphabet must be 32 bytes long"),
            Error::InvalidPadding => write!(f, "Invalid padding character"),
            Error::PaddingContainedInAlphabet(ch) => write!(f, "Padding character '{ch}' is contained in the alphabet"),
        }
    }
}

#[cfg(feature="std")]
impl std::error::Error for Error {}

/// No padding
pub const NO_PADDING: Option<char> = None;

/// Standard padding
pub const STD_PADDING: Option<char> = Some('=');



/// The standard base32 encoding, as defined in
/// RFC 4648.
pub const STD_ENCODING: Encoding = Encoding::new(ENCODE_STD);

/// The `Extended Hex Alphabet` defined in RFC 4648.
// It is typically used in DNS.
pub const HEX_ENCODING: Encoding = Encoding::new(ENCODE_HEX);

/// An Encoding is a radix 32 encoding/decoding scheme, defined by a
/// 32-character alphabet. The most common is the "base32" encoding
/// introduced for SASL GSSAPI and standardized in RFC 4648.
/// The alternate "base32hex" encoding is used in DNSSEC.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Encoding {
    encode: [u8; BASE],
    decode_map: [u8; 256],
    pad_char: Option<char>,
}

impl Encoding {
    /// Returns a new Encoding defined by the given alphabet,
    /// which must be a 32-byte slice.
    #[inline]
    pub const fn new(encoder: [u8; BASE]) -> Self {
        let mut decode_map = [0xff; 256];
        assign_32!(encoder, decode_map);

        Encoding {
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
    pub const fn with_padding(encoder: [u8; 32], padding: Option<char>) -> Result<Self, Error> {
        let mut decode_map = [0xff; 256];
        assign_32!(encoder, decode_map);

        if let Some(padding) = padding {
            if padding == '\r' || padding == '\n' || padding > (0xff as char) {
                return Err(Error::InvalidPadding);
            }
    
            check_32!(encoder, padding, return Err(Error::PaddingContainedInAlphabet(padding)));
            return Ok(Encoding {
                encode: encoder,
                decode_map,
                pad_char: Some(padding),
            });
        }
        
        Ok(Encoding {
            encode: encoder,
            decode_map,
            pad_char: None,
        })
        
    }

    /// Creates a new encoding identical to enc except
    /// with a specified padding character.
    /// The padding character must not be '\r' or '\n', must not
    /// be contained in the encoding's alphabet and must be a rune equal or
    /// below '\xff'.
    /// 
    /// # Panic
    /// - `padding` must not be '\r' or '\n', must not be contained in the alphabet and must be a rune equal or
    /// below '\xff'.
    /// - `encoder` must be a 32-byte slice
    #[inline]
    pub const fn with_padding_unchecked(encoder: [u8; BASE], padding: Option<char>) -> Self {
        let mut decode_map = [0xff; 256];
        assign_32!(encoder, decode_map);

        if let Some(padding) = padding {
            if padding == '\r' || padding == '\n' || padding > (0xff as char) {
                panic!("Padding must not be '\\r' or '\\n' and must be a rune equal or below '\\xff'");
            }
    
            check_32!(encoder, padding, panic!("Padding character is contained in the alphabet"));

            return Encoding {
                encode: encoder,
                decode_map,
                pad_char: Some(padding),
            };
        }

        Encoding {
            encode: encoder,
            decode_map,
            pad_char: None,
        }
    }

    /// Returns the length in bytes of the base32 encoding
    /// of source slice.
    #[inline]
    pub const fn encoded_len(&self, src: &[u8]) -> usize {
        let n = src.len();
        if self.pad_char.is_none() {
            return (n*8 + 4) / 5;
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
                },
                3 => {
                    b[4] |= (src[2] << 1) & 0x1F;
			        b[3] = (src[2] >> 4) & 0x1F;
                    b[3] |= (src[1] << 4) & 0x1F;
			        b[2] = (src[1] >> 1) & 0x1F;
			        b[1] = (src[1] >> 6) & 0x1F;
                    b[1] |= (src[0] << 2) & 0x1F;
			        b[0] = src[0] >> 3;
                },
                2 => {
                    b[3] |= (src[1] << 4) & 0x1F;
			        b[2] = (src[1] >> 1) & 0x1F;
			        b[1] = (src[1] >> 6) & 0x1F;
                    b[1] |= (src[0] << 2) & 0x1F;
			        b[0] = src[0] >> 3;
                },
                1 => {
                    b[1] |= (src[0] << 2) & 0x1F;
			        b[0] = src[0] >> 3;
                },
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
        let mut dst = alloc::vec![0; self.encoded_len(src)];
        self.encode(src, &mut dst);
        dst
    }

    /// Decodes src using the encoding enc. It writes at most
    /// DecodedLen(len(src)) bytes to dst and returns the number of bytes
    /// written. If src contains invalid base32 data, it will return the
    /// number of bytes successfully written and DecodeError.
    /// 
    /// See `decode_without_newline` if you want to ignore the newline characters
    #[inline]
    pub fn decode(&self, src: &[u8], dst: &mut [u8]) -> Result<usize, DecodeError> {
        self.decode_in(src, dst).map(|(n, _)| n)
    }

    /// Returns the bytes represented by the base32 slice.
    #[inline]
    #[cfg(feature = "alloc")]
    pub fn decode_to_vec(&self, src: &[u8]) -> Result<alloc::vec::Vec<u8>, DecodeError> {
        let mut dst = alloc::vec![0; self.decode_len(src)];
        self.decode_in(src, &mut dst).map(|(n, _)| {
            dst.truncate(n);
            dst
        })
    }

    /// Decodes src using the encoding enc. It writes at most
    /// DecodedLen(len(src)) bytes to dst and returns the number of bytes
    /// written. If src contains invalid base32 data, it will return the
    /// number of bytes successfully written and DecodeError.
    /// New line characters (`\r` and `\n`) are ignored.
    #[inline]
    #[cfg(feature = "alloc")]
    pub fn decode_without_newline(&self, src: &[u8], dst: &mut [u8]) -> Result<usize, DecodeError> {
        let mut buf = alloc::vec![0; src.len()];
        let l = strip_new_lines(src, &mut buf);
        self.decode_in(&buf[..l], dst).map(|(n, _)| n) 
    }


    /// Returns the bytes represented by the base32 slice (ignore newline characters (`\r` and `\n`)).
    #[inline]
    #[cfg(feature = "alloc")]
    pub fn decode_without_newline_to_vec(&self, src: &[u8]) -> Result<alloc::vec::Vec<u8>, DecodeError> {
        let mut buf = src.to_vec();
        let l = strip_new_lines(src, &mut buf);
        let ptr = buf.as_mut_ptr();
        let len = buf.len();
        unsafe { 
            self.decode_without_newline(
                &buf[..l], 
                core::slice::from_raw_parts_mut(ptr, len)).map(|n| {
                buf.truncate(n);
                buf
            }) 
        }
    }

    /// Returns the maximum length in bytes of the decoded data
    /// corresponding to n bytes of base32-encoded data.
    #[inline]
    pub const fn decode_len(&self, src: &[u8]) -> usize {
        let n = src.len();
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
                            k+=1;
                        }

                        (dlen, end) = (j, true);

                        // 7, 5 and 2 are not valid padding lengths, and so 1, 3 and 6 are not
				        // valid dlen values. See RFC 4648 Section 6 "Base 32 Encoding" listing
				        // the five valid padding lengths, and Section 9 "Illustrations and
				        // Examples" for an illustration for how the 1st, 3rd and 6th base32
				        // src bytes do not yield enough information to decode a dst byte.
                        if dlen == 1 || dlen == 3 || dlen == 6 {
                            return Err(DecodeError(olen - src.len() - 1));
                        }
                        break;
                    },
                    _ => {
                        dbuf[j] = self.decode_map[in_ as usize];
                        if dbuf[j] == 0xFF {
                            return Err(DecodeError(olen - src.len() - 1));
                        }
                        j += 1;
                    },
                }

                
            }
        
            // Pack 8x 5-bit source blocks into 5 byte destination
		    // quantum
            match dlen {
                8 => {
                    dst[dsti+4] = dbuf[6] << 5 | dbuf[7];
                    n += 1;
                    dst[dsti+3] = dbuf[4] << 7 | dbuf[5] << 2 | dbuf[6] >> 3;
                    n += 1;
                    dst[dsti+2] = dbuf[3] << 4 | dbuf[4] >> 1;
                    n += 1;
                    dst[dsti+1] = dbuf[1] << 6 | dbuf[2] << 1 | dbuf[3] >> 4;
                    n += 1;
                    dst[dsti] = dbuf[0] << 3 | dbuf[1] >> 2;
                    n += 1;
                },
                7 => {
                    dst[dsti+3] = dbuf[4] << 7 | dbuf[5] << 2 | dbuf[6] >> 3;
                    n += 1;
                    dst[dsti+2] = dbuf[3] << 4 | dbuf[4] >> 1;
                    n += 1;
                    dst[dsti+1] = dbuf[1] << 6 | dbuf[2] << 1 | dbuf[3] >> 4;
                    n += 1;
                    dst[dsti] = dbuf[0] << 3 | dbuf[1] >> 2;
                    n += 1;
                },
                5 => {
                    dst[dsti+2] = dbuf[3] << 4 | dbuf[4] >> 1;
                    n += 1;
                    dst[dsti+1] = dbuf[1] << 6 | dbuf[2] << 1 | dbuf[3] >> 4;
                    n += 1;
                    dst[dsti] = dbuf[0] << 3 | dbuf[1] >> 2;
                    n += 1; 
                },
                4 => {
                    dst[dsti+1] = dbuf[1] << 6 | dbuf[2] << 1 | dbuf[3] >> 4;
                    n += 1;
                    dst[dsti] = dbuf[0] << 3 | dbuf[1] >> 2;
                    n += 1;
                },
                2 => {
                    dst[dsti] = dbuf[0] << 3 | dbuf[1] >> 2;
                    n += 1; 
                },
                _ => {},
            }
            dsti += 5;
        }
        Ok((n, end))
    }
}


/// Removes newline characters and returns the number
/// of non-newline characters copied to dst.
fn strip_new_lines(src: &[u8], dst: &mut [u8]) -> usize {
    let mut offset = 0;
    for b in src {
        if b.eq(&b'\r') || b.eq(&b'\n') {
            continue;
        }
        dst[offset] = *b;
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
