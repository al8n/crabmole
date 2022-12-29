/// 65533
const REPLACEMENT_CHARACTER: char = '\u{FFFD}';
/// 1114111
const MAX_CHAR: char = char::MAX;
/// 55296
const SURR_1: u32 = 0xd800;
/// 56320
const SURR_2: u32 = 0xdc00;
/// 57344
const SURR_3: u32 = 0xe000;
/// 65536
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
    if SURR_1 <= r1 && r1 < SURR_2 && SURR_2 <= r2 && r2 < SURR_3 {
        // Safety: checked valid
        return unsafe { core::mem::transmute(((r1 - SURR_1) << 10 | (r2 - SURR_2)) + SURR_SELF) };
    }
    REPLACEMENT_CHARACTER
}

/// Returns the UTF-16 surrogate pair r1, r2 for the given char.
/// If the char is not a valid Unicode code point or does not need encoding,
/// [`encode_char`] returns U+FFFD, U+FFFD.
#[inline]
// TODO: remove unsafe block when char::from_u32 is const stable
pub const fn encode_char(r: char) -> (char, char) {
    let mut r = r as i32;
    if r < (SURR_SELF as i32) || r > (MAX_CHAR as i32) {
        return (REPLACEMENT_CHARACTER, REPLACEMENT_CHARACTER);
    }
    r -= SURR_SELF as i32;
    unsafe {
        (
            core::mem::transmute(SURR_1 as i32 + ((r >> 10) & 0x3ff)),
            core::mem::transmute(SURR_2 as i32 + (r & 0x3FF)),
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
            () if (*v as u32) < SURR_1 || (SURR_3 <= (*v as u32) && (*v as u32) < SURR_SELF) => {
                a.push(unsafe { core::mem::transmute::<_, u32>(*v) as u16 });
            }
            () if (SURR_SELF <= (*v as u32)) && ((*v as u32) <= (MAX_CHAR as u32)) => {
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
                    unsafe { core::mem::transmute(r as u32) },
                    unsafe { core::mem::transmute(s[i + 1] as u32) },
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

#[cfg(test)]
mod tests {

    use alloc::vec;

    use crate::unicode::utf8::ERROR_CHAR;

    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(MAX_CHAR, std::char::MAX);
        assert_eq!(REPLACEMENT_CHARACTER, std::char::REPLACEMENT_CHARACTER)
    }

    struct EncodeTest {
        _in: Vec<char>,
        out: Vec<u16>,
    }

    fn encode_tests() -> Vec<EncodeTest> {
        vec![
            EncodeTest {
                _in: vec![1 as char, 2 as char, 3 as char, 4 as char],
                out: vec![1, 2, 3, 4],
            },
            EncodeTest {
                _in: unsafe {
                    vec![
                        core::mem::transmute(0xFFFFu32),
                        core::mem::transmute(0x10000u32),
                        core::mem::transmute(0x10001u32),
                        core::mem::transmute(0x12345u32),
                        core::mem::transmute(0x10FFFFu32),
                    ]
                },
                out: vec![
                    0xFFFF, 0xD800, 0xDC00, 0xD800, 0xDC01, 0xD808, 0xDF45, 0xDBFF, 0xDFFF,
                ],
            },
            EncodeTest {
                _in: vec![
                    'a',
                    'b',
                    unsafe { core::mem::transmute(0xD7FFu32) },
                    unsafe { core::mem::transmute(0xD800u32) },
                    unsafe { core::mem::transmute(0xDFFFu32) },
                    unsafe { core::mem::transmute(0xE000u32) },
                    unsafe { core::mem::transmute(0x110000u32) },
                    ERROR_CHAR,
                ],
                out: vec![
                    'a' as u16, 'b' as u16, 0xD7FF, 0xFFFD, 0xFFFD, 0xE000, 0xFFFD, 0xFFFD,
                ],
            },
        ]
    }

    #[test]
    fn test_encode() {
        for tt in encode_tests() {
            let out = encode(&tt._in);
            assert_eq!(out, tt.out);
        }
    }

    #[test]
    fn test_append_char() {
        for tt in encode_tests() {
            let mut out: Vec<u16> = alloc::vec::Vec::new();
            for u in 0..tt._in.len() {
                append_char(&mut out, tt._in[u]);
            }
            assert_eq!(out, tt.out);
        }
    }

    #[test]
    fn test_encode_char() {
        for (i, tt) in encode_tests().iter().enumerate() {
            let mut j = 0;
            for r in 0..tt._in.len() {
                let (r1, r2) = encode_char(tt._in[r]);
                if (tt._in[r] as u32) < 0x10000u32 || (tt._in[r] as u32) > (MAX_CHAR as u32) {
                    assert!(j < tt.out.len(), "{}:ran out of tt.out", i);
                    assert_eq!(r1, REPLACEMENT_CHARACTER);
                    assert_eq!(r2, REPLACEMENT_CHARACTER);
                    j += 1;
                } else {
                    assert!(j + 1 < tt.out.len(), "{}:ran out of tt.out", i);
                    assert_eq!(r1, unsafe { core::mem::transmute(tt.out[j] as u32) });
                    assert_eq!(r2, unsafe { core::mem::transmute(tt.out[j + 1] as u32) });
                    j += 2;
                    let dec = decode_char(r1, r2);
                    assert_eq!(dec, tt._in[r]);
                }
            }
            assert_eq!(j, tt.out.len());
        }
    }

    struct DecodeTest {
        _in: Vec<u16>,
        out: Vec<char>,
    }

    fn decode_tests() -> Vec<DecodeTest> {
        vec![
            DecodeTest {
                out: vec![1 as char, 2 as char, 3 as char, 4 as char],
                _in: vec![1, 2, 3, 4],
            },
            DecodeTest {
                out: vec![
                    std::char::from_u32(0xFFFFu32).unwrap_or(REPLACEMENT_CHARACTER),
                    std::char::from_u32(0x10000u32).unwrap_or(REPLACEMENT_CHARACTER),
                    std::char::from_u32(0x10001u32).unwrap_or(REPLACEMENT_CHARACTER),
                    std::char::from_u32(0x12345u32).unwrap_or(REPLACEMENT_CHARACTER),
                    std::char::from_u32(0x10FFFFu32).unwrap_or(REPLACEMENT_CHARACTER),
                ],
                _in: vec![
                    0xFFFF, 0xD800, 0xDC00, 0xD800, 0xDC01, 0xD808, 0xDF45, 0xDBFF, 0xDFFF,
                ],
            },
            DecodeTest {
                out: vec![
                    std::char::from_u32(0xFFFDu32).unwrap_or(REPLACEMENT_CHARACTER),
                    'a',
                ],
                _in: vec![0xD800, 'a' as u16],
            },
            DecodeTest {
                out: vec![std::char::from_u32(0xFFFD).unwrap_or(REPLACEMENT_CHARACTER)],
                _in: vec![0xDFFF],
            },
        ]
    }

    #[test]
    fn test_decode() {
        for tt in decode_tests() {
            let out = decode(&tt._in);
            assert_eq!(out, tt.out);
        }
    }

    struct DecodeCharTest {
        r1: char,
        r2: char,
        want: char,
    }

    fn decode_char_tests() -> Vec<DecodeCharTest> {
        vec![
            DecodeCharTest {
                r1: unsafe { core::mem::transmute(0xD800u32) },
                r2: unsafe { core::mem::transmute(0xDC00u32) },
                want: std::char::from_u32(0x10000u32).unwrap_or(REPLACEMENT_CHARACTER),
            },
            DecodeCharTest {
                r1: unsafe { core::mem::transmute(0xD800u32) },
                r2: unsafe { core::mem::transmute(0xDC01u32) },
                want: std::char::from_u32(0x10001u32).unwrap_or(REPLACEMENT_CHARACTER),
            },
            DecodeCharTest {
                r1: unsafe { core::mem::transmute(0xD808u32) },
                r2: unsafe { core::mem::transmute(0xDF45u32) },
                want: std::char::from_u32(0x12345u32).unwrap_or(REPLACEMENT_CHARACTER),
            },
            DecodeCharTest {
                r1: unsafe { core::mem::transmute(0xDBFFu32) },
                r2: unsafe { core::mem::transmute(0xDFFFu32) },
                want: std::char::from_u32(0x10FFFFu32).unwrap_or(REPLACEMENT_CHARACTER),
            },
            DecodeCharTest {
                r1: unsafe { core::mem::transmute(0xD800u32) },
                r2: 'a',
                want: std::char::from_u32(0xFFFDu32).unwrap_or(REPLACEMENT_CHARACTER),
            },
        ]
    }

    #[test]
    fn test_decode_char() {
        for tt in decode_char_tests() {
            let got = decode_char(tt.r1, tt.r2);
            assert_eq!(got, tt.want);
        }
    }

    struct SurrogateTest {
        r: char,
        want: bool,
    }

    fn surrogate_tests() -> Vec<SurrogateTest> {
        vec![
            SurrogateTest {
                r: '\u{007A}', // LATIN SMALL LETTER Z
                want: false,
            },
            SurrogateTest {
                r: '\u{6C34}', // CJK UNIFIED IDEOGRAPH-6C34 (water)
                want: false,
            },
            SurrogateTest {
                r: '\u{FEFF}', // Byte Order Mark
                want: false,
            },
            SurrogateTest {
                r: '\u{10000}', // LINEAR B SYLLABLE B008 A (first non-BMP code point)
                want: false,
            },
            SurrogateTest {
                r: '\u{1D11E}', // MUSICAL SYMBOL G CLEF
                want: false,
            },
            SurrogateTest {
                r: '\u{10FFFD}', // PRIVATE USE CHARACTER-10FFFD (last Unicode code point)
                want: false,
            },
            SurrogateTest {
                r: unsafe { core::mem::transmute(0xD7FFu32) }, // SURR1 - 1;
                want: false,
            },
            SurrogateTest {
                r: unsafe { core::mem::transmute(0xD800u32) }, // SURR1
                want: true,
            },
            SurrogateTest {
                r: unsafe { core::mem::transmute(0xDC00u32) }, // SURR2
                want: true,
            },
            SurrogateTest {
                r: unsafe { core::mem::transmute(0xE000u32) }, // SURR3
                want: false,
            },
            SurrogateTest {
                r: unsafe { core::mem::transmute(0xDFFFu32) }, // SURR3 - 1
                want: true,
            },
        ]
    }

    #[test]
    fn test_is_surrogate() {
        for tt in surrogate_tests() {
            let got = is_surrogate(tt.r);
            assert_eq!(got, tt.want);
        }
    }
}
