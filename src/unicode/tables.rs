use super::{Range16, Range32, RangeTable};

macro_rules! rt {
    (r16: {$({$lo16:expr, $hi16:expr, $stride16: expr}),* $(,)?}, r32: {$({$lo32:expr, $hi32:expr, $stride32: expr}),* $(,)?}, latin_offset: $latin_offset:expr $(,)?) => {
        &RangeTable {
            r16: r16![$({$lo16, $hi16, $stride16}),*],
            r32: r32![$({$lo32, $hi32, $stride32}),*],
            latin_offset: $latin_offset,
        }
    };
    (latin_offset: $latin_offset:expr $(,)?) => {
        &RangeTable {
            r16: r16![],
            r32: r32![],
            latin_offset: $latin_offset,
        }
    };
    (r16: {$({$lo16:expr, $hi16:expr, $stride16: expr}),* $(,)?} $(,)?) => {
        &RangeTable {
            r16: r16![$({$lo16, $hi16, $stride16}),*],
            r32: r32![],
            latin_offset: 0,
        }
    };
    (r32: {$({$lo32:expr, $hi32:expr, $stride32: expr}),* $(,)?} $(,)?) => {
        &RangeTable {
            r16: r16![],
            r32: r32![$({$lo32, $hi32, $stride32}),*],
            latin_offset: 0,
        }
    };
    (r32: {$({$lo32:expr, $hi32:expr, $stride32: expr}),* $(,)?}, latin_offset: $latin_offset:expr $(,)?) => {
        &RangeTable {
            r16: r16![],
            r32: r32![$({$lo32, $hi32, $stride32}),*],
            latin_offset: $latin_offset,
        }
    };
    (r16: {$({$lo16:expr, $hi16:expr, $stride16: expr}),* $(,)?}, latin_offset: $latin_offset:expr $(,)?) => {
        &RangeTable {
            r16: r16![$({$lo16, $hi16, $stride16}),*],
            r32: r32![],
            latin_offset: $latin_offset,
        }
    };
    (r16: {$({$lo16:expr, $hi16:expr, $stride16: expr}),* $(,)?}, r32: {$({$lo32:expr, $hi32:expr, $stride32: expr}),* $(,)?} $(,)?) => {
        &RangeTable {
            r16: r16![$({$lo16, $hi16, $stride16}),*],
            r32: r32![$({$lo32, $hi32, $stride32}),*],
            latin_offset: 0,
        }
    };
}

macro_rules! r16 {
    ($({$lo:expr, $hi:expr, $stride: expr}),* $(,)?) => {
        &[$(
            Range16::new($lo, $hi, $stride),
        )*]
    };
}

macro_rules! r32 {
    ($({$lo:expr, $hi:expr, $stride: expr}),* $(,)?) => {
        &[$(
            Range32::new($lo, $hi, $stride),
        )*]
    };
}

macro_rules! rt_aliases {
    (
        $(
            $name:ident {
                $(
                    $(#[$inner:ident $($args:tt)*])*
                    $alias:ident;
                )*
            }
        ),+ $(,)?
    ) => {
        $(
            impl RangeTable {
                $(
                    $(#[$inner $($args)*])*
                    pub const $alias: &'static Self = $name;
                )*
            }
        )*
    };
}

rt_aliases! {
    _C {
       /// The set of Unicode control and special characters, category C.
       C;
       /// The set of Unicode control and special characters, category C.
       OTHER;
    },
    _CC {
        /// CC is the set of Unicode characters in category Cc (Other, control).
        CC;
    },
    _ND {
        /// The set of Unicode characters in category Nd (Number, decimal digit).
        ND;
        /// The set of Unicode characters with the "decimal digit" property.
        DIGIT;
    }
}

const _C: &RangeTable = rt! {
    r16: {
        {0x0000, 0x001f, 1},
        {0x007f, 0x009f, 1},
        {0x00ad, 0x0600, 1363},
        {0x0601, 0x0605, 1},
        {0x061c, 0x06dd, 193},
        {0x070f, 0x08e2, 467},
        {0x180e, 0x200b, 2045},
        {0x200c, 0x200f, 1},
        {0x202a, 0x202e, 1},
        {0x2060, 0x2064, 1},
        {0x2066, 0x206f, 1},
        {0xd800, 0xf8ff, 1},
        {0xfeff, 0xfff9, 250},
        {0xfffa, 0xfffb, 1},
    },
    r32: {
        {0x110bd, 0x110cd, 16},
        {0x13430, 0x13438, 1},
        {0x1bca0, 0x1bca3, 1},
        {0x1d173, 0x1d17a, 1},
        {0xe0001, 0xe0020, 31},
        {0xe0021, 0xe007f, 1},
        {0xf0000, 0xffffd, 1},
        {0x100000, 0x10fffd, 1},
    },
    latin_offset: 2,
};

const _CC: &RangeTable = rt! {
    r16: {
        {0x0000, 0x001f, 1},
        {0x007f, 0x009f, 1},
    },
    latin_offset: 2,
};

const _CF: &RangeTable = rt! {
    r16: {
        {0x00ad, 0x0600, 1363},
        {0x0601, 0x0605, 1},
        {0x061c, 0x06dd, 193},
        {0x070f, 0x08e2, 467},
        {0x180e, 0x200b, 2045},
        {0x200c, 0x200f, 1},
        {0x202a, 0x202e, 1},
        {0x2060, 0x2064, 1},
        {0x2066, 0x206f, 1},
        {0xfeff, 0xfff9, 250},
        {0xfffa, 0xfffb, 1},
    },
    r32: {
        {0x110bd, 0x110cd, 16},
        {0x13430, 0x13438, 1},
        {0x1bca0, 0x1bca3, 1},
        {0x1d173, 0x1d17a, 1},
        {0xe0001, 0xe0020, 31},
        {0xe0021, 0xe007f, 1},
    },
};

const _ND: &RangeTable = rt! {
    r16: {
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
    r32: {
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
