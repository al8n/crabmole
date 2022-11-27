macro_rules! assign_64 {
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
        $dst[32] = $src[32];
        $dst[33] = $src[33];
        $dst[34] = $src[34];
        $dst[35] = $src[35];
        $dst[36] = $src[36];
        $dst[37] = $src[37];
        $dst[38] = $src[38];
        $dst[39] = $src[39];
        $dst[40] = $src[40];
        $dst[41] = $src[41];
        $dst[42] = $src[42];
        $dst[43] = $src[43];
        $dst[44] = $src[44];
        $dst[45] = $src[45];
        $dst[46] = $src[46];
        $dst[47] = $src[47];
        $dst[48] = $src[48];
        $dst[49] = $src[49];
        $dst[50] = $src[50];
        $dst[51] = $src[51];
        $dst[52] = $src[52];
        $dst[53] = $src[53];
        $dst[54] = $src[54];
        $dst[55] = $src[55];
        $dst[56] = $src[56];
        $dst[57] = $src[57];
        $dst[58] = $src[58];
        $dst[59] = $src[59];
        $dst[60] = $src[60];
        $dst[61] = $src[61];
        $dst[62] = $src[62];
        $dst[63] = $src[63];
    };
}

macro_rules! check_64 {
    ($encoder: ident, $invalid: expr) => {{
        if $encoder[0] == b'\n' || $encoder[0] == b'\r' {
            $invalid;
        }

        if $encoder[1] == b'\n' || $encoder[1] == b'\r' {
            $invalid;
        }

        if $encoder[2] == b'\n' || $encoder[2] == b'\r' {
            $invalid;
        }

        if $encoder[3] == b'\n' || $encoder[3] == b'\r' {
            $invalid;
        }

        if $encoder[4] == b'\n' || $encoder[4] == b'\r' {
            $invalid;
        }

        if $encoder[5] == b'\n' || $encoder[5] == b'\r' {
            $invalid;
        }

        if $encoder[6] == b'\n' || $encoder[6] == b'\r' {
            $invalid;
        }

        if $encoder[7] == b'\n' || $encoder[7] == b'\r' {
            $invalid;
        }

        if $encoder[8] == b'\n' || $encoder[8] == b'\r' {
            $invalid;
        }

        if $encoder[9] == b'\n' || $encoder[9] == b'\r' {
            $invalid;
        }

        if $encoder[10] == b'\n' || $encoder[10] == b'\r' {
            $invalid;
        }

        if $encoder[11] == b'\n' || $encoder[11] == b'\r' {
            $invalid;
        }

        if $encoder[12] == b'\n' || $encoder[12] == b'\r' {
            $invalid;
        }

        if $encoder[13] == b'\n' || $encoder[13] == b'\r' {
            $invalid;
        }

        if $encoder[14] == b'\n' || $encoder[14] == b'\r' {
            $invalid;
        }

        if $encoder[15] == b'\n' || $encoder[15] == b'\r' {
            $invalid;
        }

        if $encoder[16] == b'\n' || $encoder[16] == b'\r' {
            $invalid;
        }

        if $encoder[17] == b'\n' || $encoder[17] == b'\r' {
            $invalid;
        }

        if $encoder[18] == b'\n' || $encoder[18] == b'\r' {
            $invalid;
        }

        if $encoder[19] == b'\n' || $encoder[19] == b'\r' {
            $invalid;
        }

        if $encoder[20] == b'\n' || $encoder[20] == b'\r' {
            $invalid;
        }

        if $encoder[21] == b'\n' || $encoder[21] == b'\r' {
            $invalid;
        }

        if $encoder[22] == b'\n' || $encoder[22] == b'\r' {
            $invalid;
        }

        if $encoder[23] == b'\n' || $encoder[23] == b'\r' {
            $invalid;
        }

        if $encoder[24] == b'\n' || $encoder[24] == b'\r' {
            $invalid;
        }

        if $encoder[25] == b'\n' || $encoder[25] == b'\r' {
            $invalid;
        }

        if $encoder[26] == b'\n' || $encoder[26] == b'\r' {
            $invalid;
        }

        if $encoder[27] == b'\n' || $encoder[27] == b'\r' {
            $invalid;
        }

        if $encoder[28] == b'\n' || $encoder[28] == b'\r' {
            $invalid;
        }

        if $encoder[29] == b'\n' || $encoder[29] == b'\r' {
            $invalid;
        }

        if $encoder[30] == b'\n' || $encoder[30] == b'\r' {
            $invalid;
        }

        if $encoder[31] == b'\n' || $encoder[31] == b'\r' {
            $invalid;
        }

        if $encoder[32] == b'\n' || $encoder[32] == b'\r' {
            $invalid;
        }

        if $encoder[33] == b'\n' || $encoder[33] == b'\r' {
            $invalid;
        }

        if $encoder[34] == b'\n' || $encoder[34] == b'\r' {
            $invalid;
        }

        if $encoder[35] == b'\n' || $encoder[35] == b'\r' {
            $invalid;
        }

        if $encoder[36] == b'\n' || $encoder[36] == b'\r' {
            $invalid;
        }

        if $encoder[37] == b'\n' || $encoder[37] == b'\r' {
            $invalid;
        }

        if $encoder[38] == b'\n' || $encoder[38] == b'\r' {
            $invalid;
        }

        if $encoder[39] == b'\n' || $encoder[39] == b'\r' {
            $invalid;
        }

        if $encoder[40] == b'\n' || $encoder[40] == b'\r' {
            $invalid;
        }

        if $encoder[41] == b'\n' || $encoder[41] == b'\r' {
            $invalid;
        }

        if $encoder[42] == b'\n' || $encoder[42] == b'\r' {
            $invalid;
        }

        if $encoder[43] == b'\n' || $encoder[43] == b'\r' {
            $invalid;
        }

        if $encoder[44] == b'\n' || $encoder[44] == b'\r' {
            $invalid;
        }

        if $encoder[45] == b'\n' || $encoder[45] == b'\r' {
            $invalid;
        }

        if $encoder[46] == b'\n' || $encoder[46] == b'\r' {
            $invalid;
        }

        if $encoder[47] == b'\n' || $encoder[47] == b'\r' {
            $invalid;
        }

        if $encoder[48] == b'\n' || $encoder[48] == b'\r' {
            $invalid;
        }

        if $encoder[49] == b'\n' || $encoder[49] == b'\r' {
            $invalid;
        }

        if $encoder[50] == b'\n' || $encoder[50] == b'\r' {
            $invalid;
        }

        if $encoder[51] == b'\n' || $encoder[51] == b'\r' {
            $invalid;
        }

        if $encoder[52] == b'\n' || $encoder[52] == b'\r' {
            $invalid;
        }

        if $encoder[53] == b'\n' || $encoder[53] == b'\r' {
            $invalid;
        }

        if $encoder[54] == b'\n' || $encoder[54] == b'\r' {
            $invalid;
        }

        if $encoder[55] == b'\n' || $encoder[55] == b'\r' {
            $invalid;
        }

        if $encoder[56] == b'\n' || $encoder[56] == b'\r' {
            $invalid;
        }

        if $encoder[57] == b'\n' || $encoder[57] == b'\r' {
            $invalid;
        }

        if $encoder[58] == b'\n' || $encoder[58] == b'\r' {
            $invalid;
        }

        if $encoder[59] == b'\n' || $encoder[59] == b'\r' {
            $invalid;
        }

        if $encoder[60] == b'\n' || $encoder[60] == b'\r' {
            $invalid;
        }

        if $encoder[61] == b'\n' || $encoder[61] == b'\r' {
            $invalid;
        }

        if $encoder[62] == b'\n' || $encoder[62] == b'\r' {
            $invalid;
        }

        if $encoder[63] == b'\n' || $encoder[63] == b'\r' {
            $invalid;
        }
    }};
    ($encoder: ident, $ch: ident, $invalid: expr) => {{
        if $encoder[0] == b'\n' || $encoder[0] == b'\r' || $encoder[0] == $ch as u8 {
            $invalid;
        }

        if $encoder[1] == b'\n' || $encoder[1] == b'\r' || $encoder[1] == $ch as u8 {
            $invalid;
        }

        if $encoder[2] == b'\n' || $encoder[2] == b'\r' || $encoder[2] == $ch as u8 {
            $invalid;
        }

        if $encoder[3] == b'\n' || $encoder[3] == b'\r' || $encoder[3] == $ch as u8 {
            $invalid;
        }

        if $encoder[4] == b'\n' || $encoder[4] == b'\r' || $encoder[4] == $ch as u8 {
            $invalid;
        }

        if $encoder[5] == b'\n' || $encoder[5] == b'\r' || $encoder[5] == $ch as u8 {
            $invalid;
        }

        if $encoder[6] == b'\n' || $encoder[6] == b'\r' || $encoder[6] == $ch as u8 {
            $invalid;
        }

        if $encoder[7] == b'\n' || $encoder[7] == b'\r' || $encoder[7] == $ch as u8 {
            $invalid;
        }

        if $encoder[8] == b'\n' || $encoder[8] == b'\r' || $encoder[8] == $ch as u8 {
            $invalid;
        }

        if $encoder[9] == b'\n' || $encoder[9] == b'\r' || $encoder[9] == $ch as u8 {
            $invalid;
        }

        if $encoder[10] == b'\n' || $encoder[10] == b'\r' || $encoder[10] == $ch as u8 {
            $invalid;
        }

        if $encoder[11] == b'\n' || $encoder[11] == b'\r' || $encoder[11] == $ch as u8 {
            $invalid;
        }

        if $encoder[12] == b'\n' || $encoder[12] == b'\r' || $encoder[12] == $ch as u8 {
            $invalid;
        }

        if $encoder[13] == b'\n' || $encoder[13] == b'\r' || $encoder[13] == $ch as u8 {
            $invalid;
        }

        if $encoder[14] == b'\n' || $encoder[14] == b'\r' || $encoder[14] == $ch as u8 {
            $invalid;
        }

        if $encoder[15] == b'\n' || $encoder[15] == b'\r' || $encoder[15] == $ch as u8 {
            $invalid;
        }

        if $encoder[16] == b'\n' || $encoder[16] == b'\r' || $encoder[16] == $ch as u8 {
            $invalid;
        }

        if $encoder[17] == b'\n' || $encoder[17] == b'\r' || $encoder[17] == $ch as u8 {
            $invalid;
        }

        if $encoder[18] == b'\n' || $encoder[18] == b'\r' || $encoder[18] == $ch as u8 {
            $invalid;
        }

        if $encoder[19] == b'\n' || $encoder[19] == b'\r' || $encoder[19] == $ch as u8 {
            $invalid;
        }

        if $encoder[20] == b'\n' || $encoder[20] == b'\r' || $encoder[20] == $ch as u8 {
            $invalid;
        }

        if $encoder[21] == b'\n' || $encoder[21] == b'\r' || $encoder[21] == $ch as u8 {
            $invalid;
        }

        if $encoder[22] == b'\n' || $encoder[22] == b'\r' || $encoder[22] == $ch as u8 {
            $invalid;
        }

        if $encoder[23] == b'\n' || $encoder[23] == b'\r' || $encoder[23] == $ch as u8 {
            $invalid;
        }

        if $encoder[24] == b'\n' || $encoder[24] == b'\r' || $encoder[24] == $ch as u8 {
            $invalid;
        }

        if $encoder[25] == b'\n' || $encoder[25] == b'\r' || $encoder[25] == $ch as u8 {
            $invalid;
        }

        if $encoder[26] == b'\n' || $encoder[26] == b'\r' || $encoder[26] == $ch as u8 {
            $invalid;
        }

        if $encoder[27] == b'\n' || $encoder[27] == b'\r' || $encoder[27] == $ch as u8 {
            $invalid;
        }

        if $encoder[28] == b'\n' || $encoder[28] == b'\r' || $encoder[28] == $ch as u8 {
            $invalid;
        }

        if $encoder[29] == b'\n' || $encoder[29] == b'\r' || $encoder[29] == $ch as u8 {
            $invalid;
        }

        if $encoder[30] == b'\n' || $encoder[30] == b'\r' || $encoder[30] == $ch as u8 {
            $invalid;
        }

        if $encoder[31] == b'\n' || $encoder[31] == b'\r' || $encoder[31] == $ch as u8 {
            $invalid;
        }

        if $encoder[32] == b'\n' || $encoder[32] == b'\r' || $encoder[32] == $ch as u8 {
            $invalid;
        }

        if $encoder[33] == b'\n' || $encoder[33] == b'\r' || $encoder[33] == $ch as u8 {
            $invalid;
        }

        if $encoder[34] == b'\n' || $encoder[34] == b'\r' || $encoder[34] == $ch as u8 {
            $invalid;
        }

        if $encoder[35] == b'\n' || $encoder[35] == b'\r' || $encoder[35] == $ch as u8 {
            $invalid;
        }

        if $encoder[36] == b'\n' || $encoder[36] == b'\r' || $encoder[36] == $ch as u8 {
            $invalid;
        }

        if $encoder[37] == b'\n' || $encoder[37] == b'\r' || $encoder[37] == $ch as u8 {
            $invalid;
        }

        if $encoder[38] == b'\n' || $encoder[38] == b'\r' || $encoder[38] == $ch as u8 {
            $invalid;
        }

        if $encoder[39] == b'\n' || $encoder[39] == b'\r' || $encoder[39] == $ch as u8 {
            $invalid;
        }

        if $encoder[40] == b'\n' || $encoder[40] == b'\r' || $encoder[40] == $ch as u8 {
            $invalid;
        }

        if $encoder[41] == b'\n' || $encoder[41] == b'\r' || $encoder[41] == $ch as u8 {
            $invalid;
        }

        if $encoder[42] == b'\n' || $encoder[42] == b'\r' || $encoder[42] == $ch as u8 {
            $invalid;
        }

        if $encoder[43] == b'\n' || $encoder[43] == b'\r' || $encoder[43] == $ch as u8 {
            $invalid;
        }

        if $encoder[44] == b'\n' || $encoder[44] == b'\r' || $encoder[44] == $ch as u8 {
            $invalid;
        }

        if $encoder[45] == b'\n' || $encoder[45] == b'\r' || $encoder[45] == $ch as u8 {
            $invalid;
        }

        if $encoder[46] == b'\n' || $encoder[46] == b'\r' || $encoder[46] == $ch as u8 {
            $invalid;
        }

        if $encoder[47] == b'\n' || $encoder[47] == b'\r' || $encoder[47] == $ch as u8 {
            $invalid;
        }

        if $encoder[48] == b'\n' || $encoder[48] == b'\r' || $encoder[48] == $ch as u8 {
            $invalid;
        }

        if $encoder[49] == b'\n' || $encoder[49] == b'\r' || $encoder[49] == $ch as u8 {
            $invalid;
        }

        if $encoder[50] == b'\n' || $encoder[50] == b'\r' || $encoder[50] == $ch as u8 {
            $invalid;
        }

        if $encoder[51] == b'\n' || $encoder[51] == b'\r' || $encoder[51] == $ch as u8 {
            $invalid;
        }

        if $encoder[52] == b'\n' || $encoder[52] == b'\r' || $encoder[52] == $ch as u8 {
            $invalid;
        }

        if $encoder[53] == b'\n' || $encoder[53] == b'\r' || $encoder[53] == $ch as u8 {
            $invalid;
        }

        if $encoder[54] == b'\n' || $encoder[54] == b'\r' || $encoder[54] == $ch as u8 {
            $invalid;
        }

        if $encoder[55] == b'\n' || $encoder[55] == b'\r' || $encoder[55] == $ch as u8 {
            $invalid;
        }

        if $encoder[56] == b'\n' || $encoder[56] == b'\r' || $encoder[56] == $ch as u8 {
            $invalid;
        }

        if $encoder[57] == b'\n' || $encoder[57] == b'\r' || $encoder[57] == $ch as u8 {
            $invalid;
        }

        if $encoder[58] == b'\n' || $encoder[58] == b'\r' || $encoder[58] == $ch as u8 {
            $invalid;
        }

        if $encoder[59] == b'\n' || $encoder[59] == b'\r' || $encoder[59] == $ch as u8 {
            $invalid;
        }

        if $encoder[60] == b'\n' || $encoder[60] == b'\r' || $encoder[60] == $ch as u8 {
            $invalid;
        }

        if $encoder[61] == b'\n' || $encoder[61] == b'\r' || $encoder[61] == $ch as u8 {
            $invalid;
        }

        if $encoder[62] == b'\n' || $encoder[62] == b'\r' || $encoder[62] == $ch as u8 {
            $invalid;
        }

        if $encoder[63] == b'\n' || $encoder[63] == b'\r' || $encoder[63] == $ch as u8 {
            $invalid;
        }
    }};
}

macro_rules! assign_decode_map {
    ($decode_map: ident, $encoder: ident) => {
        $decode_map[$encoder[0] as usize] = 0;
        $decode_map[$encoder[1] as usize] = 1;
        $decode_map[$encoder[2] as usize] = 2;
        $decode_map[$encoder[3] as usize] = 3;
        $decode_map[$encoder[4] as usize] = 4;
        $decode_map[$encoder[5] as usize] = 5;
        $decode_map[$encoder[6] as usize] = 6;
        $decode_map[$encoder[7] as usize] = 7;
        $decode_map[$encoder[8] as usize] = 8;
        $decode_map[$encoder[9] as usize] = 9;
        $decode_map[$encoder[10] as usize] = 10;
        $decode_map[$encoder[11] as usize] = 11;
        $decode_map[$encoder[12] as usize] = 12;
        $decode_map[$encoder[13] as usize] = 13;
        $decode_map[$encoder[14] as usize] = 14;
        $decode_map[$encoder[15] as usize] = 15;
        $decode_map[$encoder[16] as usize] = 16;
        $decode_map[$encoder[17] as usize] = 17;
        $decode_map[$encoder[18] as usize] = 18;
        $decode_map[$encoder[19] as usize] = 19;
        $decode_map[$encoder[20] as usize] = 20;
        $decode_map[$encoder[21] as usize] = 21;
        $decode_map[$encoder[22] as usize] = 22;
        $decode_map[$encoder[23] as usize] = 23;
        $decode_map[$encoder[24] as usize] = 24;
        $decode_map[$encoder[25] as usize] = 25;
        $decode_map[$encoder[26] as usize] = 26;
        $decode_map[$encoder[27] as usize] = 27;
        $decode_map[$encoder[28] as usize] = 28;
        $decode_map[$encoder[29] as usize] = 29;
        $decode_map[$encoder[30] as usize] = 30;
        $decode_map[$encoder[31] as usize] = 31;
        $decode_map[$encoder[32] as usize] = 32;
        $decode_map[$encoder[33] as usize] = 33;
        $decode_map[$encoder[34] as usize] = 34;
        $decode_map[$encoder[35] as usize] = 35;
        $decode_map[$encoder[36] as usize] = 36;
        $decode_map[$encoder[37] as usize] = 37;
        $decode_map[$encoder[38] as usize] = 38;
        $decode_map[$encoder[39] as usize] = 39;
        $decode_map[$encoder[40] as usize] = 40;
        $decode_map[$encoder[41] as usize] = 41;
        $decode_map[$encoder[42] as usize] = 42;
        $decode_map[$encoder[43] as usize] = 43;
        $decode_map[$encoder[44] as usize] = 44;
        $decode_map[$encoder[45] as usize] = 45;
        $decode_map[$encoder[46] as usize] = 46;
        $decode_map[$encoder[47] as usize] = 47;
        $decode_map[$encoder[48] as usize] = 48;
        $decode_map[$encoder[49] as usize] = 49;
        $decode_map[$encoder[50] as usize] = 50;
        $decode_map[$encoder[51] as usize] = 51;
        $decode_map[$encoder[52] as usize] = 52;
        $decode_map[$encoder[53] as usize] = 53;
        $decode_map[$encoder[54] as usize] = 54;
        $decode_map[$encoder[55] as usize] = 55;
        $decode_map[$encoder[56] as usize] = 56;
        $decode_map[$encoder[57] as usize] = 57;
        $decode_map[$encoder[58] as usize] = 58;
        $decode_map[$encoder[59] as usize] = 59;
        $decode_map[$encoder[60] as usize] = 60;
        $decode_map[$encoder[61] as usize] = 61;
        $decode_map[$encoder[62] as usize] = 62;
        $decode_map[$encoder[63] as usize] = 63;
    };
}

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

const BASE: usize = 64;

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
/// This is the same as StdBase64 but omits padding characters.
pub const RAW_STD_ENCODING: Base64 = Base64::new_unchecked(ENCODE_STD).with_padding_unchecked(None);

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
        check_64!(encoder, CH, return Err(Error::InvalidEncoder));

        let mut decode_map = DECODE_MAP_INITIALIZE;
        assign_64!(encoder, decode_map);
        assign_decode_map!(decode_map, encoder);

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
        check_64!(
            encoder,
            CH,
            panic!("encoding alphabet contains newline character or padding character")
        );

        let mut decode_map = DECODE_MAP_INITIALIZE;
        assign_64!(encoder, decode_map);
        assign_decode_map!(decode_map, encoder);

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
            decode_map,
            pad_char: _,
            strict,
        } = self;

        match pad {
            Some(ch) => {
                check_64!(encoder, ch, return Err(Error::InvalidPadding));
            }
            None => {
                check_64!(encoder, return Err(Error::InvalidPadding));
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
            decode_map,
            pad_char: _,
            strict,
        } = self;

        match pad {
            Some(ch) => {
                check_64!(
                    encoder,
                    ch,
                    panic!("encoding alphabet contains newline character or padding character")
                );
            }
            None => {
                check_64!(
                    encoder,
                    panic!("encoding alphabet contains newline character or padding character")
                );
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
}
