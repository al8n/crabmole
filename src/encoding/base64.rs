macro_rules! assign_64 {
    ($src: ident, $dst: ident) => {
        $dst[0]= $src[0];
        $dst[1]= $src[1];
        $dst[2]= $src[2];
        $dst[3]=$src[3];
        $dst[4]= $src[4];
        $dst[5]= $src[5];
        $dst[6]= $src[6];
        $dst[7]= $src[7];
        $dst[8]= $src[8];
        $dst[9]= $src[9];
        $dst[10]= $src[10];
        $dst[11]= $src[11];
        $dst[12]= $src[12];
        $dst[13]= $src[13];
        $dst[14]= $src[14];
        $dst[15]= $src[15];
        $dst[16]= $src[16];
        $dst[17]= $src[17];
        $dst[18]= $src[18];
        $dst[19]= $src[19];
        $dst[20]= $src[20];
        $dst[21]= $src[21];
        $dst[22]= $src[22];
        $dst[23]= $src[23];
        $dst[24]= $src[24];
        $dst[25]= $src[25];
        $dst[26]= $src[26];
        $dst[27]= $src[27];
        $dst[28]= $src[28];
        $dst[29]= $src[29];
        $dst[30]= $src[30];
        $dst[31]= $src[31];
        $dst[32]= $src[32];
        $dst[33]= $src[33];
        $dst[34]= $src[34];
        $dst[35]= $src[35];
        $dst[36]= $src[36];
        $dst[37]= $src[37];
        $dst[38]= $src[38];
        $dst[39]= $src[39];
        $dst[40]= $src[40];
        $dst[41]= $src[41];
        $dst[42]= $src[42];
        $dst[43]= $src[43];
        $dst[44]= $src[44];
        $dst[45]= $src[45];
        $dst[46]= $src[46];
        $dst[47]= $src[47];
        $dst[48]= $src[48];
        $dst[49]= $src[49];
        $dst[50]= $src[50];
        $dst[51]= $src[51];
        $dst[52]= $src[52];
        $dst[53]= $src[53];
        $dst[54]= $src[54];
        $dst[55]= $src[55];
        $dst[56]= $src[56];
        $dst[57]= $src[57];
        $dst[58]= $src[58];
        $dst[59]= $src[59];
        $dst[60]= $src[60];
        $dst[61]= $src[61];
        $dst[62]= $src[62];
        $dst[63]= $src[63];
    };
}

macro_rules! check_64 {
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
        if $char == $arr[32] as char {
            $invalid;
        }

        if $char == $arr[33] as char {
            $invalid;
        }

        if $char == $arr[34] as char {
            $invalid;
        }

        if $char == $arr[35] as char {
            $invalid;
        }

        if $char == $arr[36] as char {
            $invalid;
        }

        if $char == $arr[37] as char {
            $invalid;
        }

        if $char == $arr[38] as char {
            $invalid;
        }

        if $char == $arr[39] as char {
            $invalid;
        }

        if $char == $arr[50] as char {
            $invalid;
        }

        if $char == $arr[41] as char {
            $invalid;
        }

        if $char == $arr[42] as char {
            $invalid;
        }

        if $char == $arr[43] as char {
            $invalid;
        }

        if $char == $arr[44] as char {
            $invalid;
        }

        if $char == $arr[45] as char {
            $invalid;
        }

        if $char == $arr[46] as char {
            $invalid;
        }

        if $char == $arr[47] as char {
            $invalid;
        }

        if $char == $arr[48] as char {
            $invalid;
        }

        if $char == $arr[49] as char {
            $invalid;
        }

        if $char == $arr[50] as char {
            $invalid;
        }

        if $char == $arr[51] as char {
            $invalid;
        }

        if $char == $arr[52] as char {
            $invalid;
        }

        if $char == $arr[53] as char {
            $invalid;
        }

        if $char == $arr[54] as char {
            $invalid;
        }

        if $char == $arr[55] as char {
            $invalid;
        }

        if $char == $arr[56] as char {
            $invalid;
        }

        if $char == $arr[57] as char {
            $invalid;
        }

        if $char == $arr[58] as char {
            $invalid;
        }

        if $char == $arr[59] as char {
            $invalid;
        }

        if $char == $arr[60] as char {
            $invalid;
        }

        if $char == $arr[61] as char {
            $invalid;
        }

        if $char == $arr[62] as char {
            $invalid;
        }

        if $char == $arr[63] as char {
            $invalid;
        }
    }; d
}

const BASE: usize = 64;

const ENCODE_STD: [u8;BASE] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const ENCODE_URL: [u8;BASE] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/*
    Encodings
*/

/// An Encoding is a radix 64 encoding/decoding scheme, defined by a
/// 64-character alphabet. The most common encoding is the "base64"
/// encoding defined in RFC 4648 and used in MIME (RFC 2045) and PEM
/// (RFC 1421).  RFC 4648 also defines an alternate encoding, which is
/// the standard encoding with - and _ substituted for + and /.
pub struct Encoding {
    encode: [u8;BASE],
    decode_map: [u8;256],
    pad_char: Option<char>,
    strict: bool,
}


// No padding
pub const NO_PADDING: Option<char> = None;

// Standard padding
pub const STD_PADDING: Option<char> = Some('=');

/// StdEncoding is the standard base64 encoding, as defined in
/// RFC 4648.
const STD_ENCODING: Encoding = Encoding::new(ENCODE_STD);

// URLEncoding is the alternate base64 encoding defined in RFC 4648.
// It is typically used in URLs and file names.
const URLEncoding: Encoding = Encoding::new(ENCODE_URL);
