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

/// Decode error
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CorruptInputError(u64);

impl CorruptInputError {
    /// leak the inner input byte
    #[inline]
    pub const fn into_inner(self) -> u64 {
        self.0
    }
}

impl core::fmt::Display for CorruptInputError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "illegal base64 data at input byte {}", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CorruptInputError {}

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
    /// number of bytes successfully written and CorruptInputError.
    /// New line characters (\r and \n) are ignored.
    pub fn decode(&self, src: &[u8], dst: &mut [u8]) -> Result<usize, CorruptInputError> {
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

        while src.len() >= 4 && dst.len() - n >= 4 {
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
    pub fn decode_to_vec(&self, src: &[u8]) -> Result<alloc::vec::Vec<u8>, CorruptInputError> {
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
    ) -> Result<(usize, usize), CorruptInputError> {
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
                        return Err(CorruptInputError((si - j) as u64));
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
                j -= 1;
                continue;
            }

            if let Some(ch) = self.pad_char {
                if (in_ as char) != ch {
                    return Err(CorruptInputError((si - 1) as u64));
                }
            }

            // We've reached the end and there's padding
            match j {
                0 | 1 => {
                    // incorrect padding
                    return Err(CorruptInputError((si - 1) as u64));
                }
                2 => {
                    // "==" is expected, the first "=" is already consumed.
                    // skip over newlines
                    while si < src.len() && (src[si] == b'\n' || src[si] == b'\r') {
                        si += 1;
                    }
                    if si == src.len() {
                        // not enough padding
                        return Err(CorruptInputError(src.len() as u64));
                    }
                    if let Some(ch) = self.pad_char {
                        if (src[si] as char) != ch {
                            return Err(CorruptInputError((si - 1) as u64));
                        }
                    }
                }
                _ => {}
            }

            // skip over newlines
            while si < src.len() && (src[si] == b'\n' || src[si] == b'\r') {
                si += 1;
            }
            if si < src.len() {
                // trailing garbage
                return Err(CorruptInputError(si as u64));
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
                    return Err(CorruptInputError((si - 1) as u64));
                }
                dbuf[1] = 0;
                dst[0] = dbuf[0];
                if self.strict && (dbuf[1] != 0 || dbuf[2] != 0) {
                    return Err(CorruptInputError((si - 2) as u64));
                }
            }
            3 => {
                dst[1] = dbuf[1];
                if self.strict && dbuf[2] != 0 {
                    return Err(CorruptInputError((si - 1) as u64));
                }
                dbuf[1] = 0;
                dst[0] = dbuf[0];
                if self.strict && (dbuf[1] != 0 || dbuf[2] != 0) {
                    return Err(CorruptInputError((si - 2) as u64));
                }
            }
            2 => {
                dst[0] = dbuf[0];
                if self.strict && (dbuf[1] != 0 || dbuf[2] != 0) {
                    return Err(CorruptInputError((si - 2) as u64));
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
            eprintln!("dbuf: {:?}", dbuf);
            assert_eq!(count, p.decoded.len());
            assert_eq!(&dbuf[..count], &p.decoded);
        }
    }
}
