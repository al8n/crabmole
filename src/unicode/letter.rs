use super::{
    pLl, pLmask, pLu, Range16, Range32, RangeTable, ASCII_FOLD, CASE_ORBIT, CASE_RANGES, MAX_ASCII,
    MAX_LATIN1, PROPERTIES,
};

/// Indices into the Delta arrays inside CaseRanges for case mapping.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum CaseDelta {
    /// Upper case
    Upper = 0,
    /// Lower case
    Lower = 1,
    /// Title case
    Title = 2,
    /// Max case
    Max = 3,
    /// If the Delta field of a [`CaseRange`] is [`CaseDelta::UpperLower`], it means
    /// this [`CaseRange`] represents a sequence of the form (say)
    /// Upper Lower Upper Lower.
    ///
    /// (Cannot be a valid delta.)
    UpperLower = 1114111 + 1,
}

/// to make the CaseRanges text shorter
type D = [i32; CaseDelta::Max as usize];

/// CaseRange represents a range of Unicode code points for simple (one
/// code point to one code point) case conversion.
/// The range runs from Lo to Hi inclusive, with a fixed stride of 1. Deltas
/// are the number to add to the code point to reach the code point for a
/// different case for that character. They may be negative. If zero, it
/// means the character is in the corresponding case. There is a special
/// case representing sequences of alternating corresponding Upper and Lower
/// pairs. It appears with a fixed Delta of
///
/// {UpperLower, UpperLower, UpperLower}
///
/// The constant UpperLower has an otherwise impossible delta value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CaseRange {
    pub(crate) lo: u32,
    pub(crate) hi: u32,
    pub(crate) delta: D,
}

impl CaseRange {
    /// Constructs a new CaseRange
    #[inline]
    pub const fn new(lo: u32, hi: u32, delta: D) -> Self {
        Self { lo, hi, delta }
    }

    /// Returns the lower bound of the range.
    #[inline]
    pub const fn lo(&self) -> u32 {
        self.lo
    }

    /// Returns the upper bound of the range.
    #[inline]
    pub const fn hi(&self) -> u32 {
        self.hi
    }

    /// Returns the delta of the range.
    #[inline]
    pub const fn delta(&self) -> D {
        self.delta
    }
}

/// [`SpecialCase`] represents language-specific case mappings such as Turkish.
/// Methods of SpecialCase customize (by overriding) the standard mappings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpecialCase<const N: usize>(pub(crate) [CaseRange; N]);

impl<const N: usize> SpecialCase<N> {
    /// Maps the [`char`] to upper case giving priority to the special mapping.
    #[inline]
    pub const fn to_upper(&self, ch: char) -> char {
        let (r1, had_mapping) = to_(CaseDelta::Upper, ch, &self.0);
        if r1 == ch && !had_mapping {
            return to_upper(ch);
        }
        r1
    }

    /// Maps the [`char`] to lower case giving priority to the special mapping.
    #[inline]
    pub const fn to_title(&self, ch: char) -> char {
        let (r1, had_mapping) = to_(CaseDelta::Title, ch, &self.0);
        if r1 == ch && !had_mapping {
            return to_title(ch);
        }
        r1
    }

    /// Maps the [`char`] to title case giving priority to the special mapping.
    #[inline]
    pub const fn to_lower(&self, ch: char) -> char {
        let (r1, had_mapping) = to_(CaseDelta::Lower, ch, &self.0);
        if r1 == ch && !had_mapping {
            return to_lower(ch);
        }
        r1
    }
}

/// Right now all the entries fit in uint16, so use uint16. If that changes, compilation
/// will fail (the constants in the composite literal will not fit in `u16`)
/// and the types here can change to `u32`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct FoldPair {
    pub(crate) from: u16,
    pub(crate) to: u16,
}

impl FoldPair {
    /// Creates a new [`FoldPair`].
    pub const fn new(from: u16, to: u16) -> Self {
        Self { from, to }
    }
}

/// The maximum size table for linear search for non-Latin1 char.
const LINEAR_MAX: usize = 18;

/// Reports whether r is in the sorted slice of 16-bit ranges.
#[inline]
const fn is16(ranges: &[Range16], r: u16) -> bool {
    if ranges.len() <= LINEAR_MAX || r <= (MAX_LATIN1 as u16) {
        let mut i = 0;
        while i < ranges.len() {
            let range_ = &ranges[i];
            if r < range_.lo {
                return false;
            }

            if r <= range_.hi {
                return range_.stride == 1 || (r - range_.lo) % range_.stride == 0;
            }
            i += 1;
        }
        return false;
    }

    // binary search over ranges
    let mut lo = 0;
    let mut hi = ranges.len();

    while lo < hi {
        let m = lo + (hi - lo) / 2;
        let range_ = &ranges[m];
        if range_.lo <= r && r <= range_.hi {
            return range_.stride == 1 || (r - range_.lo) % range_.stride == 0;
        }

        if r < range_.lo {
            hi = m;
        } else {
            lo = m + 1;
        }
    }

    false
}

/// Reports whether r is in the sorted slice of 32-bit ranges.
#[inline]
const fn is32(ranges: &[Range32], r: u32) -> bool {
    if ranges.len() <= LINEAR_MAX {
        let mut i = 0;
        while i < ranges.len() {
            let range_ = &ranges[i];
            if r < range_.lo {
                return false;
            }

            if r <= range_.hi {
                return range_.stride == 1 || (r - range_.lo) % range_.stride == 0;
            }
            i += 1;
        }
        return false;
    }

    // binary search over ranges
    let mut lo = 0;
    let mut hi = ranges.len();

    while lo < hi {
        let m = lo + (hi - lo) / 2;
        let range_ = &ranges[m];
        if range_.lo <= r && r <= range_.hi {
            return range_.stride == 1 || (r - range_.lo) % range_.stride == 0;
        }

        if r < range_.lo {
            hi = m;
        } else {
            lo = m + 1;
        }
    }

    false
}

/// Reports whether the char is in the specified table of ranges.
pub const fn is(rt: &RangeTable, ch: char) -> bool {
    let r16 = rt.r16;
    // Compare as uint32 to correctly handle negative chars.
    if !r16.is_empty() && (ch as u32) <= (r16[r16.len() - 1].hi as u32) {
        return is16(r16, ch as u16);
    }

    let r32 = rt.r32;
    if !r32.is_empty() && (ch as u32) <= r32[0].lo {
        return is32(r32, ch as u32);
    }

    false
}

/// Reports whether the char is a letter.
pub const fn is_excluding_latin(rt: &RangeTable, ch: char) -> bool {
    let r16 = rt.r16;
    // Compare as uint32 to correctly handle negative chars.
    let off = rt.latin_offset;
    let r16_len = r16.len();
    if off < r16_len && (ch as u32) <= (r16[r16_len - 1].hi as u32) {
        // Safety: the same as &r16[off..]
        // use unsafe because indexing by slice cannot be used in const fn.
        let r16 = unsafe { core::slice::from_raw_parts(r16.as_ptr().add(off), r16_len - off) };
        return is16(r16, ch as u16);
    }

    let r32 = rt.r32;
    if !r32.is_empty() && (ch as u32) >= r32[0].lo {
        return is32(r32, ch as u32);
    }

    false
}

/// Reports whether the char is an upper case letter.
#[inline]
pub const fn is_upper(ch: char) -> bool {
    if (ch as u32) <= (MAX_LATIN1 as u32) {
        return PROPERTIES[ch as usize] & pLmask == pLu;
    }

    is_excluding_latin(RangeTable::UPPER, ch)
}

/// Reports whether the char is a lower case letter.
#[inline]
pub fn is_lower(ch: char) -> bool {
    if (ch as u32) <= (MAX_LATIN1 as u32) {
        return PROPERTIES[ch as usize] & pLmask == pLl;
    }

    is_excluding_latin(RangeTable::LOWER, ch)
}

/// Reports whether the char is a title case letter.
#[inline]
pub const fn is_title(ch: char) -> bool {
    if (ch as u32) <= (MAX_ASCII as u32) {
        return false;
    }

    is_excluding_latin(RangeTable::TITLE, ch)
}

/// Iterates over Unicode code points equivalent under
/// the Unicode-defined simple case folding. Among the code points
/// equivalent to char (including char itself), [`simple_fold`] returns the
/// smallest char > r if one exists, or else the smallest char >= 0.
/// If r is not a valid Unicode code point, `simple_fold(r)` returns r.
///
/// For example:
///
/// `simple_fold('A') = 'a'`
///
/// `simple_fold('a') = 'A'`
///
/// `simple_fold('K') = 'k'`
///
/// `simple_fold('k') = '\u212A' (Kelvin symbol, K)`
///
/// `simple_fold('\u212A') = 'K'`
///
/// `simple_fold('1') = '1'`
///
/// `simple_fold(-2) = -2`
// TODO: remove unsafe block in this fn when feature = "const_option" is stable
pub const fn simple_fold(ch: char) -> char {
    if (ch as u32) > (char::MAX as u32) {
        return ch;
    }

    if (ch as usize) < ASCII_FOLD.len() {
        return unsafe { core::mem::transmute(ASCII_FOLD[ch as usize] as u32) };
    }

    // Consult caseOrbit table for special cases.
    let mut lo = 0;
    let mut hi = CASE_ORBIT.len();
    while lo < hi {
        let m = lo + (hi - lo) / 2;
        if (CASE_ORBIT[m].from as u32) < ch as u32 {
            lo = m + 1;
        } else {
            hi = m;
        }
    }

    if lo < CASE_ORBIT.len() && (CASE_ORBIT[lo].from as u32) == (ch as u32) {
        return unsafe { core::mem::transmute(CASE_ORBIT[lo].to as u32) };
    }

    // No folding specified. This is a one- or two-element
    // equivalence class containing char and ToLower(char)
    // and ToUpper(char) if they are different from char.
    let c = to_lower(ch);
    if c != ch {
        return c;
    }
    to_upper(ch)
}

/// Maps the char to lower case.
pub const fn to_lower(ch: char) -> char {
    if (ch as u32) <= (MAX_ASCII as u32) {
        let mut c = ch;
        if 'A' as u32 <= (ch as u32) && (ch as u32) <= 'Z' as u32 {
            c = unsafe { core::mem::transmute(c as u32 + 'a' as u32 - 'A' as u32) };
        }
        return c;
    }

    to(CaseDelta::Lower, ch)
}

/// Maps the char to upper case.
pub const fn to_upper(ch: char) -> char {
    if (ch as u32) <= (MAX_ASCII as u32) {
        let mut c = ch;
        if 'a' as u32 <= (ch as u32) && (ch as u32) <= 'z' as u32 {
            c = unsafe { core::mem::transmute(c as u32 - 'a' as u32 + 'A' as u32) };
        }
        return c;
    }

    to(CaseDelta::Upper, ch)
}

/// Maps the char to title case.
pub const fn to_title(ch: char) -> char {
    if (ch as u32) <= (MAX_ASCII as u32) {
        let mut c = ch;
        if 'a' as u32 <= (ch as u32) && (ch as u32) <= 'z' as u32 {
            c = unsafe { core::mem::transmute(c as u32 - 'a' as u32 + 'A' as u32) };
        }
        return c;
    }

    to(CaseDelta::Title, ch)
}

/// Maps the char to the specified case: UpperCase, LowerCase, or TitleCase.
#[inline]
pub const fn to(case: CaseDelta, ch: char) -> char {
    let (ch, _) = to_(case, ch, CASE_RANGES);
    ch
}

const fn to_(case: CaseDelta, ch: char, case_range: &[CaseRange]) -> (char, bool) {
    if CaseDelta::Max as usize <= case as usize {
        // as reasonable an error as any
        return (char::REPLACEMENT_CHARACTER, false);
    }

    // binary search over ranges
    let mut lo = 0;
    let mut hi = case_range.len();
    while lo < hi {
        let m = lo + (hi - lo) / 2;
        let cr: &CaseRange = &case_range[m];

        if cr.lo <= (ch as u32) && (ch as u32) <= cr.hi {
            let delta = cr.delta[case as usize];
            if delta > char::MAX as i32 {
                // In an Upper-Lower sequence, which always starts with
                // an UpperCase letter, the real deltas always look like:
                //	{0, 1, 0}    UpperCase (Lower is next)
                //	{-1, 0, -1}  LowerCase (Upper, Title are previous)
                // The characters at even offsets from the beginning of the
                // sequence are upper case; the ones at odd offsets are lower.
                // The correct mapping can be done by clearing or setting the low
                // bit in the sequence offset.
                // The constants UpperCase and TitleCase are even while LowerCase
                // is odd so we take the low bit from _case.
                let loi = cr.lo as i32;
                return (
                    unsafe {
                        core::mem::transmute(loi + (((ch as i32) - loi) & !1 | ((case as i32) & 1)))
                    },
                    true,
                );
            }
            return (unsafe { core::mem::transmute((ch as i32) + delta) }, true);
        }

        if (ch as u32) < cr.lo {
            hi = m;
        } else {
            lo = m + 1;
        }
    }
    (ch, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unicode::*;
    use alloc::vec;

    fn upper_test() -> Vec<char> {
        vec![
            std::char::from_u32(0x41).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xc0).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xd8).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x100).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x139).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x14a).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x178).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x181).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x376).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x3cf).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x13bd).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x1f2a).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x2102).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x2c00).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x2c10).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x2c20).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xa650).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xa722).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xff3a).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x10400).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x1d400).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x1d7ca).unwrap_or(char::REPLACEMENT_CHARACTER),
        ]
    }

    fn not_upper_test() -> Vec<char> {
        vec![
            std::char::from_u32(0x40).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x5b).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x61).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x185).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x1b0).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x377).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x387).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x2150).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xab7d).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xffff).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x10000).unwrap_or(char::REPLACEMENT_CHARACTER),
        ]
    }

    fn letter_test() -> Vec<char> {
        vec![
            std::char::from_u32(0x41).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x61).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xaa).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xba).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xc8).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xdb).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xf9).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x2ec).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x535).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x620).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x6e6).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x93d).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xa15).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xb99).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xdc0).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xedd).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x1000).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x1200).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x1312).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x1401).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x2c00).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xa800).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xf900).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xfa30).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xffda).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xffdc).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x10000).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x10300).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x10400).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x20000).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x2f800).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x2fa1d).unwrap_or(char::REPLACEMENT_CHARACTER),
        ]
    }

    fn not_letter_test() -> Vec<char> {
        vec![
            std::char::from_u32(0x20).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x35).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x375).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x619).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x700).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x1885).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xfffe).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x1ffff).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x10ffff).unwrap_or(char::REPLACEMENT_CHARACTER),
        ]
    }

    fn space_test() -> Vec<char> {
        vec![
            std::char::from_u32(0x09).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x0a).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x0b).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x0c).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x0d).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x20).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x85).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0xA0).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x2000).unwrap_or(char::REPLACEMENT_CHARACTER),
            std::char::from_u32(0x3000).unwrap_or(char::REPLACEMENT_CHARACTER),
        ]
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CaseT {
        cas: CaseDelta,
        _in: char,
        out: char,
    }

    fn case_test() -> Vec<CaseT> {
        vec![
            // ASCII (special-cased so test carefully)
            CaseT {
                cas: CaseDelta::Upper,
                _in: '\n',
                out: '\n',
            },
            CaseT {
                cas: CaseDelta::Upper,
                _in: 'a',
                out: 'A',
            },
            CaseT {
                cas: CaseDelta::Upper,
                _in: 'A',
                out: 'A',
            },
            CaseT {
                cas: CaseDelta::Upper,
                _in: '7',
                out: '7',
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: '\n',
                out: '\n',
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: 'a',
                out: 'a',
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: 'A',
                out: 'a',
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: '7',
                out: '7',
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: '\n',
                out: '\n',
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: 'a',
                out: 'A',
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: 'A',
                out: 'A',
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: '7',
                out: '7',
            },
            // Latin-1: easy to read the tests!
            CaseT {
                //128
                cas: CaseDelta::Lower,
                _in: 0x80 as char,
                out: 0x80 as char,
            },
            CaseT {
                //197
                cas: CaseDelta::Upper,
                _in: 'Å',
                out: 'Å',
            },
            CaseT {
                //229 197
                cas: CaseDelta::Upper,
                _in: 'å',
                out: 'Å',
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: 0x80 as char,
                out: 0x80 as char,
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: 'Å',
                out: 'å',
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: 'å',
                out: 'å',
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: 0x80 as char,
                out: 0x80 as char,
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: 'Å',
                out: 'Å',
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: 'å',
                out: 'Å',
            },
            // 0131;LATIN SMALL LETTER DOTLESS I;Ll;0;L;;;;;N;;;0049;;0049
            CaseT {
                //305 73
                cas: CaseDelta::Upper,
                _in: char::from_u32(0x0131).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: 'I',
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: char::from_u32(0x0131).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x0131).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: char::from_u32(0x0131).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: 'I',
            },
            // 0133;LATIN SMALL LIGATURE IJ;Ll;0;L;<compat> 0069 006A;;;;N;LATIN SMALL LETTER I J;;0132;;0132
            CaseT {
                //307 306
                cas: CaseDelta::Upper,
                _in: char::from_u32(0x0133).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x0132).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: char::from_u32(0x0133).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x0133).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: char::from_u32(0x0133).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x0132).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            // 212A;KELVIN SIGN;Lu;0;L;004B;;;;N;DEGREES KELVIN;;;006B;
            CaseT {
                //8490
                cas: CaseDelta::Upper,
                _in: char::from_u32(0x212A).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x212A).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                //8490 107
                cas: CaseDelta::Lower,
                _in: char::from_u32(0x212A).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: 'k',
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: char::from_u32(0x212A).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x212A).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            // From an UpperLower sequence
            // A640;CYRILLIC CAPITAL LETTER ZEMLYA;Lu;0;L;;;;;N;;;;A641;
            CaseT {
                //42560 42560
                cas: CaseDelta::Upper,
                _in: char::from_u32(0xA640).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0xA640).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                //42560 42561
                cas: CaseDelta::Lower,
                _in: char::from_u32(0xA640).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0xA641).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: char::from_u32(0xA640).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0xA640).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            // A641;CYRILLIC SMALL LETTER ZEMLYA;Ll;0;L;;;;;N;;;A640;;A640
            CaseT {
                cas: CaseDelta::Upper,
                _in: char::from_u32(0xA641).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0xA640).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: char::from_u32(0xA641).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0xA641).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: char::from_u32(0xA641).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0xA640).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            // A64E;CYRILLIC CAPITAL LETTER NEUTRAL YER;Lu;0;L;;;;;N;;;;A64F;
            CaseT {
                //42574
                cas: CaseDelta::Upper,
                _in: char::from_u32(0xA64E).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0xA64E).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                //42574 42575
                cas: CaseDelta::Lower,
                _in: char::from_u32(0xA64E).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0xA64F).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: char::from_u32(0xA64E).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0xA64E).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            // 65F;CYRILLIC SMALL LETTER YN;Ll;0;L;;;;;N;;;A65E;;A65E
            CaseT {
                //42591 42590
                cas: CaseDelta::Upper,
                _in: char::from_u32(0xA65F).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0xA65E).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: char::from_u32(0xA65F).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0xA65F).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: char::from_u32(0xA65F).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0xA65E).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            // From another UpperLower sequence
            // 0139;LATIN CAPITAL LETTER L WITH ACUTE;Lu;0;L;004C 0301;;;;N;LATIN CAPITAL LETTER L ACUTE;;;013A;
            CaseT {
                //313
                cas: CaseDelta::Upper,
                _in: char::from_u32(0x0139).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x0139).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: char::from_u32(0x0139).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x013A).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: char::from_u32(0x0139).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x0139).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            //  013F;LATIN CAPITAL LETTER L WITH MIDDLE DOT;Lu;0;L;<compat> 004C 00B7;;;;N;;;;0140;
            CaseT {
                cas: CaseDelta::Upper,
                _in: char::from_u32(0x013F).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x013F).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: char::from_u32(0x013F).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x0140).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: char::from_u32(0x013F).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x013F).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            // 0148;LATIN SMALL LETTER N WITH CARON;Ll;0;L;006E 030C;;;;N;LATIN SMALL LETTER N HACEK;;0147;;0147
            CaseT {
                cas: CaseDelta::Upper,
                _in: char::from_u32(0x0148).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x0147).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: char::from_u32(0x0148).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x0148).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: char::from_u32(0x0148).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x0147).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            // Lowercase lower than uppercase.
            // AB78;CHEROKEE SMALL LETTER GE;Ll;0;L;;;;;N;;;13A8;;13A8
            CaseT {
                cas: CaseDelta::Upper,
                _in: char::from_u32(0xAB78).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x13A8).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: char::from_u32(0xAB78).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0xAB78).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: char::from_u32(0xAB78).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x13A8).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Upper,
                _in: char::from_u32(0x13A8).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x13A8).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: char::from_u32(0x13A8).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0xAB78).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: char::from_u32(0x13A8).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x13A8).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            // Last block in the 5.1.0 table
            // 10400;DESERET CAPITAL LETTER LONG I;Lu;0;L;;;;;N;;;;10428;
            CaseT {
                cas: CaseDelta::Upper,
                _in: char::from_u32(0x10400).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x10400).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: char::from_u32(0x10400).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x10428).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: char::from_u32(0x10400).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x10400).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            // 10427;DESERET CAPITAL LETTER EW;Lu;0;L;;;;;N;;;;1044F;
            CaseT {
                cas: CaseDelta::Upper,
                _in: char::from_u32(0x10427).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x10427).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: char::from_u32(0x10427).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x1044F).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: char::from_u32(0x10427).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x10427).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            // 10428;DESERET SMALL LETTER LONG I;Ll;0;L;;;;;N;;;10400;;10400
            CaseT {
                cas: CaseDelta::Upper,
                _in: char::from_u32(0x10428).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x10400).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: char::from_u32(0x10428).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x10428).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: char::from_u32(0x10428).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x10400).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            // 1044F;DESERET SMALL LETTER EW;Ll;0;L;;;;;N;;;10427;;10427
            CaseT {
                cas: CaseDelta::Upper,
                _in: char::from_u32(0x1044F).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x10427).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: char::from_u32(0x1044F).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x1044F).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: char::from_u32(0x1044F).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x10427).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            // First one not in the 5.1.0 table
            // 10450;SHAVIAN LETTER PEEP;Lo;0;L;;;;;N;;;;;
            CaseT {
                // 66639
                cas: CaseDelta::Upper,
                _in: char::from_u32(0x10450).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x10450).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Lower,
                _in: char::from_u32(0x10450).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x10450).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                cas: CaseDelta::Title,
                _in: char::from_u32(0x10450).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x10450).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            // Non-letters with case.
            CaseT {
                //8545 8561
                cas: CaseDelta::Lower,
                _in: char::from_u32(0x2161).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x2171).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
            CaseT {
                //837 921
                cas: CaseDelta::Upper,
                _in: char::from_u32(0x0345).unwrap_or(char::REPLACEMENT_CHARACTER),
                out: char::from_u32(0x0399).unwrap_or(char::REPLACEMENT_CHARACTER),
            },
        ]
    }

    #[test]
    fn test_is_letter() {
        for r in upper_test() {
            assert!(is_letter(r));
        }
        for q in letter_test() {
            assert!(is_letter(q));
        }
        for p in not_letter_test() {
            assert!(!is_letter(p));
        }
    }

    #[test]
    fn test_is_upper() {
        for r in upper_test() {
            assert!(is_upper(r));
        }
        for q in not_upper_test() {
            assert!(!is_upper(q));
        }
        for p in not_letter_test() {
            assert!(!is_upper(p));
        }
    }

    #[test]
    fn test_to() {
        let mut r: char;
        for c in case_test() {
            r = to(c.cas, c._in);
            assert_eq!(r, c.out);
        }
    }

    #[test]
    fn test_to_upper_case() {
        for c in case_test() {
            if c.cas != CaseDelta::Upper {
                continue;
            }
            let r = to_upper(c._in);
            assert_eq!(r, c.out);
        }
    }

    #[test]
    fn test_to_lower_case() {
        for c in case_test() {
            if c.cas != CaseDelta::Lower {
                continue;
            }
            let r = to_lower(c._in);
            assert_eq!(r, c.out);
        }
    }

    #[test]
    fn test_to_title_case() {
        for c in case_test() {
            if c.cas != CaseDelta::Title {
                continue;
            }
            let r = to_title(c._in);
            assert_eq!(r, c.out);
        }
    }

    #[test]
    fn test_is_space() {
        for c in space_test() {
            assert!(is_space(c));
        }
        for c in letter_test() {
            assert!(!is_space(c));
        }
    }

    #[test]
    fn test_letter_optimizations() {
        let mut i: u32 = 0;
        while i <= MAX_LATIN1 as u32 {
            assert_eq!(
                is(
                    RangeTable::LETTER,
                    char::from_u32(i).unwrap_or(char::REPLACEMENT_CHARACTER)
                ),
                is_letter(char::from_u32(i).unwrap_or(char::REPLACEMENT_CHARACTER))
            );
            assert_eq!(
                is(
                    RangeTable::UPPER,
                    char::from_u32(i).unwrap_or(char::REPLACEMENT_CHARACTER)
                ),
                is_upper(char::from_u32(i).unwrap_or(char::REPLACEMENT_CHARACTER))
            );
            assert_eq!(
                is(
                    RangeTable::LOWER,
                    char::from_u32(i).unwrap_or(char::REPLACEMENT_CHARACTER)
                ),
                is_lower(char::from_u32(i).unwrap_or(char::REPLACEMENT_CHARACTER))
            );
            assert_eq!(
                is(
                    RangeTable::TITLE,
                    char::from_u32(i).unwrap_or(char::REPLACEMENT_CHARACTER)
                ),
                is_title(char::from_u32(i).unwrap_or(char::REPLACEMENT_CHARACTER))
            );
            assert_eq!(
                is(
                    RangeTable::WHITE_SPACE,
                    char::from_u32(i).unwrap_or(char::REPLACEMENT_CHARACTER)
                ),
                is_space(char::from_u32(i).unwrap_or(char::REPLACEMENT_CHARACTER))
            );
            assert_eq!(
                to(
                    CaseDelta::Upper,
                    char::from_u32(i).unwrap_or(char::REPLACEMENT_CHARACTER)
                ),
                to_upper(char::from_u32(i).unwrap_or(char::REPLACEMENT_CHARACTER))
            );
            assert_eq!(
                to(
                    CaseDelta::Lower,
                    char::from_u32(i).unwrap_or(char::REPLACEMENT_CHARACTER)
                ),
                to_lower(char::from_u32(i).unwrap_or(char::REPLACEMENT_CHARACTER))
            );
            assert_eq!(
                to(
                    CaseDelta::Title,
                    char::from_u32(i).unwrap_or(char::REPLACEMENT_CHARACTER)
                ),
                to_title(char::from_u32(i).unwrap_or(char::REPLACEMENT_CHARACTER))
            );
            i += 1;
        }
    }

    #[test]
    fn test_turkish_case() {
        let lower: Vec<char> = "abcçdefgğhıijklmnoöprsştuüvyz".chars().collect();
        let upper: Vec<char> = "ABCÇDEFGĞHIİJKLMNOÖPRSŞTUÜVYZ".chars().collect();

        for (i, l) in lower.iter().enumerate() {
            let u = upper[i];
            assert_eq!(TURKISH_CASE.to_lower(*l), *l);
            assert_eq!(TURKISH_CASE.to_upper(u), u);
            assert_eq!(TURKISH_CASE.to_upper(*l), u);
            assert_eq!(TURKISH_CASE.to_lower(u), *l);
            assert_eq!(TURKISH_CASE.to_title(u), u);
            assert_eq!(TURKISH_CASE.to_title(*l), u);
        }
    }

    fn simple_fold_tests() -> Vec<&'static str> {
        vec![
            // SimpleFold(x) returns the next equivalent rune > x or wraps
            // around to smaller values.

            // Easy cases.
            "Aa",
            "δΔ",
            // ASCII special cases.
            "KkK",
            "Ssſ",
            // Non-ASCII special cases.
            "ρϱΡ",
            "ͅΙιι",
            // Extra special cases: has lower/upper but no case fold.
            "İ",
            "ı",
            // Upper comes before lower (Cherokee).
            "\u{13b0}\u{ab80}",
        ]
    }

    #[test]
    fn test_simple_fold() {
        for tt in simple_fold_tests() {
            let cycle: Vec<char> = tt.chars().collect();
            let mut r = cycle[cycle.len() - 1];
            for out in cycle {
                r = simple_fold(r);
                assert_eq!(r, out);
                r = out;
            }
        }
    }

    fn no_change_for_capital_a() -> CaseRange {
        CaseRange {
            lo: 'A' as u32,
            hi: 'A' as u32,
            delta: [0, 0, 0],
        }
    }

    fn to_lower_special(c: crate::unicode::SpecialCase<1>, s: String) -> String {
        s.chars().map(|x| c.to_lower(x)).collect::<String>()
    }

    #[test]
    fn test_special_case_no_mapping() {
        // Issue 25636
        // no change for rune 'A', zero delta, under upper/lower/title case change.

        let got = to_lower_special(
            crate::unicode::SpecialCase([no_change_for_capital_a()]),
            "ABC".to_string(),
        );
        let want = "Abc";
        assert_eq!(got, want);
    }

    // #[test]
    // fn test_latin_offset() {
    //     fn maps() -> Vec<std::collections::HashMap<String, RangeTable>> {
    //         vec![
    //             categories,
    //             fold_category,
    //             fold_script,
    //             properties,
    //             scripts,
    //             _S
    //         ]
    //     }
    // }
    
}
