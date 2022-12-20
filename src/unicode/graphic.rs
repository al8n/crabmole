#![allow(non_upper_case_globals)]

use super::{is, is_excluding_latin, RangeTable, MAX_LATIN1, PROPERTIES};

bitflags::bitflags! {
    pub(crate) struct GraphicFlags: u8 {
        const pC = 1;
        const pP = 2;
        const pN = 4;
        const pS = 8;
        const pZ = 16;
        const pLu = 32;
        const pLl = 64;
        const pp = 128;
        const pg = Self::pp.bits | Self::pZ.bits;
        const pLo = Self::pLl.bits | Self::pLu.bits;
        const pLmask = Self::pLo.bits;
    }
}

pub(crate) const pC: u8 = GraphicFlags::pC.bits();
pub(crate) const pP: u8 = GraphicFlags::pP.bits();
pub(crate) const pN: u8 = GraphicFlags::pN.bits();
pub(crate) const pS: u8 = GraphicFlags::pS.bits();
pub(crate) const pZ: u8 = GraphicFlags::pZ.bits();
pub(crate) const pLu: u8 = GraphicFlags::pLu.bits();
pub(crate) const pLl: u8 = GraphicFlags::pLl.bits();
pub(crate) const pp: u8 = GraphicFlags::pp.bits();
pub(crate) const pg: u8 = GraphicFlags::pg.bits();
pub(crate) const pLo: u8 = GraphicFlags::pLo.bits();
pub(crate) const pLmask: u8 = GraphicFlags::pLmask.bits();

/// Defines the set of graphic characters according to Unicode.
pub const GRAPHIC_RANGES: &[RangeTable] = &[
    *RangeTable::L,
    *RangeTable::M,
    *RangeTable::N,
    *RangeTable::P,
    *RangeTable::S,
    *RangeTable::ZS,
];

/// Defines the set of printable characters according to Go.
/// ASCII space, U+0020, is handled separately.
pub const PRINT_RANGES: &[RangeTable] = &[
    *RangeTable::L,
    *RangeTable::M,
    *RangeTable::N,
    *RangeTable::P,
    *RangeTable::S,
];

/// Reports whether the rune is defined as a Graphic by Unicode.
/// Such characters include letters, marks, numbers, punctuation, symbols, and
/// spaces, from categories L, M, N, P, S, Zs.
#[inline]
pub const fn is_graphic(ch: char) -> bool {
    if (ch as u32) <= (MAX_LATIN1 as u32) {
        return PROPERTIES[ch as usize] & pg != 0;
    }
    contains(ch, GRAPHIC_RANGES)
}

/// reports whether the char is defined as printable by Rust. Such
/// characters include letters, marks, numbers, punctuation, symbols, and the
/// ASCII space character, from categories L, M, N, P, S and the ASCII space
/// character. This categorization is the same as [`is_graphic`] except that the
/// only spacing character is ASCII space, U+0020.
#[inline]
pub const fn is_print(ch: char) -> bool {
    if (ch as u32) <= (MAX_LATIN1 as u32) {
        return PROPERTIES[ch as usize] & pp != 0;
    }
    contains(ch, PRINT_RANGES)
}

/// Reports whether the char is a member of one of the ranges.
/// The function "In" provides a nicer signature and should be used in preference to IsOneOf.
#[inline]
pub const fn is_one_of(ranges: &[RangeTable], ch: char) -> bool {
    let mut i = 0;
    while i < ranges.len() {
        if is(&ranges[i], ch) {
            return true;
        }
        i += 1;
    }
    false
}

/// Reports whether the char is a member of one of the ranges.
#[inline]
pub const fn contains(ch: char, ranges: &[RangeTable]) -> bool {
    let mut i = 0;
    while i < ranges.len() {
        if is(&ranges[i], ch) {
            return true;
        }
        i += 1;
    }
    false
}

/// Reports whether the char is a control character.
/// The C (Other) Unicode category includes more code points
/// such as surrogates; use Is(C, r) to test for them.
#[inline]
pub const fn is_control(ch: char) -> bool {
    if ch as u32 <= (MAX_LATIN1 as u32) {
        return PROPERTIES[ch as usize] & pC != 0;
    }
    // All control characters are < MaxLatin1.
    false
}

/// Reports whether the char is a letter (category L).
#[inline]
pub const fn is_letter(ch: char) -> bool {
    if ch as u32 <= MAX_LATIN1 as u32 {
        return PROPERTIES[ch as usize] & pLmask != 0;
    }
    is_excluding_latin(RangeTable::LETTER, ch)
}

/// Reports whether the char is a mark character (category M).
#[inline]
pub const fn is_mark(ch: char) -> bool {
    is_excluding_latin(RangeTable::MARK, ch)
}

/// Reports whether the char is a number (category N).
#[inline]
pub const fn is_number(ch: char) -> bool {
    if ch as u32 <= MAX_LATIN1 as u32 {
        return PROPERTIES[ch as usize] & pN != 0;
    }
    is_excluding_latin(RangeTable::NUMBER, ch)
}

/// Reports whether the char is a Unicode punctuation character
/// (category [`RangeTable::P`]).
///
/// [`RangeTable::P`]: struct.RangeTable.html#methods.P
#[inline]
pub const fn is_punt(ch: char) -> bool {
    if ch as u32 <= MAX_LATIN1 as u32 {
        PROPERTIES[ch as usize] & pP != 0
    } else {
        is(RangeTable::PUNCT, ch)
    }
}

/// reports whether the char is a space character as defined
/// by Unicode's White Space property; in the Latin-1 space
/// this is
///
/// '\t', '\n', '\v', '\f', '\r', ' ', U+0085 (NEL), U+00A0 (NBSP).
///
/// Other definitions of spacing characters are set by category
/// Z and property Pattern_White_Space.
pub const fn is_space(ch: char) -> bool {
    if ch as u32 <= MAX_LATIN1 as u32 {
        return matches!(
            ch,
            '\t' | '\n' | '\x0b' | '\x0c' | '\r' | ' ' | '\u{85}' | '\u{a0}'
        );
    }

    is_excluding_latin(RangeTable::WHITE_SPACE, ch)
}

/// Reports whether the char is a symbolic character.
pub const fn is_symbol(ch: char) -> bool {
    if ch as u32 <= MAX_LATIN1 as u32 {
        PROPERTIES[ch as usize] & pS != 0
    } else {
        is_excluding_latin(RangeTable::SYMBOL, ch)
    }
}
