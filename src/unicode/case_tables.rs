use super::{CaseRange, SpecialCase};

// TODO: This file contains the special casing rules for Turkish and Azeri only.
// It should encompass all the languages with special casing rules
// and be generated automatically, but that requires some API
// development first.

/// Turkish case
pub const TURKISH_CASE: SpecialCase<4> = SpecialCase([
    CaseRange {
        lo: 0x49,
        hi: 0x49,
        delta: [0, 0x131 - 0x49, 0],
    },
    CaseRange {
        lo: 0x130,
        hi: 0x130,
        delta: [0x130 - 0x69, 0, 0x130 - 0x69],
    },
    CaseRange {
        lo: 0x4a,
        hi: 0x4a,
        delta: [0, 0x69 - 0x130, 0],
    },
    CaseRange {
        lo: 0x49,
        hi: 0x130,
        delta: [0x49 - 0x131, 0, 0x49 - 0x131],
    },
]);

/// Azeri case
pub const AZERI_CASE: SpecialCase<4> = TURKISH_CASE;
