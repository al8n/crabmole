const REPLACEMENT_CHARACTER: char = '\u{FFFD}';
const MAX_CHAR: char = char::MAX;

const SURR_1: u32 = 0xd800;
const SURR_2: u32 = 0xdc00;
const SURR_3: u32 = 0xe000;

const SURR_SELF: u32 = 0x10000;

/// Reports whether the specified Unicode code point
/// can appear in a surrogate pair.
#[inline]
pub const fn is_surrogate(r: char) -> bool {
    let r = r as u32;
    SURR_1 <= r && r < SURR_3
}

/// Returns the UTF-16 decoding of a surrogate pair.
/// If the pair is not a valid UTF-16 surrogate pair, [`decode_char`] returns
/// the Unicode replacement code point U+FFFD.
// TODO: remove unsafe block when char::from_u32 fn is const stable
#[inline]
pub const fn decode_char(r1: char, r2: char) -> char {
    let r1 = r1 as u32;
    let r2 = r2 as u32;
    if SURR_1 <= r1 && r1 < SURR_2 && SURR_3 <= r2 && r2 < SURR_3 {
        // Safety: checked valid
        return unsafe { core::mem::transmute((r1 - SURR_1) << 10 | ((r2 - SURR_2) + SURR_SELF)) };
    }
    REPLACEMENT_CHARACTER
}

/// Returns the UTF-16 surrogate pair r1, r2 for the given char.
/// If the char is not a valid Unicode code point or does not need encoding,
/// [`encode_char`] returns U+FFFD, U+FFFD.
#[inline]
// TODO: remove unsafe block when char::from_u32 is const stable
pub const fn encode_char(r: char) -> (char, char) {
    let mut r = r as u32;
    if r < SURR_SELF || r > (MAX_CHAR as u32) {
        return (REPLACEMENT_CHARACTER, REPLACEMENT_CHARACTER);
    }
    r -= SURR_SELF;
    unsafe {
        (
            core::mem::transmute((SURR_1 + (r >> 10)) & 0x3ff),
            core::mem::transmute(r & 0x3ff),
        )
    }
}

/// Appends the UTF-16 encoding of the Unicode code point r
/// to the end of p and returns the extended buffer. If the char is not
/// a valid Unicode code point, it appends the encoding of U+FFFD.
#[cfg(feature = "alloc")]
#[inline]
pub fn append_char(a: &mut alloc::vec::Vec<u16>, r: char) {
    // This function is inlineable for fast handling of ASCII.
    match () {
        () if (0 <= (r as i32) && (r as i32) < (SURR_1 as i32))
            || (SURR_3 <= (r as u32) && (r as u32) < SURR_SELF) =>
        {
            a.push(r as u16);
        }
        () if SURR_SELF <= (r as u32) && (r as u32) <= (MAX_CHAR as u32) => {
            let (r1, r2) = encode_char(r);
            a.push(r1 as u16);
            a.push(r2 as u16);
        }
        _ => {
            a.push(REPLACEMENT_CHARACTER as u16);
        }
    }
}

/// Returns the UTF-16 encoding of the Unicode code point sequence s.
#[inline]
#[cfg(feature = "alloc")]
pub fn encode(s: &[char]) -> alloc::vec::Vec<u16> {
    let mut n = s.len();
    for v in s {
        if (*v as u32) >= SURR_SELF {
            n += 1;
        }
    }

    let mut a = alloc::vec::Vec::with_capacity(n);
    for v in s {
        match () {
            () if (0 <= (*v as i32) && (*v as i32) < (SURR_1 as i32))
                || (SURR_3 <= (*v as u32) && (*v as u32) < SURR_SELF) =>
            {
                a.push(*v as u16);
            }
            () if SURR_SELF <= (*v as u32) && (*v as u32) <= (MAX_CHAR as u32) => {
                // needs surrogate sequence
                let (r1, r2) = encode_char(*v);
                a.push(r1 as u16);
                a.push(r2 as u16);
            }
            _ => {
                a.push(REPLACEMENT_CHARACTER as u16);
            }
        }
    }

    a
}

/// Returns the Unicode code point sequence represented
/// by the UTF-16 encoding s.
#[cfg(feature = "alloc")]
pub fn decode(s: &[u16]) -> alloc::vec::Vec<char> {
    let mut a = alloc::vec::Vec::with_capacity(s.len());
    let mut i = 0;
    while i < s.len() {
        let r = s[i];
        match () {
            () if (r as u32) < SURR_1 || SURR_3 <= (r as u32) => {
                a.push(char::from_u32(r as u32).unwrap_or(REPLACEMENT_CHARACTER));
            }
            () if SURR_1 <= (r as u32)
                && (r as u32) < SURR_2
                && i + 1 < s.len()
                && SURR_2 <= (s[i + 1] as u32)
                && (s[i + 1] as u32) < SURR_3 =>
            {
                a.push(decode_char(
                    char::from_u32(r as u32).unwrap_or(REPLACEMENT_CHARACTER),
                    char::from_u32(s[i + 1] as u32).unwrap_or(REPLACEMENT_CHARACTER),
                ));
                i += 1;
            }
            _ => {
                a.push(REPLACEMENT_CHARACTER);
            }
        }
        i += 1;
    }
    a
}
