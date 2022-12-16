use super::{Range16, Range32, RangeTable};

macro_rules! r16 {
    ($({$lo:expr, $hi:expr, $stride: expr}),+ $(,)?) => {
        &[$(
            Range16 {
                lo: $lo,
                hi: $hi,
                stride: $stride,
            },
        )*]
    };
}

macro_rules! r32 {
    ($({$lo:expr, $hi:expr, $stride: expr}),+ $(,)?) => {
        &[$(
            Range32 {
                lo: $lo,
                hi: $hi,
                stride: $stride,
            },
        )*]
    };
}

/// DIGIT range table
pub const DIGIT: RangeTable = ND;

const ND: RangeTable = RangeTable {
    r16: r16! {
        {0x0030, 0x0039, 1},
        {0x0660, 0x0669, 1},
        {0x06f0, 0x06f9, 1},
        {0x07c0, 0x07c9, 1},
        {0x0966, 0x096f, 1},
        {0x09e6, 0x09ef, 1},
        {0x0a66, 0x0a6f, 1},
        {0x0ae6, 0x0aef, 1},
        {0x0b66, 0x0b6f, 1},
        {0x0be6, 0x0bef, 1},
        {0x0c66, 0x0c6f, 1},
        {0x0ce6, 0x0cef, 1},
        {0x0d66, 0x0d6f, 1},
        {0x0de6, 0x0def, 1},
        {0x0e50, 0x0e59, 1},
        {0x0ed0, 0x0ed9, 1},
        {0x0f20, 0x0f29, 1},
        {0x1040, 0x1049, 1},
        {0x1090, 0x1099, 1},
        {0x17e0, 0x17e9, 1},
        {0x1810, 0x1819, 1},
        {0x1946, 0x194f, 1},
        {0x19d0, 0x19d9, 1},
        {0x1a80, 0x1a89, 1},
        {0x1a90, 0x1a99, 1},
        {0x1b50, 0x1b59, 1},
        {0x1bb0, 0x1bb9, 1},
        {0x1c40, 0x1c49, 1},
        {0x1c50, 0x1c59, 1},
        {0xa620, 0xa629, 1},
        {0xa8d0, 0xa8d9, 1},
        {0xa900, 0xa909, 1},
        {0xa9d0, 0xa9d9, 1},
        {0xa9f0, 0xa9f9, 1},
        {0xaa50, 0xaa59, 1},
        {0xabf0, 0xabf9, 1},
        {0xff10, 0xff19, 1},
    },
    r32: r32! {
        {0x104a0, 0x104a9, 1},
        {0x10d30, 0x10d39, 1},
        {0x11066, 0x1106f, 1},
        {0x110f0, 0x110f9, 1},
        {0x11136, 0x1113f, 1},
        {0x111d0, 0x111d9, 1},
        {0x112f0, 0x112f9, 1},
        {0x11450, 0x11459, 1},
        {0x114d0, 0x114d9, 1},
        {0x11650, 0x11659, 1},
        {0x116c0, 0x116c9, 1},
        {0x11730, 0x11739, 1},
        {0x118e0, 0x118e9, 1},
        {0x11950, 0x11959, 1},
        {0x11c50, 0x11c59, 1},
        {0x11d50, 0x11d59, 1},
        {0x11da0, 0x11da9, 1},
        {0x16a60, 0x16a69, 1},
        {0x16b50, 0x16b59, 1},
        {0x1d7ce, 0x1d7ff, 1},
        {0x1e140, 0x1e149, 1},
        {0x1e2f0, 0x1e2f9, 1},
        {0x1e950, 0x1e959, 1},
        {0x1fbf0, 0x1fbf9, 1},
    },
    latin_offset: 1,
};
