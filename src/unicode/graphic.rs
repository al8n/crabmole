#![allow(non_upper_case_globals)]

use super::RangeTable;

bitflags::bitflags! {
    struct P: u8 {
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

/// Defines the set of graphic characters according to Unicode.
pub const GRAPHIC_RANGES: &[&RangeTable] = &[
    RangeTable::L,
    RangeTable::M,
    RangeTable::N,
    RangeTable::P,
    RangeTable::S,
    RangeTable::ZS,
];

/// Defines the set of printable characters according to Go.
/// ASCII space, U+0020, is handled separately.
pub const PRINT_RANGES: &[&RangeTable] = &[
    RangeTable::L,
    RangeTable::M,
    RangeTable::N,
    RangeTable::P,
    RangeTable::S,
];
