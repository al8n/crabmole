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

/// Reports whether the bytes in p begin with a full UTF-8 encoding of a rune.
/// An invalid encoding is considered a full Char since it will convert as a width-1 error rune.
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

/// Returns the number of bytes required to encode the rune.
/// It returns [`Option::None`] if the rune is not a valid value to encode in UTF-8.
#[inline]
pub const fn char_len(r: &char) -> Option<usize> {
    let r = *r as i32;
    match () {
        () if r < 0 => None,
        () if r <= CHAR_1_MAX => Some(1),
        () if r <= CHAR_2_MAX => Some(2),
        () if SURROGATE_MIN <= r && r <= SURROGATE_MAX => None,
        () if r <= CHAR_3_MAX => Some(3),
        () if r <= MAX_CHAR as i32 => Some(4),
        _ => None,
    }
}

/// Returns the number of runes in p. Erroneous and short
/// encodings are treated as single runes of width 1 byte.
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
        let c = p[i + 1];
        let c2 = p[i + 2];
        let c3 = p[i + 3];
        if c < accept.lo || accept.hi < c {
            size = 1;
        } else if size == 2 {
        } else if c2 < LOCB || HICB < c2 {
            size = 1;
        } else if size == 3 {
        } else if c3 < LOCB || HICB < c3 {
            size = 1;
        }
        i += size;
    }
    n
}

/// Reports whether the byte could be the first byte of an encoded,
/// possibly invalid rune. Second and subsequent bytes always have the top two
/// bits set to 10.
#[inline]
pub const fn char_start(b: u8) -> bool {
    b & 0xC0 != 0x80
}

/// Reports whether p consists entirely of valid UTF-8-encoded chars.
#[inline]
#[allow(clippy::manual_range_contains)]
pub fn valid(mut p: &[u8]) -> bool {
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

        p = &p[8..];
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

        let c = p[i + 1];
        let c2 = p[i + 2];
        let c3 = p[i + 3];
        if c < accept.lo || accept.hi < c {
            return false;
        } else if size == 2 {
        } else if c2 < LOCB || HICB < c2 {
            return false;
        } else if size == 3 {
        } else if c3 < LOCB || HICB < c3 {
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
    let r = r as i32;
    match () {
        () if 0 <= r && r < SURROGATE_MIN => true,
        () if SURROGATE_MAX < r && r <= MAX_CHAR as i32 => true,
        _ => false,
    }
}

/// Writes into p (which must be large enough) the UTF-8 encoding of the rune.
/// If the rune is out of range, it writes the encoding of CharError.
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
/// returns the extended buffer. If the rune is out of range,
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
        },
        () if (i > (MAX_CHAR as u32)) || ((SURROGATE_MIN as u32) <= i && i <= (SURROGATE_MAX as u32)) => {
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

/// Unpacks the first UTF-8 encoding in p and returns the rune and
/// its width in bytes. If p is empty it returns (CharError, 0). Otherwise, if
/// the encoding is invalid, it returns (CharError, 1). Both are impossible
/// results for correct, non-empty UTF-8.
///
/// An encoding is invalid if it is incorrect UTF-8, encodes a rune that is
/// out of range, or is not the shortest possible UTF-8 encoding for the
/// value. No other validation is performed.
#[allow(clippy::manual_range_contains)]
#[inline]
// TODO: const this function when rust issue#89259 stable
pub fn decode_char(p: &[u8]) -> (char, usize) {
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

        let mask = (x as u32) << 31 >> 31; // Create 0x0000 or 0xFFFF
        return (
            char::from_u32(((p[0] as u32) & !mask) | ((ERROR_CHAR as u32) & mask))
                .unwrap_or(ERROR_CHAR),
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
            char::from_u32((((p0 & MASK_2) as u32) << 6) | ((b1 & MASK_X) as u32))
                .unwrap_or(ERROR_CHAR),
            2,
        );
    }

    let b2 = p[2];
    if b2 < LOCB || HICB < b2 {
        return (ERROR_CHAR, 1);
    }

    if sz <= 3 {
        return (
            char::from_u32(
                (((p0 & MASK_3) as u32) << 12)
                    | (((b1 & MASK_X) as u32) << 6)
                    | ((b2 & MASK_X) as u32),
            )
            .unwrap_or(ERROR_CHAR),
            3,
        );
    }

    let b3 = p[3];
    if b3 < LOCB || HICB < b3 {
        return (ERROR_CHAR, 1);
    }

    (
        char::from_u32(
            (((p0 & MASK_4) as u32) << 18)
                | (((b1 & MASK_X) as u32) << 12)
                | (((b2 & MASK_X) as u32) << 6)
                | ((b3 & MASK_X) as u32),
        )
        .unwrap_or(ERROR_CHAR),
        4,
    )
}

/// Unpacks the last UTF-8 encoding in p and returns the rune and
/// its width in bytes. If p is empty it returns (CharError, 0). Otherwise, if
/// the encoding is invalid, it returns (CharError, 1). Both are impossible
/// results for correct, non-empty UTF-8.
///
/// An encoding is invalid if it is incorrect UTF-8, encodes a rune that is
/// out of range, or is not the shortest possible UTF-8 encoding for the
/// value. No other validation is performed.
#[allow(clippy::manual_range_contains)]
#[inline]
// TODO: const this function when rust issue#89259 stable
pub fn decode_last_char(p: &[u8]) -> (char, usize) {
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
    let lim = end - (UTF_MAX as isize);
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
    let (r, size) = decode_char(&p[start as usize..end as usize]);
    if start + (size as isize) != end {
        return (ERROR_CHAR, 1);
    }
    (r, size)
}
