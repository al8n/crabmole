/// The "error" Char or "Unicode replacement character"
pub const ERROR_CHAR: char = char::REPLACEMENT_CHARACTER;

/// Characters below SELF_CHAR are represented AS themselves in a single byte.
pub const SELF_CHAR: char = 0x80 as char;

/// The highest valid code point a char can have, '\u{10FFFF}'.
pub const MAX_CHAR: char = char::MAX;

/// Maximum number of bytes of a UTF-8 encoded Unicode character.
pub const UTF_MAX: usize = 4;

const SURROGATE_MIN: i32 = 0xD800;
const SURROGATE_MAX: i32 = 0xDFFF;

// const T1: u8 = 0b00000000;
const TX: u8 = 0b10000000;
const T2: u8 = 0b11000000;
const T3: u8 = 0b11100000;
const T4: u8 = 0b11110000;
// const T5: u8 = 0b11111000;

const MASK_X: u8 = 0b00111111;
const MASK_2: u8 = 0b00011111;
const MASK_3: u8 = 0b00001111;
const MASK_4: u8 = 0b00000111;

const CHAR_1_MAX: i32 = (1 << 7) - 1;
const CHAR_2_MAX: i32 = (1 << 11) - 1;
const CHAR_3_MAX: i32 = (1 << 16) - 1;

const LOCB: u8 = 0b10000000;
const HICB: u8 = 0b10111111;

const XX: u8 = 0xF1;
const AS: u8 = 0xF0;
const S1: u8 = 0x02;
const S2: u8 = 0x13;
const S3: u8 = 0x03;
const S4: u8 = 0x23;
const S5: u8 = 0x34;
const S6: u8 = 0x04;
const S7: u8 = 0x44;

const FIRST: [u8; 256] = [
    //   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, // 0x00-0x0F
    AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, // 0x10-0x1F
    AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, // 0x20-0x2F
    AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, // 0x30-0x3F
    AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, // 0x40-0x4F
    AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, // 0x50-0x5F
    AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, // 0x60-0x6F
    AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, AS, // 0x70-0x7F
    //   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, // 0x80-0x8F
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, // 0x90-0x9F
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, // 0xA0-0xAF
    XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, // 0xB0-0xBF
    XX, XX, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, // 0xC0-0xCF
    S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, S1, // 0xD0-0xDF
    S2, S3, S3, S3, S3, S3, S3, S3, S3, S3, S3, S3, S3, S4, S3, S3, // 0xE0-0xEF
    S5, S6, S6, S6, S7, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, XX, // 0xF0-0xFF
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct AcceptRange {
    lo: u8,
    hi: u8,
}

impl AcceptRange {
    const fn new() -> Self {
        Self { lo: 0, hi: 0 }
    }
}

const ACCEPT_RANGES: [AcceptRange; 16] = [
    AcceptRange { lo: LOCB, hi: HICB },
    AcceptRange { lo: 0xA0, hi: HICB },
    AcceptRange { lo: LOCB, hi: 0x9F },
    AcceptRange { lo: 0x90, hi: HICB },
    AcceptRange { lo: LOCB, hi: 0x8F },
    AcceptRange::new(),
    AcceptRange::new(),
    AcceptRange::new(),
    AcceptRange::new(),
    AcceptRange::new(),
    AcceptRange::new(),
    AcceptRange::new(),
    AcceptRange::new(),
    AcceptRange::new(),
    AcceptRange::new(),
    AcceptRange::new(),
];

/// Reports whether the bytes in p begin with a full UTF-8 encoding of a char.
/// An invalid encoding is considered a full Char since it will convert as a width-1 error char.
#[inline]
pub const fn full_char(p: &[u8]) -> bool {
    let n = p.len();
    if n == 0 {
        return false;
    }

    let x = FIRST[p[0] as usize];
    if n >= (x & 7) as usize {
        return true; // ASCII, invalid or valid.
    }

    // Must be short or invalid.
    let accept = ACCEPT_RANGES[(x >> 4) as usize];
    if (n > 1 && (p[1] < accept.lo || accept.hi < p[1])) || (n > 2 && (p[2] < LOCB || HICB < p[2]))
    {
        return true;
    }
    false
}

/// Returns the number of bytes required to encode the char.
/// It returns [`Option::None`] if the char is not a valid value to encode in UTF-8.
#[inline]
pub const fn char_len(r: Option<char>) -> Option<usize> {
    match r {
        Some(ch) if (ch as i32) < 0 => None,
        Some(ch) if (ch as i32) <= CHAR_1_MAX => Some(1),
        Some(ch) if (ch as i32) <= CHAR_2_MAX => Some(2),
        Some(ch) if SURROGATE_MIN <= (ch as i32) && (ch as i32) <= SURROGATE_MAX => None,
        Some(ch) if (ch as i32) <= CHAR_3_MAX => Some(3),
        Some(ch) if (ch as i32) <= MAX_CHAR as i32 => Some(4),
        _ => None,
    }
}

/// Returns the number of chars in p. Erroneous and short
/// encodings are treated as single chars of width 1 byte.
#[inline]
pub const fn char_count(p: &[u8]) -> usize {
    let np = p.len();
    let mut i = 0;
    let mut n = 0;
    while i < np {
        n += 1;
        let c = p[i];
        if (c as char) < SELF_CHAR {
            // ASCII fast path
            i += 1;
            continue;
        }

        let x = FIRST[c as usize];
        if x == XX {
            i += 1; // invalid
            continue;
        }

        let mut size = (x & 7) as usize;
        if i + size > np {
            i += 1; // short or invalid
            continue;
        }

        let accept = ACCEPT_RANGES[(x >> 4) as usize];

        if p[i + 1] < accept.lo || accept.hi < p[i + 1] {
            size = 1;
        }

        if size == 2 {
            i += size;
            continue;
        }

        if p[i + 2] < LOCB || HICB < p[i + 2] {
            size = 1;
        }

        if size == 3 {
            i += size;
            continue;
        }

        if p[i + 3] < LOCB || HICB < p[i + 3] {
            size = 1;
        }

        i += size;
    }
    n
}

/// Reports whether the byte could be the first byte of an encoded,
/// possibly invalid char. Second and subsequent bytes always have the top two
/// bits set to 10.
#[inline]
pub const fn char_start(b: u8) -> bool {
    b & 0xC0 != 0x80
}

/// Reports whether p consists entirely of valid UTF-8-encoded chars.
#[inline]
pub const fn valid(mut p: &[u8]) -> bool {
    // Fast path. Check for and skip 8 bytes of ASCII characters per iteration.
    while p.len() >= 8 {
        // Combining two 32 bit loads allows the same code to be used
        // for 32 and 64 bit platforms.
        // The compiler can generate a 32bit load for first32 and second32
        // on many platforms. See test/codegen/memcombine.go.
        let first32 =
            (p[0] as u32) | ((p[1] as u32) << 8) | ((p[2] as u32) << 16) | ((p[3] as u32) << 24);
        let second32 =
            (p[4] as u32) | ((p[5] as u32) << 8) | ((p[6] as u32) << 16) | ((p[7] as u32) << 24);

        if (first32 | second32) & 0x80808080 != 0 {
            // Found a non ASCII byte (>= CharSelf).
            break;
        }

        // Safety: the same as &p[8..]
        p = unsafe { core::slice::from_raw_parts(p.as_ptr().add(8), p.len() - 8) };
    }

    let n = p.len();
    let mut i = 0;
    while i < n {
        let pi = p[i];
        if (pi as char) < SELF_CHAR {
            i += 1;
            continue;
        }
        let x = FIRST[pi as usize];
        if x == XX {
            return false; // Illegal starter byte.
        }
        let size = (x & 7) as usize;
        if i + size > n {
            return false; // Short or invalid
        }
        let accept = ACCEPT_RANGES[(x >> 4) as usize];

        if p[i + 1] < accept.lo || accept.hi < p[i + 1] {
            return false;
        } else if size == 2 {
        } else if p[i + 2] < LOCB || HICB < p[i + 2] {
            return false;
        } else if size == 3 {
        } else if p[i + 3] < LOCB || HICB < p[i + 3] {
            return false;
        }
        i += size;
    }
    true
}

/// Reports whether r can be legally encoded as UTF-8.
/// Code points that are out of range or a surrogate half are illegal.
#[inline]
pub const fn valid_char(r: char) -> bool {
    ((r as u32) ^ 0xD800).wrapping_sub(0x800) < 0x110000 - 0x800
}

/// Writes into p (which must be large enough) the UTF-8 encoding of the char.
/// If the char is out of range, it writes the encoding of [`ERROR_CHAR`].
/// It returns the number of bytes written.
#[inline]
pub fn encode_char(p: &mut [u8], r: char) -> usize {
    // Negative values are erroneous. Making it unsigned addresses the problem.
    let i = r as u32;
    match () {
        () if i <= (CHAR_1_MAX as u32) => {
            p[0] = i as u8;
            1
        }
        () if i <= (CHAR_2_MAX as u32) => {
            p[0] = T2 | ((i >> 6) as u8);
            p[1] = TX | ((i as u8) & MASK_X);
            2
        }
        () if i > (MAX_CHAR as u32)
            || (SURROGATE_MIN as u32) <= i && i <= (SURROGATE_MAX as u32) =>
        {
            let rr = ERROR_CHAR as u32;
            p[0] = T3 | ((rr >> 12) as u8);
            p[1] = TX | (((rr >> 6) as u8) & MASK_X);
            p[2] = TX | ((rr as u8) & MASK_X);
            3
        }
        () if i <= (CHAR_3_MAX as u32) => {
            p[0] = T3 | ((i >> 12) as u8);
            p[1] = TX | (((i >> 6) as u8) & MASK_X);
            p[2] = TX | ((i as u8) & MASK_X);
            3
        }
        _ => {
            p[0] = T4 | ((i >> 18) as u8);
            p[1] = TX | (((i >> 12) as u8) & MASK_X);
            p[2] = TX | (((i >> 6) as u8) & MASK_X);
            p[3] = TX | ((i as u8) & MASK_X);
            4
        }
    }
}

/// Appends the UTF-8 encoding of r to the end of p and
/// returns the extended buffer. If the char is out of range,
/// it appends the encoding of [`ERROR_CHAR`].
#[inline]
#[cfg(feature = "alloc")]
pub fn append_char(p: &mut alloc::vec::Vec<u8>, r: char) {
    // This function is inlineable for fast handling of ASCII.
    if (r as u32) <= (CHAR_1_MAX as u32) {
        p.push(r as u8);
        return;
    }
    let i = r as u32;
    match () {
        () if i <= (CHAR_2_MAX as u32) => {
            p.push(T2 | ((i >> 6) as u8));
            p.push(TX | ((i as u8) & MASK_X));
        }
        () if (i > (MAX_CHAR as u32))
            || ((SURROGATE_MIN as u32) <= i && i <= (SURROGATE_MAX as u32)) =>
        {
            let rr = ERROR_CHAR as u32;
            p.push(T3 | ((rr >> 12) as u8));
            p.push(TX | (((rr >> 6) as u8) & MASK_X));
            p.push(TX | ((rr as u8) & MASK_X));
        }
        () if i <= (CHAR_3_MAX as u32) => {
            p.push(T3 | ((i >> 12) as u8));
            p.push(TX | (((i >> 6) as u8) & MASK_X));
            p.push(TX | ((i as u8) & MASK_X));
        }
        _ => {
            p.push(T4 | ((i >> 18) as u8));
            p.push(TX | (((i >> 12) as u8) & MASK_X));
            p.push(TX | (((i >> 6) as u8) & MASK_X));
            p.push(TX | ((i as u8) & MASK_X));
        }
    }
}

/// Unpacks the first UTF-8 encoding in p and returns the char and
/// its width in bytes. If p is empty it returns ([`ERROR_CHAR`], 0). Otherwise, if
/// the encoding is invalid, it returns ([`ERROR_CHAR`], 1). Both are impossible
/// results for correct, non-empty UTF-8.
///
/// An encoding is invalid if it is incorrect UTF-8, encodes a char that is
/// out of range, or is not the shortest possible UTF-8 encoding for the
/// value. No other validation is performed.
#[allow(clippy::manual_range_contains)]
#[inline]
// TODO: const this function when rust issue#89259 stable and remove the unsafe blocks
pub const fn decode_char(p: &[u8]) -> (char, usize) {
    let n = p.len();
    if n < 1 {
        return (ERROR_CHAR, 0);
    }

    let p0 = p[0];
    let x = FIRST[p0 as usize];
    if x >= AS {
        // The following code simulates an additional check for x == xx and
        // handling the ASCII and invalid cases accordingly. This mask-and-or
        // approach prevents an additional branch.

        let mask = (x as i32) << 31 >> 31; // Create 0x0000 or 0xFFFF
        return (
            unsafe {
                core::mem::transmute(
                    (((p[0] as i32) & !mask) | ((ERROR_CHAR as i32) & mask)) as u32,
                )
            },
            1,
        );
    }

    let sz = (x & 7) as usize;
    let accept = ACCEPT_RANGES[(x >> 4) as usize];
    if n < sz {
        return (ERROR_CHAR, 1);
    }

    let b1 = p[1];
    if b1 < accept.lo || accept.hi < b1 {
        return (ERROR_CHAR, 1);
    }

    if sz <= 2 {
        return (
            unsafe { core::mem::transmute((((p0 & MASK_2) as u32) << 6) | ((b1 & MASK_X) as u32)) },
            2,
        );
    }

    let b2 = p[2];
    if b2 < LOCB || HICB < b2 {
        return (ERROR_CHAR, 1);
    }

    if sz <= 3 {
        return (
            unsafe {
                core::mem::transmute(
                    (((p0 & MASK_3) as u32) << 12)
                        | (((b1 & MASK_X) as u32) << 6)
                        | ((b2 & MASK_X) as u32),
                )
            },
            3,
        );
    }

    let b3 = p[3];
    if b3 < LOCB || HICB < b3 {
        return (ERROR_CHAR, 1);
    }

    (
        unsafe {
            core::mem::transmute(
                (((p0 & MASK_4) as u32) << 18)
                    | (((b1 & MASK_X) as u32) << 12)
                    | (((b2 & MASK_X) as u32) << 6)
                    | ((b3 & MASK_X) as u32),
            )
        },
        4,
    )
}

/// Unpacks the last UTF-8 encoding in p and returns the char and
/// its width in bytes. If p is empty it returns ([`ERROR_CHAR`], 0). Otherwise, if
/// the encoding is invalid, it returns ([`ERROR_CHAR`], 1). Both are impossible
/// results for correct, non-empty UTF-8.
///
/// An encoding is invalid if it is incorrect UTF-8, encodes a char that is
/// out of range, or is not the shortest possible UTF-8 encoding for the
/// value. No other validation is performed.
#[allow(clippy::manual_range_contains)]
#[inline]
pub const fn decode_last_char(p: &[u8]) -> (char, usize) {
    let end = p.len() as isize;
    if end == 0 {
        return (ERROR_CHAR, 0);
    }

    let mut start = end - 1;
    let r = p[start as usize] as char;
    if r < SELF_CHAR {
        return (r, 1);
    }

    // guard against O(n^2) behavior when traversing
    // backwards through strings with long sequences of
    // invalid UTF-8.
    let mut lim = end - (UTF_MAX as isize);
    if lim < 0 {
        lim = 0;
    }
    start -= 1;
    while start >= lim {
        if char_start(p[start as usize]) {
            break;
        }
        start -= 1;
    }

    if start < 0 {
        start = 0;
    }

    // Safety: the same as &p[start..end]
    let (r, size) = decode_char(unsafe {
        core::slice::from_raw_parts(p.as_ptr().add(start as usize), (end - start) as usize)
    });
    if start + (size as isize) != end {
        return (ERROR_CHAR, 1);
    }
    (r, size)
}

#[cfg(test)]
mod tests {
    use crate::unicode::utf8::*;

    #[derive(Debug)]
    struct Utf8Map {
        r: char,
        str: Vec<u8>,
    }

    fn utf8_map() -> Vec<Utf8Map> {
        vec![
            Utf8Map {
                r: std::char::from_u32(0x0000).unwrap_or(ERROR_CHAR),
                str: vec![0],
            },
            Utf8Map {
                r: std::char::from_u32(0x0001).unwrap_or(ERROR_CHAR),
                str: vec![1],
            },
            Utf8Map {
                r: std::char::from_u32(0x007e).unwrap_or(ERROR_CHAR),
                str: vec![126],
            },
            Utf8Map {
                r: std::char::from_u32(0x007f).unwrap_or(ERROR_CHAR),
                str: vec![127],
            },
            Utf8Map {
                r: std::char::from_u32(0x0080).unwrap_or(ERROR_CHAR),
                str: vec![194, 128],
            },
            Utf8Map {
                r: std::char::from_u32(0x0081).unwrap_or(ERROR_CHAR),
                str: vec![194, 129],
            },
            Utf8Map {
                r: std::char::from_u32(0x00bf).unwrap_or(ERROR_CHAR),
                str: vec![194, 191],
            },
            Utf8Map {
                r: std::char::from_u32(0x00c0).unwrap_or(ERROR_CHAR),
                str: vec![195, 128],
            },
            Utf8Map {
                r: std::char::from_u32(0x00c1).unwrap_or(ERROR_CHAR),
                str: vec![195, 129],
            },
            Utf8Map {
                r: std::char::from_u32(0x00c8).unwrap_or(ERROR_CHAR),
                str: vec![195, 136],
            },
            Utf8Map {
                r: std::char::from_u32(0x00d0).unwrap_or(ERROR_CHAR),
                str: vec![195, 144],
            },
            Utf8Map {
                r: std::char::from_u32(0x00e0).unwrap_or(ERROR_CHAR),
                str: vec![195, 160],
            },
            Utf8Map {
                r: std::char::from_u32(0x00f0).unwrap_or(ERROR_CHAR),
                str: vec![195, 176],
            },
            Utf8Map {
                r: std::char::from_u32(0x00f8).unwrap_or(ERROR_CHAR),
                str: vec![195, 184],
            },
            Utf8Map {
                r: std::char::from_u32(0x00ff).unwrap_or(ERROR_CHAR),
                str: vec![195, 191],
            },
            Utf8Map {
                r: std::char::from_u32(0x0100).unwrap_or(ERROR_CHAR),
                str: vec![196, 128],
            },
            Utf8Map {
                r: std::char::from_u32(0x07ff).unwrap_or(ERROR_CHAR),
                str: vec![223, 191],
            },
            Utf8Map {
                r: std::char::from_u32(0x0400).unwrap_or(ERROR_CHAR),
                str: vec![208, 128],
            },
            Utf8Map {
                r: std::char::from_u32(0x0800).unwrap_or(ERROR_CHAR),
                str: vec![224, 160, 128],
            },
            Utf8Map {
                r: std::char::from_u32(0x0801).unwrap_or(ERROR_CHAR),
                str: vec![224, 160, 129],
            },
            Utf8Map {
                r: std::char::from_u32(0x1000).unwrap_or(ERROR_CHAR),
                str: vec![225, 128, 128],
            },
            Utf8Map {
                r: std::char::from_u32(0xd000).unwrap_or(ERROR_CHAR),
                str: vec![237, 128, 128],
            },
            Utf8Map {
                r: std::char::from_u32(0xd7ff).unwrap_or(ERROR_CHAR), // last code point before surrogate half.
                str: vec![237, 159, 191],
            },
            Utf8Map {
                r: std::char::from_u32(0xe000).unwrap_or(ERROR_CHAR), // first code point after surrogate half.
                str: vec![238, 128, 128],
            },
            Utf8Map {
                r: std::char::from_u32(0xfffe).unwrap_or(ERROR_CHAR),
                str: vec![239, 191, 190],
            },
            Utf8Map {
                r: std::char::from_u32(0xffff).unwrap_or(ERROR_CHAR),
                str: vec![239, 191, 191],
            },
            Utf8Map {
                r: std::char::from_u32(0x10000).unwrap_or(ERROR_CHAR),
                str: vec![240, 144, 128, 128],
            },
            Utf8Map {
                r: std::char::from_u32(0x10001).unwrap_or(ERROR_CHAR),
                str: vec![240, 144, 128, 129],
            },
            Utf8Map {
                r: std::char::from_u32(0x40000).unwrap_or(ERROR_CHAR),
                str: vec![241, 128, 128, 128],
            },
            Utf8Map {
                r: std::char::from_u32(0x10fffe).unwrap_or(ERROR_CHAR),
                str: vec![244, 143, 191, 190],
            },
            Utf8Map {
                r: std::char::from_u32(0x10ffff).unwrap_or(ERROR_CHAR),
                str: vec![244, 143, 191, 191],
            },
            Utf8Map {
                r: std::char::from_u32(0xFFFD).unwrap_or(ERROR_CHAR),
                str: vec![239, 191, 189],
            },
        ]
    }

    fn surrogate_map() -> Vec<Utf8Map> {
        vec![
            Utf8Map {
                r: std::char::from_u32(0xd800).unwrap_or(ERROR_CHAR),
                str: vec![237, 160, 128],
            },
            Utf8Map {
                r: std::char::from_u32(0xdfff).unwrap_or(ERROR_CHAR),
                str: vec![237, 191, 191],
            },
        ]
    }

    fn strings_tests() -> Vec<Vec<u8>> {
        vec![
            "".as_bytes().to_vec(),
            "abcd".as_bytes().to_vec(),
            "☺☻☹".as_bytes().to_vec(),
            "日a本b語ç日ð本Ê語þ日¥本¼語i日©".as_bytes().to_vec(),
            "日a本b語ç日ð本Ê語þ日¥本¼語i日©日a本b語ç日ð本Ê語þ日¥本¼語i日©日a本b語ç日ð本Ê語þ日¥本¼語i日©".as_bytes().to_vec(),
            vec![128, 128, 128, 128]
        ]
    }

    fn string_tests() -> Vec<String> {
        vec![
            "".to_string(),
            "abcd".to_string(),
            "☺☻☹".to_string(),
            "日a本b語ç日ð本Ê語þ日¥本¼語i日©".to_string(),
            "日a本b語ç日ð本Ê語þ日¥本¼語i日©日a本b語ç日ð本Ê語þ日¥本¼語i日©日a本b語ç日ð本Ê語þ日¥本¼語i日©".to_string(),
            String::from_utf8_lossy(&[128, 128, 128, 128]).to_string()
        ]
    }

    #[test]
    fn test_full_char() {
        for m in utf8_map() {
            let b = m.str;
            assert!(full_char(&b));

            let b1 = &b[..b.len()];
            assert!(full_char(b1));
        }

        for s in [b"\x00", b"\x01"] {
            let b = s;
            assert!(full_char(b));
        }
    }

    #[test]
    fn test_encode_char() {
        for m in utf8_map() {
            let b = m.str;
            let mut buf = vec![0; 10];
            let n = encode_char(&mut buf, m.r);
            let b1 = &buf[..n];
            assert_eq!(b1, b);
        }
    }

    #[test]
    fn test_append_char() {
        for m in utf8_map() {
            let mut buf: Vec<u8> = vec![];
            append_char(&mut buf, m.r);
            assert_eq!(&buf, &m.str);

            buf = "init".as_bytes().to_vec();
            append_char(&mut buf, m.r);
            let mut s = b"init".to_vec();
            for st in m.str {
                s.push(st);
            }
            assert_eq!(&buf, &s);
        }
    }

    #[test]
    fn test_decode_surrogate_char() {
        for m in surrogate_map() {
            let b = &m.str;
            let (r, size) = decode_char(b.as_slice());
            assert_eq!(r, ERROR_CHAR);
            assert_eq!(size, 1);
        }
    }

    #[test]
    fn test_decode_char() {
        for mut m in utf8_map() {
            let b = &mut m.str;
            let (r, size) = decode_char(b.as_slice());
            assert_eq!(r, m.r);
            assert_eq!(size, b.len());

            // there's an extra byte that bytes left behind - make sure trailing byte works
            let (r, size) = decode_char(&b[..b.capacity()]);
            assert_eq!(r, m.r);
            assert_eq!(size, b.len());

            let mut s = b.clone();
            s.push(0);
            let (r, size) = decode_char(&s);
            assert_eq!(r, m.r);
            assert_eq!(size, b.len());

            // make sure missing bytes fail
            let mut want_size = 1;
            if want_size >= b.len() {
                want_size = 0;
            }

            let (_, size) = decode_char(&b[..b.len() - 1]);
            assert_eq!(size, want_size);

            let n = m.str.len();
            let b = &mut m.str[..n - 1];
            let (_, size) = decode_char(b);
            assert_eq!(size, want_size);

            // make sure bad sequences fail
            let mut b = m.str.clone();
            let n = b.len();
            if n == 1 {
                b[0] = 128;
            } else {
                b[n - 1] = 127;
            }
            let (r, size) = decode_char(b.as_slice());
            assert_eq!(r, ERROR_CHAR);
            assert_eq!(size, 1);

            let s = &b;
            let (r, size) = decode_char(s);
            assert_eq!(r, ERROR_CHAR);
            assert_eq!(size, 1);
        }
    }

    fn invalid_sequence_test() -> Vec<Vec<u8>> {
        vec![
            b"\xed\xa0\x80\x80".to_vec(), // surrogate min
            b"\xed\xbf\xbf\x80".to_vec(), // surrogate max
            // xx
            b"\x91\x80\x80\x80".to_vec(),
            // s1
            b"\xC2\x7F\x80\x80".to_vec(),
            b"\xC2\xC0\x80\x80".to_vec(),
            b"\xDF\x7F\x80\x80".to_vec(),
            b"\xDF\xC0\x80\x80".to_vec(),
            // s2
            b"\xE0\x9F\xBF\x80".to_vec(),
            b"\xE0\xA0\x7F\x80".to_vec(),
            b"\xE0\xBF\xC0\x80".to_vec(),
            b"\xE0\xC0\x80\x80".to_vec(),
            // s3
            b"\xE1\x7F\xBF\x80".to_vec(),
            b"\xE1\x80\x7F\x80".to_vec(),
            b"\xE1\xBF\xC0\x80".to_vec(),
            b"\xE1\xC0\x80\x80".to_vec(),
            //s4
            b"\xED\x7F\xBF\x80".to_vec(),
            b"\xED\x80\x7F\x80".to_vec(),
            b"\xED\x9F\xC0\x80".to_vec(),
            b"\xED\xA0\x80\x80".to_vec(),
            // s5
            b"\xF0\x8F\xBF\xBF".to_vec(),
            b"\xF0\x90\x7F\xBF".to_vec(),
            b"\xF0\x90\x80\x7F".to_vec(),
            b"\xF0\xBF\xBF\xC0".to_vec(),
            b"\xF0\xBF\xC0\x80".to_vec(),
            b"\xF0\xC0\x80\x80".to_vec(),
            // s6
            b"\xF1\x7F\xBF\xBF".to_vec(),
            b"\xF1\x80\x7F\xBF".to_vec(),
            b"\xF1\x80\x80\x7F".to_vec(),
            b"\xF1\xBF\xBF\xC0".to_vec(),
            b"\xF1\xBF\xC0\x80".to_vec(),
            b"\xF1\xC0\x80\x80".to_vec(),
            // s7
            b"\xF4\x7F\xBF\xBF".to_vec(),
            b"\xF4\x80\x7F\xBF".to_vec(),
            b"\xF4\x80\x80\x7F".to_vec(),
            b"\xF4\x8F\xBF\xC0".to_vec(),
            b"\xF4\x8F\xC0\x80".to_vec(),
            b"\xF4\x90\x80\x80".to_vec(),
        ]
    }

    fn runtime_decode_char(s: Vec<u8>) -> char {
        let replace = ERROR_CHAR.to_string();
        let s1 = std::str::from_utf8(&s).unwrap_or(&replace).chars();
        #[allow(clippy::never_loop)]
        for r in s1 {
            return r;
        }
        ERROR_CHAR
    }

    #[test]
    fn test_decode_invalid_sequence() {
        for s in invalid_sequence_test() {
            let (r1, _) = decode_char(&s);
            let want = ERROR_CHAR;
            assert_eq!(r1, want);
            let r3 = runtime_decode_char(s);
            assert_eq!(r1, r3);
        }
    }

    fn bytes_order_before_lossy(_s: Vec<u8>) -> Vec<usize> {
        let mut err_order: Vec<usize> = vec![];
        let mut char_order: Vec<usize> = vec![];
        let mut l = 0;
        while !_s[l..].is_empty() && l < _s.len() {
            match std::str::from_utf8(&_s[l..]) {
                Ok(r) => {
                    l = _s.len();
                    for (i, _) in r.char_indices() {
                        char_order.push(i);
                    }
                }
                Err(err) => {
                    l += err.valid_up_to();
                    err_order.push(l);
                    l += 1;
                }
            }
        }
        if !err_order.is_empty() {
            #[allow(clippy::needless_range_loop)]
            for i in 0..char_order.len() {
                char_order[i] += err_order[err_order.len() - 1] + 1;
            }
        }
        if err_order.len() > 1 {
            for i in 0..err_order.len() - 1 {
                for (j, _) in std::str::from_utf8(&_s[err_order[i] + 1..err_order[i + 1]])
                    .unwrap()
                    .char_indices()
                {
                    char_order.push(err_order[i] + 1 + j);
                }
            }
        }
        if !err_order.is_empty() && err_order[0] > 0 {
            for (j, _) in std::str::from_utf8(&_s[..err_order[0]])
                .unwrap()
                .char_indices()
            {
                char_order.push(j);
            }
        }
        for i in err_order {
            char_order.push(i);
        }
        char_order.sort();
        char_order
    }

    #[test]
    fn test_sequencing() {
        fn test_sequence(_s: Vec<u8>) {
            #[derive(Debug, Clone, Copy, Eq, PartialEq)]
            struct Info {
                index: usize,
                ch: char,
            }
            let s2 = String::from_utf8_lossy(&_s).to_string();
            let char_pos = bytes_order_before_lossy(_s.clone());
            assert_eq!(char_pos.len(), s2.chars().count());
            let mut s1 = s2.char_indices();
            let mut index = vec![
                Info {
                    index: 0,
                    ch: ERROR_CHAR,
                };
                char_pos.len()
            ];

            let b = _s.as_slice();
            let mut si = 0;
            let mut j: isize = 0;
            for i in char_pos {
                let (_, r) = match s1.next() {
                    Some((i, r)) => (i, r),
                    _ => continue,
                };
                assert_eq!(si, i);
                index[j as usize] = Info { index: i, ch: r };
                j += 1;
                let (r1, size1) = decode_char(&b[i..]);
                assert_eq!(r, r1);
                si += size1;
            }

            j -= 1;
            si = b.len();
            while si > 0 {
                let (r1, size1) = decode_last_char(&_s[..si]);
                assert_eq!(r1, index[j as usize].ch);
                si -= size1;
                assert_eq!(si, index[j as usize].index);
                j -= 1;
            }
            assert_eq!(si, 0);
        }

        for ts in strings_tests() {
            for m in utf8_map() {
                for s in vec![
                    {
                        let mut x = ts.clone();
                        x.extend(m.str.iter());
                        x
                    },
                    {
                        let mut x = m.str.clone();
                        x.extend(ts.iter());
                        x
                    },
                    {
                        let mut x = ts.clone();
                        x.extend(m.str.iter());
                        x.extend(ts.iter());
                        assert_ne!(x, ts);
                        x
                    },
                ] {
                    test_sequence(s);
                }
            }
        }
    }

    fn runtime_char_count(s: String) -> usize {
        return s.chars().count();
    }

    #[test]
    fn test_runtime_conversion() {
        for ts in string_tests() {
            let count = char_count(ts.as_bytes());
            let n = runtime_char_count(ts.clone());
            assert_eq!(count, n);

            let chars = ts.chars().collect::<Vec<char>>();
            let n = chars.len();
            assert_eq!(count, n);

            for (i, r) in ts.chars().enumerate() {
                assert_eq!(r, chars[i]);
            }
        }
    }

    struct CharCountTest {
        in_: &'static [u8],
        out: usize,
    }

    fn char_count_tests() -> Vec<CharCountTest> {
        vec![
            CharCountTest {
                in_: "abcd".as_bytes(),
                out: 4,
            },
            CharCountTest {
                in_: "☺☻☹".as_bytes(),
                out: 3,
            },
            CharCountTest {
                in_: "1,2,3,4".as_bytes(),
                out: 7,
            },
            CharCountTest {
                in_: b"\xe2\x00",
                out: 2,
            },
            CharCountTest {
                in_: b"\xe2\x80",
                out: 2,
            },
            CharCountTest {
                in_: b"a\xe2\x80",
                out: 3,
            },
        ]
    }

    #[test]
    fn test_char_count() {
        for tt in char_count_tests() {
            assert_eq!(char_count(tt.in_), tt.out);
        }
    }

    struct CharLenTest {
        r: Option<char>,
        size: usize,
    }

    fn char_len_tests() -> Vec<CharLenTest> {
        vec![
            CharLenTest {
                r: std::char::from_u32(0),
                size: 1,
            },
            CharLenTest {
                r: Some('e'),
                size: 1,
            },
            CharLenTest {
                r: Some('é'),
                size: 2,
            },
            CharLenTest {
                r: Some('☺'),
                size: 3,
            },
            CharLenTest {
                r: Some(ERROR_CHAR),
                size: 3,
            },
            CharLenTest {
                r: Some(MAX_CHAR),
                size: 4,
            },
            CharLenTest {
                r: std::char::from_u32(0xD800),
                size: 0,
            },
            CharLenTest {
                r: std::char::from_u32(0xDFFF),
                size: 0,
            },
            CharLenTest {
                r: std::char::from_u32((MAX_CHAR as u32) + 1),
                size: 0,
            },
        ]
    }

    #[test]
    fn test_char_len() {
        for tt in char_len_tests() {
            let size = char_len(tt.r).unwrap_or(0);
            assert_eq!(size, tt.size);
        }
    }

    struct ValidTest {
        in_: &'static [u8],
        out: bool,
    }

    fn valid_tests() -> Vec<ValidTest> {
        vec![
            ValidTest {
                in_: "".as_bytes(),
                out: true,
            },
            ValidTest {
                in_: "a".as_bytes(),
                out: true,
            },
            ValidTest {
                in_: "abc".as_bytes(),
                out: true,
            },
            ValidTest {
                in_: "Ж".as_bytes(),
                out: true,
            },
            ValidTest {
                in_: "ЖЖ".as_bytes(),
                out: true,
            },
            ValidTest {
                in_: "брэд-ЛГТМ".as_bytes(),
                out: true,
            },
            ValidTest {
                in_: "☺☻☹".as_bytes(),
                out: true,
            },
            ValidTest {
                in_: b"aa\xe2",
                out: false,
            },
            ValidTest {
                in_: &[66, 250],
                out: false,
            },
            ValidTest {
                in_: &[66, 250, 67],
                out: false,
            },
            ValidTest {
                in_: "a\u{FFFDb}".as_bytes(),
                out: true,
            },
            ValidTest {
                // U+10FFFF
                in_: b"\xF4\x8F\xBF\xBF",
                out: true,
            },
            ValidTest {
                // U+10FFFF+1; out of range
                in_: b"\xF4\x90\x80\x80",
                out: false,
            },
            ValidTest {
                // 0x1FFFFF; out of range
                in_: b"\xF7\xBF\xBF\xBF",
                out: false,
            },
            ValidTest {
                // 0x3FFFFFF; out of range
                in_: b"\xFB\xBF\xBF\xBF\xBF",
                out: false,
            },
            ValidTest {
                // U+0000 encoded in two bytes: incorrect
                in_: b"\xc0\x80",
                out: false,
            },
            ValidTest {
                // U+D800 high surrogate (sic)
                in_: b"\xed\xa0\x80",
                out: false,
            },
            ValidTest {
                // U+DFFF low surrogate (sic)
                in_: b"\xed\xbf\xbf",
                out: false,
            },
        ]
    }

    #[test]
    fn test_valid() {
        for tt in valid_tests() {
            assert_eq!(valid(tt.in_), tt.out);
        }
    }

    struct ValidCharTest {
        r: char,
        ok: bool,
    }

    #[allow(clippy::transmute_int_to_char)]
    fn valid_char_tests() -> Vec<ValidCharTest> {
        vec![
            ValidCharTest {
                r: std::char::from_u32(0).unwrap(),
                ok: true,
            },
            ValidCharTest {
                r: Some('e').unwrap(),
                ok: true,
            },
            ValidCharTest {
                r: Some('é').unwrap(),
                ok: true,
            },
            ValidCharTest {
                r: Some('☺').unwrap(),
                ok: true,
            },
            ValidCharTest {
                r: Some(ERROR_CHAR).unwrap(),
                ok: true,
            },
            ValidCharTest {
                r: Some(MAX_CHAR).unwrap(),
                ok: true,
            },
            ValidCharTest {
                r: std::char::from_u32(0xD7FF).unwrap(),
                ok: true,
            },
            ValidCharTest {
                r: unsafe { core::mem::transmute(0xD800u32) },
                ok: false,
            },
            ValidCharTest {
                r: unsafe { core::mem::transmute(0xDFFFu32) },
                ok: false,
            },
            ValidCharTest {
                r: std::char::from_u32(0xE000).unwrap(),
                ok: true,
            },
            ValidCharTest {
                r: unsafe { core::mem::transmute(MAX_CHAR as u32 + 1) },
                ok: false,
            },
        ]
    }

    #[test]
    fn test_valid_char() {
        for tt in valid_char_tests() {
            let ok = valid_char(tt.r);
            assert_eq!(ok, tt.ok);
        }
    }
}
