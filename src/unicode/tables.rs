use super::{Range16, Range32, RangeTable};

macro_rules! rt {
    (name: $name:ident, r16: {$({$lo16:expr, $hi16:expr, $stride16: expr}),* $(,)?}, r32: {$({$lo32:expr, $hi32:expr, $stride32: expr}),* $(,)?}, latin_offset: $latin_offset:expr $(,)?) => {
        const $name: &RangeTable = &RangeTable {
            r16: r16![$({$lo16, $hi16, $stride16}),*],
            r32: r32![$({$lo32, $hi32, $stride32}),*],
            latin_offset: $latin_offset,
        };
    };
    (name: $name:ident, latin_offset: $latin_offset:expr $(,)?) => {
        const $name: &RangeTable = &RangeTable {
            r16: r16![],
            r32: r32![],
            latin_offset: $latin_offset,
        };
    };
    (name: $name:ident, r16: {$({$lo16:expr, $hi16:expr, $stride16: expr}),* $(,)?} $(,)?) => {
        const $name: &RangeTable = &RangeTable {
            r16: r16![$({$lo16, $hi16, $stride16}),*],
            r32: r32![],
            latin_offset: 0,
        };
    };
    (name: $name:ident, r32: {$({$lo32:expr, $hi32:expr, $stride32: expr}),* $(,)?} $(,)?) => {
        const $name: &RangeTable = &RangeTable {
            r16: r16![],
            r32: r32![$({$lo32, $hi32, $stride32}),*],
            latin_offset: 0,
        };
    };
    (name: $name:ident, r32: {$({$lo32:expr, $hi32:expr, $stride32: expr}),* $(,)?}, latin_offset: $latin_offset:expr $(,)?) => {
        const $name: &RangeTable = &RangeTable {
            r16: r16![],
            r32: r32![$({$lo32, $hi32, $stride32}),*],
            latin_offset: $latin_offset,
        };
    };
    (name: $name:ident, r16: {$({$lo16:expr, $hi16:expr, $stride16: expr}),* $(,)?}, latin_offset: $latin_offset:expr $(,)?) => {
        const $name: &RangeTable = &RangeTable {
            r16: r16![$({$lo16, $hi16, $stride16}),*],
            r32: r32![],
            latin_offset: $latin_offset,
        };
    };
    (name: $name:ident, r16: {$({$lo16:expr, $hi16:expr, $stride16: expr}),* $(,)?}, r32: {$({$lo32:expr, $hi32:expr, $stride32: expr}),* $(,)?} $(,)?) => {
        const $name: &RangeTable = &RangeTable {
            r16: r16![$({$lo16, $hi16, $stride16}),*],
            r32: r32![$({$lo32, $hi32, $stride32}),*],
            latin_offset: 0,
        };
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

macro_rules! index_helper {
    ($($key:literal:$val:ident),* $(,)?) => {
        $($key => $val,)*
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
    _CF {
        /// The set of Unicode characters in category Cf (Other, format).
        CF;
    },
    _CO {
        /// The set of Unicode characters in category Co (Other, private use).
        CO;
    },
    _CS {
        /// The set of Unicode characters in category Cs (Other, surrogate).
        CS;
    },
    _ND {
        /// The set of Unicode characters in category Nd (Number, decimal digit).
        ND;
        /// The set of Unicode characters with the "decimal digit" property.
        DIGIT;
    },
    _L {
        /// The set of Unicode letters, category Letter.
        LETTER;
        /// The set of Unicode letters, category L.
        L;
    },
    _LM {
        /// The set of Unicode characters in category Lm (Letter, modifier).
        LM;
    },
    _LO {
        /// The set of Unicode characters in category Lo (Letter, other).
        LO;
    },
    _LL {
        /// The set of Unicode characters in category Ll (Letter, lowercase).
        LL;
        /// The set of Unicode lower case letters.
        LOWER;
    },
    _M {
        /// The set of Unicode mark characters, category M.
        MARK;
        /// The set of Unicode mark characters, category M.
        M;
    },
    _MC {
        /// The set of Unicode characters in category Mc (Mark, spacing combining).
        MC;
    },
    _ME {
        /// The set of Unicode characters in category Me (Mark, enclosing).
        ME;
    },
    _MN {
        /// The set of Unicode characters in category Mn (Mark, nonspacing).
        MN;
    },
    _NL {
        /// The set of Unicode characters in category Nl (Number, letter).
        NL;
    },
    _NO {
        /// The set of Unicode characters in category No (Number, other).
        NO;
    },
    _N {
        /// The set of Unicode characters in category N (Number).
        NUMBER;
        /// The set of Unicode characters in category N (Number).
        N;
    },
    _PC {
        /// The set of Unicode characters in category Pc (Punctuation, connector).
        PC;
    },
    _PD {
        /// The set of Unicode characters in category Pd (Punctuation, dash).
        PD;
    },
    _PE {
        /// The set of Unicode characters in category Pe (Punctuation, close).
        PE;
    },
    _PF {
        /// The set of Unicode characters in category Pf (Punctuation, final quote).
        PF;
    },
    _PI {
        /// The set of Unicode characters in category Pi (Punctuation, initial quote).
        PI;
    },
    _PO {
        /// The set of Unicode characters in category Po (Punctuation, other).
        PO;
    },
    _PS {
        /// The set of Unicode characters in category Ps (Punctuation, open).
        PS;
    },
    _P {
        /// The set of Unicode punctuation characters, category P.
        PUNCT;
        /// The set of Unicode punctuation characters, category P.
        P;
    },
    _SC {
        /// The set of Unicode characters in category Sc (Symbol, currency).
        SC;
    },
    _SK {
        /// The set of Unicode characters in category Sk (Symbol, modifier).
        SK;
    },
    _SM {
        /// The set of Unicode characters in category Sm (Symbol, math).
        SM;
    },
    _SO {
        /// The set of Unicode characters in category So (Symbol, other).
        SO;
    },
    _S {
        /// The set of Unicode symbol characters, category S.
        SYMBOL;
        /// The set of Unicode symbol characters, category S.
        S;
    },
    _ZL {
        /// The set of Unicode characters in category Zl (Separator, line).
        ZL;
    },
    _ZP {
        /// The set of Unicode characters in category Zp (Separator, paragraph).
        ZP;
    },
    _ZS {
        /// The set of Unicode characters in category Zs (Separator, space).
        ZS;
    },
    _Z {
        /// The set of Unicode space characters, category Z.
        SPACE;
        /// The set of Unicode space characters, category Z.
        Z;
    },
    _LT {
        /// The set of Unicode characters in category Lt (Letter, titlecase).
        LT;
        /// The set of Unicode title case letters.
        TITLE;
    },
    _LU {
        /// The set of Unicode characters in category Lu (Letter, uppercase).
        LU;
        /// The set of Unicode upper case letters.
        UPPER;
    },
}

rt! {
    name: _C,
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
}

rt! {
    name: _CC,
    r16: {
        {0x0000, 0x001f, 1},
        {0x007f, 0x009f, 1},
    },
    latin_offset: 2,
}

rt! {
    name: _CF,
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
}

rt! {
    name: _CO,
    r16: {
        {0xe000, 0xf8ff, 1},
    },
    r32: {
        {0xf0000, 0xffffd, 1},
        {0x100000, 0x10fffd, 1},
    },
}

rt! {
    name: _CS,
    r16: {
        {0xd800, 0xdfff, 1},
    },
}

rt! {
    name: _L,
    r16: {
        {0x0041, 0x005a, 1},
        {0x0061, 0x007a, 1},
        {0x00aa, 0x00b5, 11},
        {0x00ba, 0x00c0, 6},
        {0x00c1, 0x00d6, 1},
        {0x00d8, 0x00f6, 1},
        {0x00f8, 0x02c1, 1},
        {0x02c6, 0x02d1, 1},
        {0x02e0, 0x02e4, 1},
        {0x02ec, 0x02ee, 2},
        {0x0370, 0x0374, 1},
        {0x0376, 0x0377, 1},
        {0x037a, 0x037d, 1},
        {0x037f, 0x0386, 7},
        {0x0388, 0x038a, 1},
        {0x038c, 0x038e, 2},
        {0x038f, 0x03a1, 1},
        {0x03a3, 0x03f5, 1},
        {0x03f7, 0x0481, 1},
        {0x048a, 0x052f, 1},
        {0x0531, 0x0556, 1},
        {0x0559, 0x0560, 7},
        {0x0561, 0x0588, 1},
        {0x05d0, 0x05ea, 1},
        {0x05ef, 0x05f2, 1},
        {0x0620, 0x064a, 1},
        {0x066e, 0x066f, 1},
        {0x0671, 0x06d3, 1},
        {0x06d5, 0x06e5, 16},
        {0x06e6, 0x06ee, 8},
        {0x06ef, 0x06fa, 11},
        {0x06fb, 0x06fc, 1},
        {0x06ff, 0x0710, 17},
        {0x0712, 0x072f, 1},
        {0x074d, 0x07a5, 1},
        {0x07b1, 0x07ca, 25},
        {0x07cb, 0x07ea, 1},
        {0x07f4, 0x07f5, 1},
        {0x07fa, 0x0800, 6},
        {0x0801, 0x0815, 1},
        {0x081a, 0x0824, 10},
        {0x0828, 0x0840, 24},
        {0x0841, 0x0858, 1},
        {0x0860, 0x086a, 1},
        {0x08a0, 0x08b4, 1},
        {0x08b6, 0x08c7, 1},
        {0x0904, 0x0939, 1},
        {0x093d, 0x0950, 19},
        {0x0958, 0x0961, 1},
        {0x0971, 0x0980, 1},
        {0x0985, 0x098c, 1},
        {0x098f, 0x0990, 1},
        {0x0993, 0x09a8, 1},
        {0x09aa, 0x09b0, 1},
        {0x09b2, 0x09b6, 4},
        {0x09b7, 0x09b9, 1},
        {0x09bd, 0x09ce, 17},
        {0x09dc, 0x09dd, 1},
        {0x09df, 0x09e1, 1},
        {0x09f0, 0x09f1, 1},
        {0x09fc, 0x0a05, 9},
        {0x0a06, 0x0a0a, 1},
        {0x0a0f, 0x0a10, 1},
        {0x0a13, 0x0a28, 1},
        {0x0a2a, 0x0a30, 1},
        {0x0a32, 0x0a33, 1},
        {0x0a35, 0x0a36, 1},
        {0x0a38, 0x0a39, 1},
        {0x0a59, 0x0a5c, 1},
        {0x0a5e, 0x0a72, 20},
        {0x0a73, 0x0a74, 1},
        {0x0a85, 0x0a8d, 1},
        {0x0a8f, 0x0a91, 1},
        {0x0a93, 0x0aa8, 1},
        {0x0aaa, 0x0ab0, 1},
        {0x0ab2, 0x0ab3, 1},
        {0x0ab5, 0x0ab9, 1},
        {0x0abd, 0x0ad0, 19},
        {0x0ae0, 0x0ae1, 1},
        {0x0af9, 0x0b05, 12},
        {0x0b06, 0x0b0c, 1},
        {0x0b0f, 0x0b10, 1},
        {0x0b13, 0x0b28, 1},
        {0x0b2a, 0x0b30, 1},
        {0x0b32, 0x0b33, 1},
        {0x0b35, 0x0b39, 1},
        {0x0b3d, 0x0b5c, 31},
        {0x0b5d, 0x0b5f, 2},
        {0x0b60, 0x0b61, 1},
        {0x0b71, 0x0b83, 18},
        {0x0b85, 0x0b8a, 1},
        {0x0b8e, 0x0b90, 1},
        {0x0b92, 0x0b95, 1},
        {0x0b99, 0x0b9a, 1},
        {0x0b9c, 0x0b9e, 2},
        {0x0b9f, 0x0ba3, 4},
        {0x0ba4, 0x0ba8, 4},
        {0x0ba9, 0x0baa, 1},
        {0x0bae, 0x0bb9, 1},
        {0x0bd0, 0x0c05, 53},
        {0x0c06, 0x0c0c, 1},
        {0x0c0e, 0x0c10, 1},
        {0x0c12, 0x0c28, 1},
        {0x0c2a, 0x0c39, 1},
        {0x0c3d, 0x0c58, 27},
        {0x0c59, 0x0c5a, 1},
        {0x0c60, 0x0c61, 1},
        {0x0c80, 0x0c85, 5},
        {0x0c86, 0x0c8c, 1},
        {0x0c8e, 0x0c90, 1},
        {0x0c92, 0x0ca8, 1},
        {0x0caa, 0x0cb3, 1},
        {0x0cb5, 0x0cb9, 1},
        {0x0cbd, 0x0cde, 33},
        {0x0ce0, 0x0ce1, 1},
        {0x0cf1, 0x0cf2, 1},
        {0x0d04, 0x0d0c, 1},
        {0x0d0e, 0x0d10, 1},
        {0x0d12, 0x0d3a, 1},
        {0x0d3d, 0x0d4e, 17},
        {0x0d54, 0x0d56, 1},
        {0x0d5f, 0x0d61, 1},
        {0x0d7a, 0x0d7f, 1},
        {0x0d85, 0x0d96, 1},
        {0x0d9a, 0x0db1, 1},
        {0x0db3, 0x0dbb, 1},
        {0x0dbd, 0x0dc0, 3},
        {0x0dc1, 0x0dc6, 1},
        {0x0e01, 0x0e30, 1},
        {0x0e32, 0x0e33, 1},
        {0x0e40, 0x0e46, 1},
        {0x0e81, 0x0e82, 1},
        {0x0e84, 0x0e86, 2},
        {0x0e87, 0x0e8a, 1},
        {0x0e8c, 0x0ea3, 1},
        {0x0ea5, 0x0ea7, 2},
        {0x0ea8, 0x0eb0, 1},
        {0x0eb2, 0x0eb3, 1},
        {0x0ebd, 0x0ec0, 3},
        {0x0ec1, 0x0ec4, 1},
        {0x0ec6, 0x0edc, 22},
        {0x0edd, 0x0edf, 1},
        {0x0f00, 0x0f40, 64},
        {0x0f41, 0x0f47, 1},
        {0x0f49, 0x0f6c, 1},
        {0x0f88, 0x0f8c, 1},
        {0x1000, 0x102a, 1},
        {0x103f, 0x1050, 17},
        {0x1051, 0x1055, 1},
        {0x105a, 0x105d, 1},
        {0x1061, 0x1065, 4},
        {0x1066, 0x106e, 8},
        {0x106f, 0x1070, 1},
        {0x1075, 0x1081, 1},
        {0x108e, 0x10a0, 18},
        {0x10a1, 0x10c5, 1},
        {0x10c7, 0x10cd, 6},
        {0x10d0, 0x10fa, 1},
        {0x10fc, 0x1248, 1},
        {0x124a, 0x124d, 1},
        {0x1250, 0x1256, 1},
        {0x1258, 0x125a, 2},
        {0x125b, 0x125d, 1},
        {0x1260, 0x1288, 1},
        {0x128a, 0x128d, 1},
        {0x1290, 0x12b0, 1},
        {0x12b2, 0x12b5, 1},
        {0x12b8, 0x12be, 1},
        {0x12c0, 0x12c2, 2},
        {0x12c3, 0x12c5, 1},
        {0x12c8, 0x12d6, 1},
        {0x12d8, 0x1310, 1},
        {0x1312, 0x1315, 1},
        {0x1318, 0x135a, 1},
        {0x1380, 0x138f, 1},
        {0x13a0, 0x13f5, 1},
        {0x13f8, 0x13fd, 1},
        {0x1401, 0x166c, 1},
        {0x166f, 0x167f, 1},
        {0x1681, 0x169a, 1},
        {0x16a0, 0x16ea, 1},
        {0x16f1, 0x16f8, 1},
        {0x1700, 0x170c, 1},
        {0x170e, 0x1711, 1},
        {0x1720, 0x1731, 1},
        {0x1740, 0x1751, 1},
        {0x1760, 0x176c, 1},
        {0x176e, 0x1770, 1},
        {0x1780, 0x17b3, 1},
        {0x17d7, 0x17dc, 5},
        {0x1820, 0x1878, 1},
        {0x1880, 0x1884, 1},
        {0x1887, 0x18a8, 1},
        {0x18aa, 0x18b0, 6},
        {0x18b1, 0x18f5, 1},
        {0x1900, 0x191e, 1},
        {0x1950, 0x196d, 1},
        {0x1970, 0x1974, 1},
        {0x1980, 0x19ab, 1},
        {0x19b0, 0x19c9, 1},
        {0x1a00, 0x1a16, 1},
        {0x1a20, 0x1a54, 1},
        {0x1aa7, 0x1b05, 94},
        {0x1b06, 0x1b33, 1},
        {0x1b45, 0x1b4b, 1},
        {0x1b83, 0x1ba0, 1},
        {0x1bae, 0x1baf, 1},
        {0x1bba, 0x1be5, 1},
        {0x1c00, 0x1c23, 1},
        {0x1c4d, 0x1c4f, 1},
        {0x1c5a, 0x1c7d, 1},
        {0x1c80, 0x1c88, 1},
        {0x1c90, 0x1cba, 1},
        {0x1cbd, 0x1cbf, 1},
        {0x1ce9, 0x1cec, 1},
        {0x1cee, 0x1cf3, 1},
        {0x1cf5, 0x1cf6, 1},
        {0x1cfa, 0x1d00, 6},
        {0x1d01, 0x1dbf, 1},
        {0x1e00, 0x1f15, 1},
        {0x1f18, 0x1f1d, 1},
        {0x1f20, 0x1f45, 1},
        {0x1f48, 0x1f4d, 1},
        {0x1f50, 0x1f57, 1},
        {0x1f59, 0x1f5f, 2},
        {0x1f60, 0x1f7d, 1},
        {0x1f80, 0x1fb4, 1},
        {0x1fb6, 0x1fbc, 1},
        {0x1fbe, 0x1fc2, 4},
        {0x1fc3, 0x1fc4, 1},
        {0x1fc6, 0x1fcc, 1},
        {0x1fd0, 0x1fd3, 1},
        {0x1fd6, 0x1fdb, 1},
        {0x1fe0, 0x1fec, 1},
        {0x1ff2, 0x1ff4, 1},
        {0x1ff6, 0x1ffc, 1},
        {0x2071, 0x207f, 14},
        {0x2090, 0x209c, 1},
        {0x2102, 0x2107, 5},
        {0x210a, 0x2113, 1},
        {0x2115, 0x2119, 4},
        {0x211a, 0x211d, 1},
        {0x2124, 0x212a, 2},
        {0x212b, 0x212d, 1},
        {0x212f, 0x2139, 1},
        {0x213c, 0x213f, 1},
        {0x2145, 0x2149, 1},
        {0x214e, 0x2183, 53},
        {0x2184, 0x2c00, 2684},
        {0x2c01, 0x2c2e, 1},
        {0x2c30, 0x2c5e, 1},
        {0x2c60, 0x2ce4, 1},
        {0x2ceb, 0x2cee, 1},
        {0x2cf2, 0x2cf3, 1},
        {0x2d00, 0x2d25, 1},
        {0x2d27, 0x2d2d, 6},
        {0x2d30, 0x2d67, 1},
        {0x2d6f, 0x2d80, 17},
        {0x2d81, 0x2d96, 1},
        {0x2da0, 0x2da6, 1},
        {0x2da8, 0x2dae, 1},
        {0x2db0, 0x2db6, 1},
        {0x2db8, 0x2dbe, 1},
        {0x2dc0, 0x2dc6, 1},
        {0x2dc8, 0x2dce, 1},
        {0x2dd0, 0x2dd6, 1},
        {0x2dd8, 0x2dde, 1},
        {0x2e2f, 0x3005, 470},
        {0x3006, 0x3031, 43},
        {0x3032, 0x3035, 1},
        {0x303b, 0x303c, 1},
        {0x3041, 0x3096, 1},
        {0x309d, 0x309f, 1},
        {0x30a1, 0x30fa, 1},
        {0x30fc, 0x30ff, 1},
        {0x3105, 0x312f, 1},
        {0x3131, 0x318e, 1},
        {0x31a0, 0x31bf, 1},
        {0x31f0, 0x31ff, 1},
        {0x3400, 0x4dbf, 1},
        {0x4e00, 0x9ffc, 1},
        {0xa000, 0xa48c, 1},
        {0xa4d0, 0xa4fd, 1},
        {0xa500, 0xa60c, 1},
        {0xa610, 0xa61f, 1},
        {0xa62a, 0xa62b, 1},
        {0xa640, 0xa66e, 1},
        {0xa67f, 0xa69d, 1},
        {0xa6a0, 0xa6e5, 1},
        {0xa717, 0xa71f, 1},
        {0xa722, 0xa788, 1},
        {0xa78b, 0xa7bf, 1},
        {0xa7c2, 0xa7ca, 1},
        {0xa7f5, 0xa801, 1},
        {0xa803, 0xa805, 1},
        {0xa807, 0xa80a, 1},
        {0xa80c, 0xa822, 1},
        {0xa840, 0xa873, 1},
        {0xa882, 0xa8b3, 1},
        {0xa8f2, 0xa8f7, 1},
        {0xa8fb, 0xa8fd, 2},
        {0xa8fe, 0xa90a, 12},
        {0xa90b, 0xa925, 1},
        {0xa930, 0xa946, 1},
        {0xa960, 0xa97c, 1},
        {0xa984, 0xa9b2, 1},
        {0xa9cf, 0xa9e0, 17},
        {0xa9e1, 0xa9e4, 1},
        {0xa9e6, 0xa9ef, 1},
        {0xa9fa, 0xa9fe, 1},
        {0xaa00, 0xaa28, 1},
        {0xaa40, 0xaa42, 1},
        {0xaa44, 0xaa4b, 1},
        {0xaa60, 0xaa76, 1},
        {0xaa7a, 0xaa7e, 4},
        {0xaa7f, 0xaaaf, 1},
        {0xaab1, 0xaab5, 4},
        {0xaab6, 0xaab9, 3},
        {0xaaba, 0xaabd, 1},
        {0xaac0, 0xaac2, 2},
        {0xaadb, 0xaadd, 1},
        {0xaae0, 0xaaea, 1},
        {0xaaf2, 0xaaf4, 1},
        {0xab01, 0xab06, 1},
        {0xab09, 0xab0e, 1},
        {0xab11, 0xab16, 1},
        {0xab20, 0xab26, 1},
        {0xab28, 0xab2e, 1},
        {0xab30, 0xab5a, 1},
        {0xab5c, 0xab69, 1},
        {0xab70, 0xabe2, 1},
        {0xac00, 0xd7a3, 1},
        {0xd7b0, 0xd7c6, 1},
        {0xd7cb, 0xd7fb, 1},
        {0xf900, 0xfa6d, 1},
        {0xfa70, 0xfad9, 1},
        {0xfb00, 0xfb06, 1},
        {0xfb13, 0xfb17, 1},
        {0xfb1d, 0xfb1f, 2},
        {0xfb20, 0xfb28, 1},
        {0xfb2a, 0xfb36, 1},
        {0xfb38, 0xfb3c, 1},
        {0xfb3e, 0xfb40, 2},
        {0xfb41, 0xfb43, 2},
        {0xfb44, 0xfb46, 2},
        {0xfb47, 0xfbb1, 1},
        {0xfbd3, 0xfd3d, 1},
        {0xfd50, 0xfd8f, 1},
        {0xfd92, 0xfdc7, 1},
        {0xfdf0, 0xfdfb, 1},
        {0xfe70, 0xfe74, 1},
        {0xfe76, 0xfefc, 1},
        {0xff21, 0xff3a, 1},
        {0xff41, 0xff5a, 1},
        {0xff66, 0xffbe, 1},
        {0xffc2, 0xffc7, 1},
        {0xffca, 0xffcf, 1},
        {0xffd2, 0xffd7, 1},
        {0xffda, 0xffdc, 1},
    },
    r32: {
        {0x10000, 0x1000b, 1},
        {0x1000d, 0x10026, 1},
        {0x10028, 0x1003a, 1},
        {0x1003c, 0x1003d, 1},
        {0x1003f, 0x1004d, 1},
        {0x10050, 0x1005d, 1},
        {0x10080, 0x100fa, 1},
        {0x10280, 0x1029c, 1},
        {0x102a0, 0x102d0, 1},
        {0x10300, 0x1031f, 1},
        {0x1032d, 0x10340, 1},
        {0x10342, 0x10349, 1},
        {0x10350, 0x10375, 1},
        {0x10380, 0x1039d, 1},
        {0x103a0, 0x103c3, 1},
        {0x103c8, 0x103cf, 1},
        {0x10400, 0x1049d, 1},
        {0x104b0, 0x104d3, 1},
        {0x104d8, 0x104fb, 1},
        {0x10500, 0x10527, 1},
        {0x10530, 0x10563, 1},
        {0x10600, 0x10736, 1},
        {0x10740, 0x10755, 1},
        {0x10760, 0x10767, 1},
        {0x10800, 0x10805, 1},
        {0x10808, 0x1080a, 2},
        {0x1080b, 0x10835, 1},
        {0x10837, 0x10838, 1},
        {0x1083c, 0x1083f, 3},
        {0x10840, 0x10855, 1},
        {0x10860, 0x10876, 1},
        {0x10880, 0x1089e, 1},
        {0x108e0, 0x108f2, 1},
        {0x108f4, 0x108f5, 1},
        {0x10900, 0x10915, 1},
        {0x10920, 0x10939, 1},
        {0x10980, 0x109b7, 1},
        {0x109be, 0x109bf, 1},
        {0x10a00, 0x10a10, 16},
        {0x10a11, 0x10a13, 1},
        {0x10a15, 0x10a17, 1},
        {0x10a19, 0x10a35, 1},
        {0x10a60, 0x10a7c, 1},
        {0x10a80, 0x10a9c, 1},
        {0x10ac0, 0x10ac7, 1},
        {0x10ac9, 0x10ae4, 1},
        {0x10b00, 0x10b35, 1},
        {0x10b40, 0x10b55, 1},
        {0x10b60, 0x10b72, 1},
        {0x10b80, 0x10b91, 1},
        {0x10c00, 0x10c48, 1},
        {0x10c80, 0x10cb2, 1},
        {0x10cc0, 0x10cf2, 1},
        {0x10d00, 0x10d23, 1},
        {0x10e80, 0x10ea9, 1},
        {0x10eb0, 0x10eb1, 1},
        {0x10f00, 0x10f1c, 1},
        {0x10f27, 0x10f30, 9},
        {0x10f31, 0x10f45, 1},
        {0x10fb0, 0x10fc4, 1},
        {0x10fe0, 0x10ff6, 1},
        {0x11003, 0x11037, 1},
        {0x11083, 0x110af, 1},
        {0x110d0, 0x110e8, 1},
        {0x11103, 0x11126, 1},
        {0x11144, 0x11147, 3},
        {0x11150, 0x11172, 1},
        {0x11176, 0x11183, 13},
        {0x11184, 0x111b2, 1},
        {0x111c1, 0x111c4, 1},
        {0x111da, 0x111dc, 2},
        {0x11200, 0x11211, 1},
        {0x11213, 0x1122b, 1},
        {0x11280, 0x11286, 1},
        {0x11288, 0x1128a, 2},
        {0x1128b, 0x1128d, 1},
        {0x1128f, 0x1129d, 1},
        {0x1129f, 0x112a8, 1},
        {0x112b0, 0x112de, 1},
        {0x11305, 0x1130c, 1},
        {0x1130f, 0x11310, 1},
        {0x11313, 0x11328, 1},
        {0x1132a, 0x11330, 1},
        {0x11332, 0x11333, 1},
        {0x11335, 0x11339, 1},
        {0x1133d, 0x11350, 19},
        {0x1135d, 0x11361, 1},
        {0x11400, 0x11434, 1},
        {0x11447, 0x1144a, 1},
        {0x1145f, 0x11461, 1},
        {0x11480, 0x114af, 1},
        {0x114c4, 0x114c5, 1},
        {0x114c7, 0x11580, 185},
        {0x11581, 0x115ae, 1},
        {0x115d8, 0x115db, 1},
        {0x11600, 0x1162f, 1},
        {0x11644, 0x11680, 60},
        {0x11681, 0x116aa, 1},
        {0x116b8, 0x11700, 72},
        {0x11701, 0x1171a, 1},
        {0x11800, 0x1182b, 1},
        {0x118a0, 0x118df, 1},
        {0x118ff, 0x11906, 1},
        {0x11909, 0x1190c, 3},
        {0x1190d, 0x11913, 1},
        {0x11915, 0x11916, 1},
        {0x11918, 0x1192f, 1},
        {0x1193f, 0x11941, 2},
        {0x119a0, 0x119a7, 1},
        {0x119aa, 0x119d0, 1},
        {0x119e1, 0x119e3, 2},
        {0x11a00, 0x11a0b, 11},
        {0x11a0c, 0x11a32, 1},
        {0x11a3a, 0x11a50, 22},
        {0x11a5c, 0x11a89, 1},
        {0x11a9d, 0x11ac0, 35},
        {0x11ac1, 0x11af8, 1},
        {0x11c00, 0x11c08, 1},
        {0x11c0a, 0x11c2e, 1},
        {0x11c40, 0x11c72, 50},
        {0x11c73, 0x11c8f, 1},
        {0x11d00, 0x11d06, 1},
        {0x11d08, 0x11d09, 1},
        {0x11d0b, 0x11d30, 1},
        {0x11d46, 0x11d60, 26},
        {0x11d61, 0x11d65, 1},
        {0x11d67, 0x11d68, 1},
        {0x11d6a, 0x11d89, 1},
        {0x11d98, 0x11ee0, 328},
        {0x11ee1, 0x11ef2, 1},
        {0x11fb0, 0x12000, 80},
        {0x12001, 0x12399, 1},
        {0x12480, 0x12543, 1},
        {0x13000, 0x1342e, 1},
        {0x14400, 0x14646, 1},
        {0x16800, 0x16a38, 1},
        {0x16a40, 0x16a5e, 1},
        {0x16ad0, 0x16aed, 1},
        {0x16b00, 0x16b2f, 1},
        {0x16b40, 0x16b43, 1},
        {0x16b63, 0x16b77, 1},
        {0x16b7d, 0x16b8f, 1},
        {0x16e40, 0x16e7f, 1},
        {0x16f00, 0x16f4a, 1},
        {0x16f50, 0x16f93, 67},
        {0x16f94, 0x16f9f, 1},
        {0x16fe0, 0x16fe1, 1},
        {0x16fe3, 0x17000, 29},
        {0x17001, 0x187f7, 1},
        {0x18800, 0x18cd5, 1},
        {0x18d00, 0x18d08, 1},
        {0x1b000, 0x1b11e, 1},
        {0x1b150, 0x1b152, 1},
        {0x1b164, 0x1b167, 1},
        {0x1b170, 0x1b2fb, 1},
        {0x1bc00, 0x1bc6a, 1},
        {0x1bc70, 0x1bc7c, 1},
        {0x1bc80, 0x1bc88, 1},
        {0x1bc90, 0x1bc99, 1},
        {0x1d400, 0x1d454, 1},
        {0x1d456, 0x1d49c, 1},
        {0x1d49e, 0x1d49f, 1},
        {0x1d4a2, 0x1d4a5, 3},
        {0x1d4a6, 0x1d4a9, 3},
        {0x1d4aa, 0x1d4ac, 1},
        {0x1d4ae, 0x1d4b9, 1},
        {0x1d4bb, 0x1d4bd, 2},
        {0x1d4be, 0x1d4c3, 1},
        {0x1d4c5, 0x1d505, 1},
        {0x1d507, 0x1d50a, 1},
        {0x1d50d, 0x1d514, 1},
        {0x1d516, 0x1d51c, 1},
        {0x1d51e, 0x1d539, 1},
        {0x1d53b, 0x1d53e, 1},
        {0x1d540, 0x1d544, 1},
        {0x1d546, 0x1d54a, 4},
        {0x1d54b, 0x1d550, 1},
        {0x1d552, 0x1d6a5, 1},
        {0x1d6a8, 0x1d6c0, 1},
        {0x1d6c2, 0x1d6da, 1},
        {0x1d6dc, 0x1d6fa, 1},
        {0x1d6fc, 0x1d714, 1},
        {0x1d716, 0x1d734, 1},
        {0x1d736, 0x1d74e, 1},
        {0x1d750, 0x1d76e, 1},
        {0x1d770, 0x1d788, 1},
        {0x1d78a, 0x1d7a8, 1},
        {0x1d7aa, 0x1d7c2, 1},
        {0x1d7c4, 0x1d7cb, 1},
        {0x1e100, 0x1e12c, 1},
        {0x1e137, 0x1e13d, 1},
        {0x1e14e, 0x1e2c0, 370},
        {0x1e2c1, 0x1e2eb, 1},
        {0x1e800, 0x1e8c4, 1},
        {0x1e900, 0x1e943, 1},
        {0x1e94b, 0x1ee00, 1205},
        {0x1ee01, 0x1ee03, 1},
        {0x1ee05, 0x1ee1f, 1},
        {0x1ee21, 0x1ee22, 1},
        {0x1ee24, 0x1ee27, 3},
        {0x1ee29, 0x1ee32, 1},
        {0x1ee34, 0x1ee37, 1},
        {0x1ee39, 0x1ee3b, 2},
        {0x1ee42, 0x1ee47, 5},
        {0x1ee49, 0x1ee4d, 2},
        {0x1ee4e, 0x1ee4f, 1},
        {0x1ee51, 0x1ee52, 1},
        {0x1ee54, 0x1ee57, 3},
        {0x1ee59, 0x1ee61, 2},
        {0x1ee62, 0x1ee64, 2},
        {0x1ee67, 0x1ee6a, 1},
        {0x1ee6c, 0x1ee72, 1},
        {0x1ee74, 0x1ee77, 1},
        {0x1ee79, 0x1ee7c, 1},
        {0x1ee7e, 0x1ee80, 2},
        {0x1ee81, 0x1ee89, 1},
        {0x1ee8b, 0x1ee9b, 1},
        {0x1eea1, 0x1eea3, 1},
        {0x1eea5, 0x1eea9, 1},
        {0x1eeab, 0x1eebb, 1},
        {0x20000, 0x2a6dd, 1},
        {0x2a700, 0x2b734, 1},
        {0x2b740, 0x2b81d, 1},
        {0x2b820, 0x2cea1, 1},
        {0x2ceb0, 0x2ebe0, 1},
        {0x2f800, 0x2fa1d, 1},
        {0x30000, 0x3134a, 1},
    },
    latin_offset: 6,
}

rt! {
    name: _LL,
    r16: {
        {0x0061, 0x007a, 1},
        {0x00b5, 0x00df, 42},
        {0x00e0, 0x00f6, 1},
        {0x00f8, 0x00ff, 1},
        {0x0101, 0x0137, 2},
        {0x0138, 0x0148, 2},
        {0x0149, 0x0177, 2},
        {0x017a, 0x017e, 2},
        {0x017f, 0x0180, 1},
        {0x0183, 0x0185, 2},
        {0x0188, 0x018c, 4},
        {0x018d, 0x0192, 5},
        {0x0195, 0x0199, 4},
        {0x019a, 0x019b, 1},
        {0x019e, 0x01a1, 3},
        {0x01a3, 0x01a5, 2},
        {0x01a8, 0x01aa, 2},
        {0x01ab, 0x01ad, 2},
        {0x01b0, 0x01b4, 4},
        {0x01b6, 0x01b9, 3},
        {0x01ba, 0x01bd, 3},
        {0x01be, 0x01bf, 1},
        {0x01c6, 0x01cc, 3},
        {0x01ce, 0x01dc, 2},
        {0x01dd, 0x01ef, 2},
        {0x01f0, 0x01f3, 3},
        {0x01f5, 0x01f9, 4},
        {0x01fb, 0x0233, 2},
        {0x0234, 0x0239, 1},
        {0x023c, 0x023f, 3},
        {0x0240, 0x0242, 2},
        {0x0247, 0x024f, 2},
        {0x0250, 0x0293, 1},
        {0x0295, 0x02af, 1},
        {0x0371, 0x0373, 2},
        {0x0377, 0x037b, 4},
        {0x037c, 0x037d, 1},
        {0x0390, 0x03ac, 28},
        {0x03ad, 0x03ce, 1},
        {0x03d0, 0x03d1, 1},
        {0x03d5, 0x03d7, 1},
        {0x03d9, 0x03ef, 2},
        {0x03f0, 0x03f3, 1},
        {0x03f5, 0x03fb, 3},
        {0x03fc, 0x0430, 52},
        {0x0431, 0x045f, 1},
        {0x0461, 0x0481, 2},
        {0x048b, 0x04bf, 2},
        {0x04c2, 0x04ce, 2},
        {0x04cf, 0x052f, 2},
        {0x0560, 0x0588, 1},
        {0x10d0, 0x10fa, 1},
        {0x10fd, 0x10ff, 1},
        {0x13f8, 0x13fd, 1},
        {0x1c80, 0x1c88, 1},
        {0x1d00, 0x1d2b, 1},
        {0x1d6b, 0x1d77, 1},
        {0x1d79, 0x1d9a, 1},
        {0x1e01, 0x1e95, 2},
        {0x1e96, 0x1e9d, 1},
        {0x1e9f, 0x1eff, 2},
        {0x1f00, 0x1f07, 1},
        {0x1f10, 0x1f15, 1},
        {0x1f20, 0x1f27, 1},
        {0x1f30, 0x1f37, 1},
        {0x1f40, 0x1f45, 1},
        {0x1f50, 0x1f57, 1},
        {0x1f60, 0x1f67, 1},
        {0x1f70, 0x1f7d, 1},
        {0x1f80, 0x1f87, 1},
        {0x1f90, 0x1f97, 1},
        {0x1fa0, 0x1fa7, 1},
        {0x1fb0, 0x1fb4, 1},
        {0x1fb6, 0x1fb7, 1},
        {0x1fbe, 0x1fc2, 4},
        {0x1fc3, 0x1fc4, 1},
        {0x1fc6, 0x1fc7, 1},
        {0x1fd0, 0x1fd3, 1},
        {0x1fd6, 0x1fd7, 1},
        {0x1fe0, 0x1fe7, 1},
        {0x1ff2, 0x1ff4, 1},
        {0x1ff6, 0x1ff7, 1},
        {0x210a, 0x210e, 4},
        {0x210f, 0x2113, 4},
        {0x212f, 0x2139, 5},
        {0x213c, 0x213d, 1},
        {0x2146, 0x2149, 1},
        {0x214e, 0x2184, 54},
        {0x2c30, 0x2c5e, 1},
        {0x2c61, 0x2c65, 4},
        {0x2c66, 0x2c6c, 2},
        {0x2c71, 0x2c73, 2},
        {0x2c74, 0x2c76, 2},
        {0x2c77, 0x2c7b, 1},
        {0x2c81, 0x2ce3, 2},
        {0x2ce4, 0x2cec, 8},
        {0x2cee, 0x2cf3, 5},
        {0x2d00, 0x2d25, 1},
        {0x2d27, 0x2d2d, 6},
        {0xa641, 0xa66d, 2},
        {0xa681, 0xa69b, 2},
        {0xa723, 0xa72f, 2},
        {0xa730, 0xa731, 1},
        {0xa733, 0xa771, 2},
        {0xa772, 0xa778, 1},
        {0xa77a, 0xa77c, 2},
        {0xa77f, 0xa787, 2},
        {0xa78c, 0xa78e, 2},
        {0xa791, 0xa793, 2},
        {0xa794, 0xa795, 1},
        {0xa797, 0xa7a9, 2},
        {0xa7af, 0xa7b5, 6},
        {0xa7b7, 0xa7bf, 2},
        {0xa7c3, 0xa7c8, 5},
        {0xa7ca, 0xa7f6, 44},
        {0xa7fa, 0xab30, 822},
        {0xab31, 0xab5a, 1},
        {0xab60, 0xab68, 1},
        {0xab70, 0xabbf, 1},
        {0xfb00, 0xfb06, 1},
        {0xfb13, 0xfb17, 1},
        {0xff41, 0xff5a, 1},
    },
    r32: {
        {0x10428, 0x1044f, 1},
        {0x104d8, 0x104fb, 1},
        {0x10cc0, 0x10cf2, 1},
        {0x118c0, 0x118df, 1},
        {0x16e60, 0x16e7f, 1},
        {0x1d41a, 0x1d433, 1},
        {0x1d44e, 0x1d454, 1},
        {0x1d456, 0x1d467, 1},
        {0x1d482, 0x1d49b, 1},
        {0x1d4b6, 0x1d4b9, 1},
        {0x1d4bb, 0x1d4bd, 2},
        {0x1d4be, 0x1d4c3, 1},
        {0x1d4c5, 0x1d4cf, 1},
        {0x1d4ea, 0x1d503, 1},
        {0x1d51e, 0x1d537, 1},
        {0x1d552, 0x1d56b, 1},
        {0x1d586, 0x1d59f, 1},
        {0x1d5ba, 0x1d5d3, 1},
        {0x1d5ee, 0x1d607, 1},
        {0x1d622, 0x1d63b, 1},
        {0x1d656, 0x1d66f, 1},
        {0x1d68a, 0x1d6a5, 1},
        {0x1d6c2, 0x1d6da, 1},
        {0x1d6dc, 0x1d6e1, 1},
        {0x1d6fc, 0x1d714, 1},
        {0x1d716, 0x1d71b, 1},
        {0x1d736, 0x1d74e, 1},
        {0x1d750, 0x1d755, 1},
        {0x1d770, 0x1d788, 1},
        {0x1d78a, 0x1d78f, 1},
        {0x1d7aa, 0x1d7c2, 1},
        {0x1d7c4, 0x1d7c9, 1},
        {0x1d7cb, 0x1e922, 4439},
        {0x1e923, 0x1e943, 1},
    },
    latin_offset: 4,
}

rt! {
    name: _LM,
    r16: {
        {0x02b0, 0x02c1, 1},
        {0x02c6, 0x02d1, 1},
        {0x02e0, 0x02e4, 1},
        {0x02ec, 0x02ee, 2},
        {0x0374, 0x037a, 6},
        {0x0559, 0x0640, 231},
        {0x06e5, 0x06e6, 1},
        {0x07f4, 0x07f5, 1},
        {0x07fa, 0x081a, 32},
        {0x0824, 0x0828, 4},
        {0x0971, 0x0e46, 1237},
        {0x0ec6, 0x10fc, 566},
        {0x17d7, 0x1843, 108},
        {0x1aa7, 0x1c78, 465},
        {0x1c79, 0x1c7d, 1},
        {0x1d2c, 0x1d6a, 1},
        {0x1d78, 0x1d9b, 35},
        {0x1d9c, 0x1dbf, 1},
        {0x2071, 0x207f, 14},
        {0x2090, 0x209c, 1},
        {0x2c7c, 0x2c7d, 1},
        {0x2d6f, 0x2e2f, 192},
        {0x3005, 0x3031, 44},
        {0x3032, 0x3035, 1},
        {0x303b, 0x309d, 98},
        {0x309e, 0x30fc, 94},
        {0x30fd, 0x30fe, 1},
        {0xa015, 0xa4f8, 1251},
        {0xa4f9, 0xa4fd, 1},
        {0xa60c, 0xa67f, 115},
        {0xa69c, 0xa69d, 1},
        {0xa717, 0xa71f, 1},
        {0xa770, 0xa788, 24},
        {0xa7f8, 0xa7f9, 1},
        {0xa9cf, 0xa9e6, 23},
        {0xaa70, 0xaadd, 109},
        {0xaaf3, 0xaaf4, 1},
        {0xab5c, 0xab5f, 1},
        {0xab69, 0xff70, 21511},
        {0xff9e, 0xff9f, 1},
    },
    r32: {
        {0x16b40, 0x16b43, 1},
        {0x16f93, 0x16f9f, 1},
        {0x16fe0, 0x16fe1, 1},
        {0x16fe3, 0x1e137, 29012},
        {0x1e138, 0x1e13d, 1},
        {0x1e94b, 0x1e94b, 1},
    }
}

rt! {
    name: _LO,
    r16: {
        {0x00aa, 0x00ba, 16},
        {0x01bb, 0x01c0, 5},
        {0x01c1, 0x01c3, 1},
        {0x0294, 0x05d0, 828},
        {0x05d1, 0x05ea, 1},
        {0x05ef, 0x05f2, 1},
        {0x0620, 0x063f, 1},
        {0x0641, 0x064a, 1},
        {0x066e, 0x066f, 1},
        {0x0671, 0x06d3, 1},
        {0x06d5, 0x06ee, 25},
        {0x06ef, 0x06fa, 11},
        {0x06fb, 0x06fc, 1},
        {0x06ff, 0x0710, 17},
        {0x0712, 0x072f, 1},
        {0x074d, 0x07a5, 1},
        {0x07b1, 0x07ca, 25},
        {0x07cb, 0x07ea, 1},
        {0x0800, 0x0815, 1},
        {0x0840, 0x0858, 1},
        {0x0860, 0x086a, 1},
        {0x08a0, 0x08b4, 1},
        {0x08b6, 0x08c7, 1},
        {0x0904, 0x0939, 1},
        {0x093d, 0x0950, 19},
        {0x0958, 0x0961, 1},
        {0x0972, 0x0980, 1},
        {0x0985, 0x098c, 1},
        {0x098f, 0x0990, 1},
        {0x0993, 0x09a8, 1},
        {0x09aa, 0x09b0, 1},
        {0x09b2, 0x09b6, 4},
        {0x09b7, 0x09b9, 1},
        {0x09bd, 0x09ce, 17},
        {0x09dc, 0x09dd, 1},
        {0x09df, 0x09e1, 1},
        {0x09f0, 0x09f1, 1},
        {0x09fc, 0x0a05, 9},
        {0x0a06, 0x0a0a, 1},
        {0x0a0f, 0x0a10, 1},
        {0x0a13, 0x0a28, 1},
        {0x0a2a, 0x0a30, 1},
        {0x0a32, 0x0a33, 1},
        {0x0a35, 0x0a36, 1},
        {0x0a38, 0x0a39, 1},
        {0x0a59, 0x0a5c, 1},
        {0x0a5e, 0x0a72, 20},
        {0x0a73, 0x0a74, 1},
        {0x0a85, 0x0a8d, 1},
        {0x0a8f, 0x0a91, 1},
        {0x0a93, 0x0aa8, 1},
        {0x0aaa, 0x0ab0, 1},
        {0x0ab2, 0x0ab3, 1},
        {0x0ab5, 0x0ab9, 1},
        {0x0abd, 0x0ad0, 19},
        {0x0ae0, 0x0ae1, 1},
        {0x0af9, 0x0b05, 12},
        {0x0b06, 0x0b0c, 1},
        {0x0b0f, 0x0b10, 1},
        {0x0b13, 0x0b28, 1},
        {0x0b2a, 0x0b30, 1},
        {0x0b32, 0x0b33, 1},
        {0x0b35, 0x0b39, 1},
        {0x0b3d, 0x0b5c, 31},
        {0x0b5d, 0x0b5f, 2},
        {0x0b60, 0x0b61, 1},
        {0x0b71, 0x0b83, 18},
        {0x0b85, 0x0b8a, 1},
        {0x0b8e, 0x0b90, 1},
        {0x0b92, 0x0b95, 1},
        {0x0b99, 0x0b9a, 1},
        {0x0b9c, 0x0b9e, 2},
        {0x0b9f, 0x0ba3, 4},
        {0x0ba4, 0x0ba8, 4},
        {0x0ba9, 0x0baa, 1},
        {0x0bae, 0x0bb9, 1},
        {0x0bd0, 0x0c05, 53},
        {0x0c06, 0x0c0c, 1},
        {0x0c0e, 0x0c10, 1},
        {0x0c12, 0x0c28, 1},
        {0x0c2a, 0x0c39, 1},
        {0x0c3d, 0x0c58, 27},
        {0x0c59, 0x0c5a, 1},
        {0x0c60, 0x0c61, 1},
        {0x0c80, 0x0c85, 5},
        {0x0c86, 0x0c8c, 1},
        {0x0c8e, 0x0c90, 1},
        {0x0c92, 0x0ca8, 1},
        {0x0caa, 0x0cb3, 1},
        {0x0cb5, 0x0cb9, 1},
        {0x0cbd, 0x0cde, 33},
        {0x0ce0, 0x0ce1, 1},
        {0x0cf1, 0x0cf2, 1},
        {0x0d04, 0x0d0c, 1},
        {0x0d0e, 0x0d10, 1},
        {0x0d12, 0x0d3a, 1},
        {0x0d3d, 0x0d4e, 17},
        {0x0d54, 0x0d56, 1},
        {0x0d5f, 0x0d61, 1},
        {0x0d7a, 0x0d7f, 1},
        {0x0d85, 0x0d96, 1},
        {0x0d9a, 0x0db1, 1},
        {0x0db3, 0x0dbb, 1},
        {0x0dbd, 0x0dc0, 3},
        {0x0dc1, 0x0dc6, 1},
        {0x0e01, 0x0e30, 1},
        {0x0e32, 0x0e33, 1},
        {0x0e40, 0x0e45, 1},
        {0x0e81, 0x0e82, 1},
        {0x0e84, 0x0e86, 2},
        {0x0e87, 0x0e8a, 1},
        {0x0e8c, 0x0ea3, 1},
        {0x0ea5, 0x0ea7, 2},
        {0x0ea8, 0x0eb0, 1},
        {0x0eb2, 0x0eb3, 1},
        {0x0ebd, 0x0ec0, 3},
        {0x0ec1, 0x0ec4, 1},
        {0x0edc, 0x0edf, 1},
        {0x0f00, 0x0f40, 64},
        {0x0f41, 0x0f47, 1},
        {0x0f49, 0x0f6c, 1},
        {0x0f88, 0x0f8c, 1},
        {0x1000, 0x102a, 1},
        {0x103f, 0x1050, 17},
        {0x1051, 0x1055, 1},
        {0x105a, 0x105d, 1},
        {0x1061, 0x1065, 4},
        {0x1066, 0x106e, 8},
        {0x106f, 0x1070, 1},
        {0x1075, 0x1081, 1},
        {0x108e, 0x1100, 114},
        {0x1101, 0x1248, 1},
        {0x124a, 0x124d, 1},
        {0x1250, 0x1256, 1},
        {0x1258, 0x125a, 2},
        {0x125b, 0x125d, 1},
        {0x1260, 0x1288, 1},
        {0x128a, 0x128d, 1},
        {0x1290, 0x12b0, 1},
        {0x12b2, 0x12b5, 1},
        {0x12b8, 0x12be, 1},
        {0x12c0, 0x12c2, 2},
        {0x12c3, 0x12c5, 1},
        {0x12c8, 0x12d6, 1},
        {0x12d8, 0x1310, 1},
        {0x1312, 0x1315, 1},
        {0x1318, 0x135a, 1},
        {0x1380, 0x138f, 1},
        {0x1401, 0x166c, 1},
        {0x166f, 0x167f, 1},
        {0x1681, 0x169a, 1},
        {0x16a0, 0x16ea, 1},
        {0x16f1, 0x16f8, 1},
        {0x1700, 0x170c, 1},
        {0x170e, 0x1711, 1},
        {0x1720, 0x1731, 1},
        {0x1740, 0x1751, 1},
        {0x1760, 0x176c, 1},
        {0x176e, 0x1770, 1},
        {0x1780, 0x17b3, 1},
        {0x17dc, 0x1820, 68},
        {0x1821, 0x1842, 1},
        {0x1844, 0x1878, 1},
        {0x1880, 0x1884, 1},
        {0x1887, 0x18a8, 1},
        {0x18aa, 0x18b0, 6},
        {0x18b1, 0x18f5, 1},
        {0x1900, 0x191e, 1},
        {0x1950, 0x196d, 1},
        {0x1970, 0x1974, 1},
        {0x1980, 0x19ab, 1},
        {0x19b0, 0x19c9, 1},
        {0x1a00, 0x1a16, 1},
        {0x1a20, 0x1a54, 1},
        {0x1b05, 0x1b33, 1},
        {0x1b45, 0x1b4b, 1},
        {0x1b83, 0x1ba0, 1},
        {0x1bae, 0x1baf, 1},
        {0x1bba, 0x1be5, 1},
        {0x1c00, 0x1c23, 1},
        {0x1c4d, 0x1c4f, 1},
        {0x1c5a, 0x1c77, 1},
        {0x1ce9, 0x1cec, 1},
        {0x1cee, 0x1cf3, 1},
        {0x1cf5, 0x1cf6, 1},
        {0x1cfa, 0x2135, 1083},
        {0x2136, 0x2138, 1},
        {0x2d30, 0x2d67, 1},
        {0x2d80, 0x2d96, 1},
        {0x2da0, 0x2da6, 1},
        {0x2da8, 0x2dae, 1},
        {0x2db0, 0x2db6, 1},
        {0x2db8, 0x2dbe, 1},
        {0x2dc0, 0x2dc6, 1},
        {0x2dc8, 0x2dce, 1},
        {0x2dd0, 0x2dd6, 1},
        {0x2dd8, 0x2dde, 1},
        {0x3006, 0x303c, 54},
        {0x3041, 0x3096, 1},
        {0x309f, 0x30a1, 2},
        {0x30a2, 0x30fa, 1},
        {0x30ff, 0x3105, 6},
        {0x3106, 0x312f, 1},
        {0x3131, 0x318e, 1},
        {0x31a0, 0x31bf, 1},
        {0x31f0, 0x31ff, 1},
        {0x3400, 0x4dbf, 1},
        {0x4e00, 0x9ffc, 1},
        {0xa000, 0xa014, 1},
        {0xa016, 0xa48c, 1},
        {0xa4d0, 0xa4f7, 1},
        {0xa500, 0xa60b, 1},
        {0xa610, 0xa61f, 1},
        {0xa62a, 0xa62b, 1},
        {0xa66e, 0xa6a0, 50},
        {0xa6a1, 0xa6e5, 1},
        {0xa78f, 0xa7f7, 104},
        {0xa7fb, 0xa801, 1},
        {0xa803, 0xa805, 1},
        {0xa807, 0xa80a, 1},
        {0xa80c, 0xa822, 1},
        {0xa840, 0xa873, 1},
        {0xa882, 0xa8b3, 1},
        {0xa8f2, 0xa8f7, 1},
        {0xa8fb, 0xa8fd, 2},
        {0xa8fe, 0xa90a, 12},
        {0xa90b, 0xa925, 1},
        {0xa930, 0xa946, 1},
        {0xa960, 0xa97c, 1},
        {0xa984, 0xa9b2, 1},
        {0xa9e0, 0xa9e4, 1},
        {0xa9e7, 0xa9ef, 1},
        {0xa9fa, 0xa9fe, 1},
        {0xaa00, 0xaa28, 1},
        {0xaa40, 0xaa42, 1},
        {0xaa44, 0xaa4b, 1},
        {0xaa60, 0xaa6f, 1},
        {0xaa71, 0xaa76, 1},
        {0xaa7a, 0xaa7e, 4},
        {0xaa7f, 0xaaaf, 1},
        {0xaab1, 0xaab5, 4},
        {0xaab6, 0xaab9, 3},
        {0xaaba, 0xaabd, 1},
        {0xaac0, 0xaac2, 2},
        {0xaadb, 0xaadc, 1},
        {0xaae0, 0xaaea, 1},
        {0xaaf2, 0xab01, 15},
        {0xab02, 0xab06, 1},
        {0xab09, 0xab0e, 1},
        {0xab11, 0xab16, 1},
        {0xab20, 0xab26, 1},
        {0xab28, 0xab2e, 1},
        {0xabc0, 0xabe2, 1},
        {0xac00, 0xd7a3, 1},
        {0xd7b0, 0xd7c6, 1},
        {0xd7cb, 0xd7fb, 1},
        {0xf900, 0xfa6d, 1},
        {0xfa70, 0xfad9, 1},
        {0xfb1d, 0xfb1f, 2},
        {0xfb20, 0xfb28, 1},
        {0xfb2a, 0xfb36, 1},
        {0xfb38, 0xfb3c, 1},
        {0xfb3e, 0xfb40, 2},
        {0xfb41, 0xfb43, 2},
        {0xfb44, 0xfb46, 2},
        {0xfb47, 0xfbb1, 1},
        {0xfbd3, 0xfd3d, 1},
        {0xfd50, 0xfd8f, 1},
        {0xfd92, 0xfdc7, 1},
        {0xfdf0, 0xfdfb, 1},
        {0xfe70, 0xfe74, 1},
        {0xfe76, 0xfefc, 1},
        {0xff66, 0xff6f, 1},
        {0xff71, 0xff9d, 1},
        {0xffa0, 0xffbe, 1},
        {0xffc2, 0xffc7, 1},
        {0xffca, 0xffcf, 1},
        {0xffd2, 0xffd7, 1},
        {0xffda, 0xffdc, 1},
    },
    r32: {
        {0x10000, 0x1000b, 1},
        {0x1000d, 0x10026, 1},
        {0x10028, 0x1003a, 1},
        {0x1003c, 0x1003d, 1},
        {0x1003f, 0x1004d, 1},
        {0x10050, 0x1005d, 1},
        {0x10080, 0x100fa, 1},
        {0x10280, 0x1029c, 1},
        {0x102a0, 0x102d0, 1},
        {0x10300, 0x1031f, 1},
        {0x1032d, 0x10340, 1},
        {0x10342, 0x10349, 1},
        {0x10350, 0x10375, 1},
        {0x10380, 0x1039d, 1},
        {0x103a0, 0x103c3, 1},
        {0x103c8, 0x103cf, 1},
        {0x10450, 0x1049d, 1},
        {0x10500, 0x10527, 1},
        {0x10530, 0x10563, 1},
        {0x10600, 0x10736, 1},
        {0x10740, 0x10755, 1},
        {0x10760, 0x10767, 1},
        {0x10800, 0x10805, 1},
        {0x10808, 0x1080a, 2},
        {0x1080b, 0x10835, 1},
        {0x10837, 0x10838, 1},
        {0x1083c, 0x1083f, 3},
        {0x10840, 0x10855, 1},
        {0x10860, 0x10876, 1},
        {0x10880, 0x1089e, 1},
        {0x108e0, 0x108f2, 1},
        {0x108f4, 0x108f5, 1},
        {0x10900, 0x10915, 1},
        {0x10920, 0x10939, 1},
        {0x10980, 0x109b7, 1},
        {0x109be, 0x109bf, 1},
        {0x10a00, 0x10a10, 16},
        {0x10a11, 0x10a13, 1},
        {0x10a15, 0x10a17, 1},
        {0x10a19, 0x10a35, 1},
        {0x10a60, 0x10a7c, 1},
        {0x10a80, 0x10a9c, 1},
        {0x10ac0, 0x10ac7, 1},
        {0x10ac9, 0x10ae4, 1},
        {0x10b00, 0x10b35, 1},
        {0x10b40, 0x10b55, 1},
        {0x10b60, 0x10b72, 1},
        {0x10b80, 0x10b91, 1},
        {0x10c00, 0x10c48, 1},
        {0x10d00, 0x10d23, 1},
        {0x10e80, 0x10ea9, 1},
        {0x10eb0, 0x10eb1, 1},
        {0x10f00, 0x10f1c, 1},
        {0x10f27, 0x10f30, 9},
        {0x10f31, 0x10f45, 1},
        {0x10fb0, 0x10fc4, 1},
        {0x10fe0, 0x10ff6, 1},
        {0x11003, 0x11037, 1},
        {0x11083, 0x110af, 1},
        {0x110d0, 0x110e8, 1},
        {0x11103, 0x11126, 1},
        {0x11144, 0x11147, 3},
        {0x11150, 0x11172, 1},
        {0x11176, 0x11183, 13},
        {0x11184, 0x111b2, 1},
        {0x111c1, 0x111c4, 1},
        {0x111da, 0x111dc, 2},
        {0x11200, 0x11211, 1},
        {0x11213, 0x1122b, 1},
        {0x11280, 0x11286, 1},
        {0x11288, 0x1128a, 2},
        {0x1128b, 0x1128d, 1},
        {0x1128f, 0x1129d, 1},
        {0x1129f, 0x112a8, 1},
        {0x112b0, 0x112de, 1},
        {0x11305, 0x1130c, 1},
        {0x1130f, 0x11310, 1},
        {0x11313, 0x11328, 1},
        {0x1132a, 0x11330, 1},
        {0x11332, 0x11333, 1},
        {0x11335, 0x11339, 1},
        {0x1133d, 0x11350, 19},
        {0x1135d, 0x11361, 1},
        {0x11400, 0x11434, 1},
        {0x11447, 0x1144a, 1},
        {0x1145f, 0x11461, 1},
        {0x11480, 0x114af, 1},
        {0x114c4, 0x114c5, 1},
        {0x114c7, 0x11580, 185},
        {0x11581, 0x115ae, 1},
        {0x115d8, 0x115db, 1},
        {0x11600, 0x1162f, 1},
        {0x11644, 0x11680, 60},
        {0x11681, 0x116aa, 1},
        {0x116b8, 0x11700, 72},
        {0x11701, 0x1171a, 1},
        {0x11800, 0x1182b, 1},
        {0x118ff, 0x11906, 1},
        {0x11909, 0x1190c, 3},
        {0x1190d, 0x11913, 1},
        {0x11915, 0x11916, 1},
        {0x11918, 0x1192f, 1},
        {0x1193f, 0x11941, 2},
        {0x119a0, 0x119a7, 1},
        {0x119aa, 0x119d0, 1},
        {0x119e1, 0x119e3, 2},
        {0x11a00, 0x11a0b, 11},
        {0x11a0c, 0x11a32, 1},
        {0x11a3a, 0x11a50, 22},
        {0x11a5c, 0x11a89, 1},
        {0x11a9d, 0x11ac0, 35},
        {0x11ac1, 0x11af8, 1},
        {0x11c00, 0x11c08, 1},
        {0x11c0a, 0x11c2e, 1},
        {0x11c40, 0x11c72, 50},
        {0x11c73, 0x11c8f, 1},
        {0x11d00, 0x11d06, 1},
        {0x11d08, 0x11d09, 1},
        {0x11d0b, 0x11d30, 1},
        {0x11d46, 0x11d60, 26},
        {0x11d61, 0x11d65, 1},
        {0x11d67, 0x11d68, 1},
        {0x11d6a, 0x11d89, 1},
        {0x11d98, 0x11ee0, 328},
        {0x11ee1, 0x11ef2, 1},
        {0x11fb0, 0x12000, 80},
        {0x12001, 0x12399, 1},
        {0x12480, 0x12543, 1},
        {0x13000, 0x1342e, 1},
        {0x14400, 0x14646, 1},
        {0x16800, 0x16a38, 1},
        {0x16a40, 0x16a5e, 1},
        {0x16ad0, 0x16aed, 1},
        {0x16b00, 0x16b2f, 1},
        {0x16b63, 0x16b77, 1},
        {0x16b7d, 0x16b8f, 1},
        {0x16f00, 0x16f4a, 1},
        {0x16f50, 0x17000, 176},
        {0x17001, 0x187f7, 1},
        {0x18800, 0x18cd5, 1},
        {0x18d00, 0x18d08, 1},
        {0x1b000, 0x1b11e, 1},
        {0x1b150, 0x1b152, 1},
        {0x1b164, 0x1b167, 1},
        {0x1b170, 0x1b2fb, 1},
        {0x1bc00, 0x1bc6a, 1},
        {0x1bc70, 0x1bc7c, 1},
        {0x1bc80, 0x1bc88, 1},
        {0x1bc90, 0x1bc99, 1},
        {0x1e100, 0x1e12c, 1},
        {0x1e14e, 0x1e2c0, 370},
        {0x1e2c1, 0x1e2eb, 1},
        {0x1e800, 0x1e8c4, 1},
        {0x1ee00, 0x1ee03, 1},
        {0x1ee05, 0x1ee1f, 1},
        {0x1ee21, 0x1ee22, 1},
        {0x1ee24, 0x1ee27, 3},
        {0x1ee29, 0x1ee32, 1},
        {0x1ee34, 0x1ee37, 1},
        {0x1ee39, 0x1ee3b, 2},
        {0x1ee42, 0x1ee47, 5},
        {0x1ee49, 0x1ee4d, 2},
        {0x1ee4e, 0x1ee4f, 1},
        {0x1ee51, 0x1ee52, 1},
        {0x1ee54, 0x1ee57, 3},
        {0x1ee59, 0x1ee61, 2},
        {0x1ee62, 0x1ee64, 2},
        {0x1ee67, 0x1ee6a, 1},
        {0x1ee6c, 0x1ee72, 1},
        {0x1ee74, 0x1ee77, 1},
        {0x1ee79, 0x1ee7c, 1},
        {0x1ee7e, 0x1ee80, 2},
        {0x1ee81, 0x1ee89, 1},
        {0x1ee8b, 0x1ee9b, 1},
        {0x1eea1, 0x1eea3, 1},
        {0x1eea5, 0x1eea9, 1},
        {0x1eeab, 0x1eebb, 1},
        {0x20000, 0x2a6dd, 1},
        {0x2a700, 0x2b734, 1},
        {0x2b740, 0x2b81d, 1},
        {0x2b820, 0x2cea1, 1},
        {0x2ceb0, 0x2ebe0, 1},
        {0x2f800, 0x2fa1d, 1},
        {0x30000, 0x3134a, 1},
    },
    latin_offset: 1,
}

rt! {
    name: _LT,
    r16: {
        {0x01c5, 0x01cb, 3},
        {0x01f2, 0x1f88, 7574},
        {0x1f89, 0x1f8f, 1},
        {0x1f98, 0x1f9f, 1},
        {0x1fa8, 0x1faf, 1},
        {0x1fbc, 0x1fcc, 16},
        {0x1ffc, 0x1ffc, 1},
    }
}

rt! {
    name: _LU,
    r16: {
        {0x0041, 0x005a, 1},
        {0x00c0, 0x00d6, 1},
        {0x00d8, 0x00de, 1},
        {0x0100, 0x0136, 2},
        {0x0139, 0x0147, 2},
        {0x014a, 0x0178, 2},
        {0x0179, 0x017d, 2},
        {0x0181, 0x0182, 1},
        {0x0184, 0x0186, 2},
        {0x0187, 0x0189, 2},
        {0x018a, 0x018b, 1},
        {0x018e, 0x0191, 1},
        {0x0193, 0x0194, 1},
        {0x0196, 0x0198, 1},
        {0x019c, 0x019d, 1},
        {0x019f, 0x01a0, 1},
        {0x01a2, 0x01a6, 2},
        {0x01a7, 0x01a9, 2},
        {0x01ac, 0x01ae, 2},
        {0x01af, 0x01b1, 2},
        {0x01b2, 0x01b3, 1},
        {0x01b5, 0x01b7, 2},
        {0x01b8, 0x01bc, 4},
        {0x01c4, 0x01cd, 3},
        {0x01cf, 0x01db, 2},
        {0x01de, 0x01ee, 2},
        {0x01f1, 0x01f4, 3},
        {0x01f6, 0x01f8, 1},
        {0x01fa, 0x0232, 2},
        {0x023a, 0x023b, 1},
        {0x023d, 0x023e, 1},
        {0x0241, 0x0243, 2},
        {0x0244, 0x0246, 1},
        {0x0248, 0x024e, 2},
        {0x0370, 0x0372, 2},
        {0x0376, 0x037f, 9},
        {0x0386, 0x0388, 2},
        {0x0389, 0x038a, 1},
        {0x038c, 0x038e, 2},
        {0x038f, 0x0391, 2},
        {0x0392, 0x03a1, 1},
        {0x03a3, 0x03ab, 1},
        {0x03cf, 0x03d2, 3},
        {0x03d3, 0x03d4, 1},
        {0x03d8, 0x03ee, 2},
        {0x03f4, 0x03f7, 3},
        {0x03f9, 0x03fa, 1},
        {0x03fd, 0x042f, 1},
        {0x0460, 0x0480, 2},
        {0x048a, 0x04c0, 2},
        {0x04c1, 0x04cd, 2},
        {0x04d0, 0x052e, 2},
        {0x0531, 0x0556, 1},
        {0x10a0, 0x10c5, 1},
        {0x10c7, 0x10cd, 6},
        {0x13a0, 0x13f5, 1},
        {0x1c90, 0x1cba, 1},
        {0x1cbd, 0x1cbf, 1},
        {0x1e00, 0x1e94, 2},
        {0x1e9e, 0x1efe, 2},
        {0x1f08, 0x1f0f, 1},
        {0x1f18, 0x1f1d, 1},
        {0x1f28, 0x1f2f, 1},
        {0x1f38, 0x1f3f, 1},
        {0x1f48, 0x1f4d, 1},
        {0x1f59, 0x1f5f, 2},
        {0x1f68, 0x1f6f, 1},
        {0x1fb8, 0x1fbb, 1},
        {0x1fc8, 0x1fcb, 1},
        {0x1fd8, 0x1fdb, 1},
        {0x1fe8, 0x1fec, 1},
        {0x1ff8, 0x1ffb, 1},
        {0x2102, 0x2107, 5},
        {0x210b, 0x210d, 1},
        {0x2110, 0x2112, 1},
        {0x2115, 0x2119, 4},
        {0x211a, 0x211d, 1},
        {0x2124, 0x212a, 2},
        {0x212b, 0x212d, 1},
        {0x2130, 0x2133, 1},
        {0x213e, 0x213f, 1},
        {0x2145, 0x2183, 62},
        {0x2c00, 0x2c2e, 1},
        {0x2c60, 0x2c62, 2},
        {0x2c63, 0x2c64, 1},
        {0x2c67, 0x2c6d, 2},
        {0x2c6e, 0x2c70, 1},
        {0x2c72, 0x2c75, 3},
        {0x2c7e, 0x2c80, 1},
        {0x2c82, 0x2ce2, 2},
        {0x2ceb, 0x2ced, 2},
        {0x2cf2, 0xa640, 31054},
        {0xa642, 0xa66c, 2},
        {0xa680, 0xa69a, 2},
        {0xa722, 0xa72e, 2},
        {0xa732, 0xa76e, 2},
        {0xa779, 0xa77d, 2},
        {0xa77e, 0xa786, 2},
        {0xa78b, 0xa78d, 2},
        {0xa790, 0xa792, 2},
        {0xa796, 0xa7aa, 2},
        {0xa7ab, 0xa7ae, 1},
        {0xa7b0, 0xa7b4, 1},
        {0xa7b6, 0xa7be, 2},
        {0xa7c2, 0xa7c4, 2},
        {0xa7c5, 0xa7c7, 1},
        {0xa7c9, 0xa7f5, 44},
        {0xff21, 0xff3a, 1},
    },
    r32: {
        {0x10400, 0x10427, 1},
        {0x104b0, 0x104d3, 1},
        {0x10c80, 0x10cb2, 1},
        {0x118a0, 0x118bf, 1},
        {0x16e40, 0x16e5f, 1},
        {0x1d400, 0x1d419, 1},
        {0x1d434, 0x1d44d, 1},
        {0x1d468, 0x1d481, 1},
        {0x1d49c, 0x1d49e, 2},
        {0x1d49f, 0x1d4a5, 3},
        {0x1d4a6, 0x1d4a9, 3},
        {0x1d4aa, 0x1d4ac, 1},
        {0x1d4ae, 0x1d4b5, 1},
        {0x1d4d0, 0x1d4e9, 1},
        {0x1d504, 0x1d505, 1},
        {0x1d507, 0x1d50a, 1},
        {0x1d50d, 0x1d514, 1},
        {0x1d516, 0x1d51c, 1},
        {0x1d538, 0x1d539, 1},
        {0x1d53b, 0x1d53e, 1},
        {0x1d540, 0x1d544, 1},
        {0x1d546, 0x1d54a, 4},
        {0x1d54b, 0x1d550, 1},
        {0x1d56c, 0x1d585, 1},
        {0x1d5a0, 0x1d5b9, 1},
        {0x1d5d4, 0x1d5ed, 1},
        {0x1d608, 0x1d621, 1},
        {0x1d63c, 0x1d655, 1},
        {0x1d670, 0x1d689, 1},
        {0x1d6a8, 0x1d6c0, 1},
        {0x1d6e2, 0x1d6fa, 1},
        {0x1d71c, 0x1d734, 1},
        {0x1d756, 0x1d76e, 1},
        {0x1d790, 0x1d7a8, 1},
        {0x1d7ca, 0x1e900, 4406},
        {0x1e901, 0x1e921, 1},
    },
    latin_offset: 3,
}

rt! {
    name: _M,
    r16: {
        {0x0300, 0x036f, 1},
        {0x0483, 0x0489, 1},
        {0x0591, 0x05bd, 1},
        {0x05bf, 0x05c1, 2},
        {0x05c2, 0x05c4, 2},
        {0x05c5, 0x05c7, 2},
        {0x0610, 0x061a, 1},
        {0x064b, 0x065f, 1},
        {0x0670, 0x06d6, 102},
        {0x06d7, 0x06dc, 1},
        {0x06df, 0x06e4, 1},
        {0x06e7, 0x06e8, 1},
        {0x06ea, 0x06ed, 1},
        {0x0711, 0x0730, 31},
        {0x0731, 0x074a, 1},
        {0x07a6, 0x07b0, 1},
        {0x07eb, 0x07f3, 1},
        {0x07fd, 0x0816, 25},
        {0x0817, 0x0819, 1},
        {0x081b, 0x0823, 1},
        {0x0825, 0x0827, 1},
        {0x0829, 0x082d, 1},
        {0x0859, 0x085b, 1},
        {0x08d3, 0x08e1, 1},
        {0x08e3, 0x0903, 1},
        {0x093a, 0x093c, 1},
        {0x093e, 0x094f, 1},
        {0x0951, 0x0957, 1},
        {0x0962, 0x0963, 1},
        {0x0981, 0x0983, 1},
        {0x09bc, 0x09be, 2},
        {0x09bf, 0x09c4, 1},
        {0x09c7, 0x09c8, 1},
        {0x09cb, 0x09cd, 1},
        {0x09d7, 0x09e2, 11},
        {0x09e3, 0x09fe, 27},
        {0x0a01, 0x0a03, 1},
        {0x0a3c, 0x0a3e, 2},
        {0x0a3f, 0x0a42, 1},
        {0x0a47, 0x0a48, 1},
        {0x0a4b, 0x0a4d, 1},
        {0x0a51, 0x0a70, 31},
        {0x0a71, 0x0a75, 4},
        {0x0a81, 0x0a83, 1},
        {0x0abc, 0x0abe, 2},
        {0x0abf, 0x0ac5, 1},
        {0x0ac7, 0x0ac9, 1},
        {0x0acb, 0x0acd, 1},
        {0x0ae2, 0x0ae3, 1},
        {0x0afa, 0x0aff, 1},
        {0x0b01, 0x0b03, 1},
        {0x0b3c, 0x0b3e, 2},
        {0x0b3f, 0x0b44, 1},
        {0x0b47, 0x0b48, 1},
        {0x0b4b, 0x0b4d, 1},
        {0x0b55, 0x0b57, 1},
        {0x0b62, 0x0b63, 1},
        {0x0b82, 0x0bbe, 60},
        {0x0bbf, 0x0bc2, 1},
        {0x0bc6, 0x0bc8, 1},
        {0x0bca, 0x0bcd, 1},
        {0x0bd7, 0x0c00, 41},
        {0x0c01, 0x0c04, 1},
        {0x0c3e, 0x0c44, 1},
        {0x0c46, 0x0c48, 1},
        {0x0c4a, 0x0c4d, 1},
        {0x0c55, 0x0c56, 1},
        {0x0c62, 0x0c63, 1},
        {0x0c81, 0x0c83, 1},
        {0x0cbc, 0x0cbe, 2},
        {0x0cbf, 0x0cc4, 1},
        {0x0cc6, 0x0cc8, 1},
        {0x0cca, 0x0ccd, 1},
        {0x0cd5, 0x0cd6, 1},
        {0x0ce2, 0x0ce3, 1},
        {0x0d00, 0x0d03, 1},
        {0x0d3b, 0x0d3c, 1},
        {0x0d3e, 0x0d44, 1},
        {0x0d46, 0x0d48, 1},
        {0x0d4a, 0x0d4d, 1},
        {0x0d57, 0x0d62, 11},
        {0x0d63, 0x0d81, 30},
        {0x0d82, 0x0d83, 1},
        {0x0dca, 0x0dcf, 5},
        {0x0dd0, 0x0dd4, 1},
        {0x0dd6, 0x0dd8, 2},
        {0x0dd9, 0x0ddf, 1},
        {0x0df2, 0x0df3, 1},
        {0x0e31, 0x0e34, 3},
        {0x0e35, 0x0e3a, 1},
        {0x0e47, 0x0e4e, 1},
        {0x0eb1, 0x0eb4, 3},
        {0x0eb5, 0x0ebc, 1},
        {0x0ec8, 0x0ecd, 1},
        {0x0f18, 0x0f19, 1},
        {0x0f35, 0x0f39, 2},
        {0x0f3e, 0x0f3f, 1},
        {0x0f71, 0x0f84, 1},
        {0x0f86, 0x0f87, 1},
        {0x0f8d, 0x0f97, 1},
        {0x0f99, 0x0fbc, 1},
        {0x0fc6, 0x102b, 101},
        {0x102c, 0x103e, 1},
        {0x1056, 0x1059, 1},
        {0x105e, 0x1060, 1},
        {0x1062, 0x1064, 1},
        {0x1067, 0x106d, 1},
        {0x1071, 0x1074, 1},
        {0x1082, 0x108d, 1},
        {0x108f, 0x109a, 11},
        {0x109b, 0x109d, 1},
        {0x135d, 0x135f, 1},
        {0x1712, 0x1714, 1},
        {0x1732, 0x1734, 1},
        {0x1752, 0x1753, 1},
        {0x1772, 0x1773, 1},
        {0x17b4, 0x17d3, 1},
        {0x17dd, 0x180b, 46},
        {0x180c, 0x180d, 1},
        {0x1885, 0x1886, 1},
        {0x18a9, 0x1920, 119},
        {0x1921, 0x192b, 1},
        {0x1930, 0x193b, 1},
        {0x1a17, 0x1a1b, 1},
        {0x1a55, 0x1a5e, 1},
        {0x1a60, 0x1a7c, 1},
        {0x1a7f, 0x1ab0, 49},
        {0x1ab1, 0x1ac0, 1},
        {0x1b00, 0x1b04, 1},
        {0x1b34, 0x1b44, 1},
        {0x1b6b, 0x1b73, 1},
        {0x1b80, 0x1b82, 1},
        {0x1ba1, 0x1bad, 1},
        {0x1be6, 0x1bf3, 1},
        {0x1c24, 0x1c37, 1},
        {0x1cd0, 0x1cd2, 1},
        {0x1cd4, 0x1ce8, 1},
        {0x1ced, 0x1cf4, 7},
        {0x1cf7, 0x1cf9, 1},
        {0x1dc0, 0x1df9, 1},
        {0x1dfb, 0x1dff, 1},
        {0x20d0, 0x20f0, 1},
        {0x2cef, 0x2cf1, 1},
        {0x2d7f, 0x2de0, 97},
        {0x2de1, 0x2dff, 1},
        {0x302a, 0x302f, 1},
        {0x3099, 0x309a, 1},
        {0xa66f, 0xa672, 1},
        {0xa674, 0xa67d, 1},
        {0xa69e, 0xa69f, 1},
        {0xa6f0, 0xa6f1, 1},
        {0xa802, 0xa806, 4},
        {0xa80b, 0xa823, 24},
        {0xa824, 0xa827, 1},
        {0xa82c, 0xa880, 84},
        {0xa881, 0xa8b4, 51},
        {0xa8b5, 0xa8c5, 1},
        {0xa8e0, 0xa8f1, 1},
        {0xa8ff, 0xa926, 39},
        {0xa927, 0xa92d, 1},
        {0xa947, 0xa953, 1},
        {0xa980, 0xa983, 1},
        {0xa9b3, 0xa9c0, 1},
        {0xa9e5, 0xaa29, 68},
        {0xaa2a, 0xaa36, 1},
        {0xaa43, 0xaa4c, 9},
        {0xaa4d, 0xaa7b, 46},
        {0xaa7c, 0xaa7d, 1},
        {0xaab0, 0xaab2, 2},
        {0xaab3, 0xaab4, 1},
        {0xaab7, 0xaab8, 1},
        {0xaabe, 0xaabf, 1},
        {0xaac1, 0xaaeb, 42},
        {0xaaec, 0xaaef, 1},
        {0xaaf5, 0xaaf6, 1},
        {0xabe3, 0xabea, 1},
        {0xabec, 0xabed, 1},
        {0xfb1e, 0xfe00, 738},
        {0xfe01, 0xfe0f, 1},
        {0xfe20, 0xfe2f, 1},
    },
    r32: {
        {0x101fd, 0x102e0, 227},
        {0x10376, 0x1037a, 1},
        {0x10a01, 0x10a03, 1},
        {0x10a05, 0x10a06, 1},
        {0x10a0c, 0x10a0f, 1},
        {0x10a38, 0x10a3a, 1},
        {0x10a3f, 0x10ae5, 166},
        {0x10ae6, 0x10d24, 574},
        {0x10d25, 0x10d27, 1},
        {0x10eab, 0x10eac, 1},
        {0x10f46, 0x10f50, 1},
        {0x11000, 0x11002, 1},
        {0x11038, 0x11046, 1},
        {0x1107f, 0x11082, 1},
        {0x110b0, 0x110ba, 1},
        {0x11100, 0x11102, 1},
        {0x11127, 0x11134, 1},
        {0x11145, 0x11146, 1},
        {0x11173, 0x11180, 13},
        {0x11181, 0x11182, 1},
        {0x111b3, 0x111c0, 1},
        {0x111c9, 0x111cc, 1},
        {0x111ce, 0x111cf, 1},
        {0x1122c, 0x11237, 1},
        {0x1123e, 0x112df, 161},
        {0x112e0, 0x112ea, 1},
        {0x11300, 0x11303, 1},
        {0x1133b, 0x1133c, 1},
        {0x1133e, 0x11344, 1},
        {0x11347, 0x11348, 1},
        {0x1134b, 0x1134d, 1},
        {0x11357, 0x11362, 11},
        {0x11363, 0x11366, 3},
        {0x11367, 0x1136c, 1},
        {0x11370, 0x11374, 1},
        {0x11435, 0x11446, 1},
        {0x1145e, 0x114b0, 82},
        {0x114b1, 0x114c3, 1},
        {0x115af, 0x115b5, 1},
        {0x115b8, 0x115c0, 1},
        {0x115dc, 0x115dd, 1},
        {0x11630, 0x11640, 1},
        {0x116ab, 0x116b7, 1},
        {0x1171d, 0x1172b, 1},
        {0x1182c, 0x1183a, 1},
        {0x11930, 0x11935, 1},
        {0x11937, 0x11938, 1},
        {0x1193b, 0x1193e, 1},
        {0x11940, 0x11942, 2},
        {0x11943, 0x119d1, 142},
        {0x119d2, 0x119d7, 1},
        {0x119da, 0x119e0, 1},
        {0x119e4, 0x11a01, 29},
        {0x11a02, 0x11a0a, 1},
        {0x11a33, 0x11a39, 1},
        {0x11a3b, 0x11a3e, 1},
        {0x11a47, 0x11a51, 10},
        {0x11a52, 0x11a5b, 1},
        {0x11a8a, 0x11a99, 1},
        {0x11c2f, 0x11c36, 1},
        {0x11c38, 0x11c3f, 1},
        {0x11c92, 0x11ca7, 1},
        {0x11ca9, 0x11cb6, 1},
        {0x11d31, 0x11d36, 1},
        {0x11d3a, 0x11d3c, 2},
        {0x11d3d, 0x11d3f, 2},
        {0x11d40, 0x11d45, 1},
        {0x11d47, 0x11d8a, 67},
        {0x11d8b, 0x11d8e, 1},
        {0x11d90, 0x11d91, 1},
        {0x11d93, 0x11d97, 1},
        {0x11ef3, 0x11ef6, 1},
        {0x16af0, 0x16af4, 1},
        {0x16b30, 0x16b36, 1},
        {0x16f4f, 0x16f51, 2},
        {0x16f52, 0x16f87, 1},
        {0x16f8f, 0x16f92, 1},
        {0x16fe4, 0x16ff0, 12},
        {0x16ff1, 0x1bc9d, 19628},
        {0x1bc9e, 0x1d165, 5319},
        {0x1d166, 0x1d169, 1},
        {0x1d16d, 0x1d172, 1},
        {0x1d17b, 0x1d182, 1},
        {0x1d185, 0x1d18b, 1},
        {0x1d1aa, 0x1d1ad, 1},
        {0x1d242, 0x1d244, 1},
        {0x1da00, 0x1da36, 1},
        {0x1da3b, 0x1da6c, 1},
        {0x1da75, 0x1da84, 15},
        {0x1da9b, 0x1da9f, 1},
        {0x1daa1, 0x1daaf, 1},
        {0x1e000, 0x1e006, 1},
        {0x1e008, 0x1e018, 1},
        {0x1e01b, 0x1e021, 1},
        {0x1e023, 0x1e024, 1},
        {0x1e026, 0x1e02a, 1},
        {0x1e130, 0x1e136, 1},
        {0x1e2ec, 0x1e2ef, 1},
        {0x1e8d0, 0x1e8d6, 1},
        {0x1e944, 0x1e94a, 1},
        {0xe0100, 0xe01ef, 1},
    }
}

rt! {
    name: _MC,
    r16: {
        {0x0903, 0x093b, 56},
        {0x093e, 0x0940, 1},
        {0x0949, 0x094c, 1},
        {0x094e, 0x094f, 1},
        {0x0982, 0x0983, 1},
        {0x09be, 0x09c0, 1},
        {0x09c7, 0x09c8, 1},
        {0x09cb, 0x09cc, 1},
        {0x09d7, 0x0a03, 44},
        {0x0a3e, 0x0a40, 1},
        {0x0a83, 0x0abe, 59},
        {0x0abf, 0x0ac0, 1},
        {0x0ac9, 0x0acb, 2},
        {0x0acc, 0x0b02, 54},
        {0x0b03, 0x0b3e, 59},
        {0x0b40, 0x0b47, 7},
        {0x0b48, 0x0b4b, 3},
        {0x0b4c, 0x0b57, 11},
        {0x0bbe, 0x0bbf, 1},
        {0x0bc1, 0x0bc2, 1},
        {0x0bc6, 0x0bc8, 1},
        {0x0bca, 0x0bcc, 1},
        {0x0bd7, 0x0c01, 42},
        {0x0c02, 0x0c03, 1},
        {0x0c41, 0x0c44, 1},
        {0x0c82, 0x0c83, 1},
        {0x0cbe, 0x0cc0, 2},
        {0x0cc1, 0x0cc4, 1},
        {0x0cc7, 0x0cc8, 1},
        {0x0cca, 0x0ccb, 1},
        {0x0cd5, 0x0cd6, 1},
        {0x0d02, 0x0d03, 1},
        {0x0d3e, 0x0d40, 1},
        {0x0d46, 0x0d48, 1},
        {0x0d4a, 0x0d4c, 1},
        {0x0d57, 0x0d82, 43},
        {0x0d83, 0x0dcf, 76},
        {0x0dd0, 0x0dd1, 1},
        {0x0dd8, 0x0ddf, 1},
        {0x0df2, 0x0df3, 1},
        {0x0f3e, 0x0f3f, 1},
        {0x0f7f, 0x102b, 172},
        {0x102c, 0x1031, 5},
        {0x1038, 0x103b, 3},
        {0x103c, 0x1056, 26},
        {0x1057, 0x1062, 11},
        {0x1063, 0x1064, 1},
        {0x1067, 0x106d, 1},
        {0x1083, 0x1084, 1},
        {0x1087, 0x108c, 1},
        {0x108f, 0x109a, 11},
        {0x109b, 0x109c, 1},
        {0x17b6, 0x17be, 8},
        {0x17bf, 0x17c5, 1},
        {0x17c7, 0x17c8, 1},
        {0x1923, 0x1926, 1},
        {0x1929, 0x192b, 1},
        {0x1930, 0x1931, 1},
        {0x1933, 0x1938, 1},
        {0x1a19, 0x1a1a, 1},
        {0x1a55, 0x1a57, 2},
        {0x1a61, 0x1a63, 2},
        {0x1a64, 0x1a6d, 9},
        {0x1a6e, 0x1a72, 1},
        {0x1b04, 0x1b35, 49},
        {0x1b3b, 0x1b3d, 2},
        {0x1b3e, 0x1b41, 1},
        {0x1b43, 0x1b44, 1},
        {0x1b82, 0x1ba1, 31},
        {0x1ba6, 0x1ba7, 1},
        {0x1baa, 0x1be7, 61},
        {0x1bea, 0x1bec, 1},
        {0x1bee, 0x1bf2, 4},
        {0x1bf3, 0x1c24, 49},
        {0x1c25, 0x1c2b, 1},
        {0x1c34, 0x1c35, 1},
        {0x1ce1, 0x1cf7, 22},
        {0x302e, 0x302f, 1},
        {0xa823, 0xa824, 1},
        {0xa827, 0xa880, 89},
        {0xa881, 0xa8b4, 51},
        {0xa8b5, 0xa8c3, 1},
        {0xa952, 0xa953, 1},
        {0xa983, 0xa9b4, 49},
        {0xa9b5, 0xa9ba, 5},
        {0xa9bb, 0xa9be, 3},
        {0xa9bf, 0xa9c0, 1},
        {0xaa2f, 0xaa30, 1},
        {0xaa33, 0xaa34, 1},
        {0xaa4d, 0xaa7b, 46},
        {0xaa7d, 0xaaeb, 110},
        {0xaaee, 0xaaef, 1},
        {0xaaf5, 0xabe3, 238},
        {0xabe4, 0xabe6, 2},
        {0xabe7, 0xabe9, 2},
        {0xabea, 0xabec, 2},
    },
    r32: {
        {0x11000, 0x11002, 2},
        {0x11082, 0x110b0, 46},
        {0x110b1, 0x110b2, 1},
        {0x110b7, 0x110b8, 1},
        {0x1112c, 0x11145, 25},
        {0x11146, 0x11182, 60},
        {0x111b3, 0x111b5, 1},
        {0x111bf, 0x111c0, 1},
        {0x111ce, 0x1122c, 94},
        {0x1122d, 0x1122e, 1},
        {0x11232, 0x11233, 1},
        {0x11235, 0x112e0, 171},
        {0x112e1, 0x112e2, 1},
        {0x11302, 0x11303, 1},
        {0x1133e, 0x1133f, 1},
        {0x11341, 0x11344, 1},
        {0x11347, 0x11348, 1},
        {0x1134b, 0x1134d, 1},
        {0x11357, 0x11362, 11},
        {0x11363, 0x11435, 210},
        {0x11436, 0x11437, 1},
        {0x11440, 0x11441, 1},
        {0x11445, 0x114b0, 107},
        {0x114b1, 0x114b2, 1},
        {0x114b9, 0x114bb, 2},
        {0x114bc, 0x114be, 1},
        {0x114c1, 0x115af, 238},
        {0x115b0, 0x115b1, 1},
        {0x115b8, 0x115bb, 1},
        {0x115be, 0x11630, 114},
        {0x11631, 0x11632, 1},
        {0x1163b, 0x1163c, 1},
        {0x1163e, 0x116ac, 110},
        {0x116ae, 0x116af, 1},
        {0x116b6, 0x11720, 106},
        {0x11721, 0x11726, 5},
        {0x1182c, 0x1182e, 1},
        {0x11838, 0x11930, 248},
        {0x11931, 0x11935, 1},
        {0x11937, 0x11938, 1},
        {0x1193d, 0x11940, 3},
        {0x11942, 0x119d1, 143},
        {0x119d2, 0x119d3, 1},
        {0x119dc, 0x119df, 1},
        {0x119e4, 0x11a39, 85},
        {0x11a57, 0x11a58, 1},
        {0x11a97, 0x11c2f, 408},
        {0x11c3e, 0x11ca9, 107},
        {0x11cb1, 0x11cb4, 3},
        {0x11d8a, 0x11d8e, 1},
        {0x11d93, 0x11d94, 1},
        {0x11d96, 0x11ef5, 351},
        {0x11ef6, 0x16f51, 20571},
        {0x16f52, 0x16f87, 1},
        {0x16ff0, 0x16ff1, 1},
        {0x1d165, 0x1d166, 1},
        {0x1d16d, 0x1d172, 1},
    }
}

rt! {
    name: _ME,
    r16: {
        {0x0488, 0x0489, 1},
        {0x1abe, 0x20dd, 1567},
        {0x20de, 0x20e0, 1},
        {0x20e2, 0x20e4, 1},
        {0xa670, 0xa672, 1},
    }
}

rt! {
    name: _MN,
    r16: {
        {0x0300, 0x036f, 1},
        {0x0483, 0x0487, 1},
        {0x0591, 0x05bd, 1},
        {0x05bf, 0x05c1, 2},
        {0x05c2, 0x05c4, 2},
        {0x05c5, 0x05c7, 2},
        {0x0610, 0x061a, 1},
        {0x064b, 0x065f, 1},
        {0x0670, 0x06d6, 102},
        {0x06d7, 0x06dc, 1},
        {0x06df, 0x06e4, 1},
        {0x06e7, 0x06e8, 1},
        {0x06ea, 0x06ed, 1},
        {0x0711, 0x0730, 31},
        {0x0731, 0x074a, 1},
        {0x07a6, 0x07b0, 1},
        {0x07eb, 0x07f3, 1},
        {0x07fd, 0x0816, 25},
        {0x0817, 0x0819, 1},
        {0x081b, 0x0823, 1},
        {0x0825, 0x0827, 1},
        {0x0829, 0x082d, 1},
        {0x0859, 0x085b, 1},
        {0x08d3, 0x08e1, 1},
        {0x08e3, 0x0902, 1},
        {0x093a, 0x093c, 2},
        {0x0941, 0x0948, 1},
        {0x094d, 0x0951, 4},
        {0x0952, 0x0957, 1},
        {0x0962, 0x0963, 1},
        {0x0981, 0x09bc, 59},
        {0x09c1, 0x09c4, 1},
        {0x09cd, 0x09e2, 21},
        {0x09e3, 0x09fe, 27},
        {0x0a01, 0x0a02, 1},
        {0x0a3c, 0x0a41, 5},
        {0x0a42, 0x0a47, 5},
        {0x0a48, 0x0a4b, 3},
        {0x0a4c, 0x0a4d, 1},
        {0x0a51, 0x0a70, 31},
        {0x0a71, 0x0a75, 4},
        {0x0a81, 0x0a82, 1},
        {0x0abc, 0x0ac1, 5},
        {0x0ac2, 0x0ac5, 1},
        {0x0ac7, 0x0ac8, 1},
        {0x0acd, 0x0ae2, 21},
        {0x0ae3, 0x0afa, 23},
        {0x0afb, 0x0aff, 1},
        {0x0b01, 0x0b3c, 59},
        {0x0b3f, 0x0b41, 2},
        {0x0b42, 0x0b44, 1},
        {0x0b4d, 0x0b55, 8},
        {0x0b56, 0x0b62, 12},
        {0x0b63, 0x0b82, 31},
        {0x0bc0, 0x0bcd, 13},
        {0x0c00, 0x0c04, 4},
        {0x0c3e, 0x0c40, 1},
        {0x0c46, 0x0c48, 1},
        {0x0c4a, 0x0c4d, 1},
        {0x0c55, 0x0c56, 1},
        {0x0c62, 0x0c63, 1},
        {0x0c81, 0x0cbc, 59},
        {0x0cbf, 0x0cc6, 7},
        {0x0ccc, 0x0ccd, 1},
        {0x0ce2, 0x0ce3, 1},
        {0x0d00, 0x0d01, 1},
        {0x0d3b, 0x0d3c, 1},
        {0x0d41, 0x0d44, 1},
        {0x0d4d, 0x0d62, 21},
        {0x0d63, 0x0d81, 30},
        {0x0dca, 0x0dd2, 8},
        {0x0dd3, 0x0dd4, 1},
        {0x0dd6, 0x0e31, 91},
        {0x0e34, 0x0e3a, 1},
        {0x0e47, 0x0e4e, 1},
        {0x0eb1, 0x0eb4, 3},
        {0x0eb5, 0x0ebc, 1},
        {0x0ec8, 0x0ecd, 1},
        {0x0f18, 0x0f19, 1},
        {0x0f35, 0x0f39, 2},
        {0x0f71, 0x0f7e, 1},
        {0x0f80, 0x0f84, 1},
        {0x0f86, 0x0f87, 1},
        {0x0f8d, 0x0f97, 1},
        {0x0f99, 0x0fbc, 1},
        {0x0fc6, 0x102d, 103},
        {0x102e, 0x1030, 1},
        {0x1032, 0x1037, 1},
        {0x1039, 0x103a, 1},
        {0x103d, 0x103e, 1},
        {0x1058, 0x1059, 1},
        {0x105e, 0x1060, 1},
        {0x1071, 0x1074, 1},
        {0x1082, 0x1085, 3},
        {0x1086, 0x108d, 7},
        {0x109d, 0x135d, 704},
        {0x135e, 0x135f, 1},
        {0x1712, 0x1714, 1},
        {0x1732, 0x1734, 1},
        {0x1752, 0x1753, 1},
        {0x1772, 0x1773, 1},
        {0x17b4, 0x17b5, 1},
        {0x17b7, 0x17bd, 1},
        {0x17c6, 0x17c9, 3},
        {0x17ca, 0x17d3, 1},
        {0x17dd, 0x180b, 46},
        {0x180c, 0x180d, 1},
        {0x1885, 0x1886, 1},
        {0x18a9, 0x1920, 119},
        {0x1921, 0x1922, 1},
        {0x1927, 0x1928, 1},
        {0x1932, 0x1939, 7},
        {0x193a, 0x193b, 1},
        {0x1a17, 0x1a18, 1},
        {0x1a1b, 0x1a56, 59},
        {0x1a58, 0x1a5e, 1},
        {0x1a60, 0x1a62, 2},
        {0x1a65, 0x1a6c, 1},
        {0x1a73, 0x1a7c, 1},
        {0x1a7f, 0x1ab0, 49},
        {0x1ab1, 0x1abd, 1},
        {0x1abf, 0x1ac0, 1},
        {0x1b00, 0x1b03, 1},
        {0x1b34, 0x1b36, 2},
        {0x1b37, 0x1b3a, 1},
        {0x1b3c, 0x1b42, 6},
        {0x1b6b, 0x1b73, 1},
        {0x1b80, 0x1b81, 1},
        {0x1ba2, 0x1ba5, 1},
        {0x1ba8, 0x1ba9, 1},
        {0x1bab, 0x1bad, 1},
        {0x1be6, 0x1be8, 2},
        {0x1be9, 0x1bed, 4},
        {0x1bef, 0x1bf1, 1},
        {0x1c2c, 0x1c33, 1},
        {0x1c36, 0x1c37, 1},
        {0x1cd0, 0x1cd2, 1},
        {0x1cd4, 0x1ce0, 1},
        {0x1ce2, 0x1ce8, 1},
        {0x1ced, 0x1cf4, 7},
        {0x1cf8, 0x1cf9, 1},
        {0x1dc0, 0x1df9, 1},
        {0x1dfb, 0x1dff, 1},
        {0x20d0, 0x20dc, 1},
        {0x20e1, 0x20e5, 4},
        {0x20e6, 0x20f0, 1},
        {0x2cef, 0x2cf1, 1},
        {0x2d7f, 0x2de0, 97},
        {0x2de1, 0x2dff, 1},
        {0x302a, 0x302d, 1},
        {0x3099, 0x309a, 1},
        {0xa66f, 0xa674, 5},
        {0xa675, 0xa67d, 1},
        {0xa69e, 0xa69f, 1},
        {0xa6f0, 0xa6f1, 1},
        {0xa802, 0xa806, 4},
        {0xa80b, 0xa825, 26},
        {0xa826, 0xa82c, 6},
        {0xa8c4, 0xa8c5, 1},
        {0xa8e0, 0xa8f1, 1},
        {0xa8ff, 0xa926, 39},
        {0xa927, 0xa92d, 1},
        {0xa947, 0xa951, 1},
        {0xa980, 0xa982, 1},
        {0xa9b3, 0xa9b6, 3},
        {0xa9b7, 0xa9b9, 1},
        {0xa9bc, 0xa9bd, 1},
        {0xa9e5, 0xaa29, 68},
        {0xaa2a, 0xaa2e, 1},
        {0xaa31, 0xaa32, 1},
        {0xaa35, 0xaa36, 1},
        {0xaa43, 0xaa4c, 9},
        {0xaa7c, 0xaab0, 52},
        {0xaab2, 0xaab4, 1},
        {0xaab7, 0xaab8, 1},
        {0xaabe, 0xaabf, 1},
        {0xaac1, 0xaaec, 43},
        {0xaaed, 0xaaf6, 9},
        {0xabe5, 0xabe8, 3},
        {0xabed, 0xfb1e, 20273},
        {0xfe00, 0xfe0f, 1},
        {0xfe20, 0xfe2f, 1},
    },
    r32: {
        {0x101fd, 0x102e0, 227},
        {0x10376, 0x1037a, 1},
        {0x10a01, 0x10a03, 1},
        {0x10a05, 0x10a06, 1},
        {0x10a0c, 0x10a0f, 1},
        {0x10a38, 0x10a3a, 1},
        {0x10a3f, 0x10ae5, 166},
        {0x10ae6, 0x10d24, 574},
        {0x10d25, 0x10d27, 1},
        {0x10eab, 0x10eac, 1},
        {0x10f46, 0x10f50, 1},
        {0x11001, 0x11038, 55},
        {0x11039, 0x11046, 1},
        {0x1107f, 0x11081, 1},
        {0x110b3, 0x110b6, 1},
        {0x110b9, 0x110ba, 1},
        {0x11100, 0x11102, 1},
        {0x11127, 0x1112b, 1},
        {0x1112d, 0x11134, 1},
        {0x11173, 0x11180, 13},
        {0x11181, 0x111b6, 53},
        {0x111b7, 0x111be, 1},
        {0x111c9, 0x111cc, 1},
        {0x111cf, 0x1122f, 96},
        {0x11230, 0x11231, 1},
        {0x11234, 0x11236, 2},
        {0x11237, 0x1123e, 7},
        {0x112df, 0x112e3, 4},
        {0x112e4, 0x112ea, 1},
        {0x11300, 0x11301, 1},
        {0x1133b, 0x1133c, 1},
        {0x11340, 0x11366, 38},
        {0x11367, 0x1136c, 1},
        {0x11370, 0x11374, 1},
        {0x11438, 0x1143f, 1},
        {0x11442, 0x11444, 1},
        {0x11446, 0x1145e, 24},
        {0x114b3, 0x114b8, 1},
        {0x114ba, 0x114bf, 5},
        {0x114c0, 0x114c2, 2},
        {0x114c3, 0x115b2, 239},
        {0x115b3, 0x115b5, 1},
        {0x115bc, 0x115bd, 1},
        {0x115bf, 0x115c0, 1},
        {0x115dc, 0x115dd, 1},
        {0x11633, 0x1163a, 1},
        {0x1163d, 0x1163f, 2},
        {0x11640, 0x116ab, 107},
        {0x116ad, 0x116b0, 3},
        {0x116b1, 0x116b5, 1},
        {0x116b7, 0x1171d, 102},
        {0x1171e, 0x1171f, 1},
        {0x11722, 0x11725, 1},
        {0x11727, 0x1172b, 1},
        {0x1182f, 0x11837, 1},
        {0x11839, 0x1183a, 1},
        {0x1193b, 0x1193c, 1},
        {0x1193e, 0x11943, 5},
        {0x119d4, 0x119d7, 1},
        {0x119da, 0x119db, 1},
        {0x119e0, 0x11a01, 33},
        {0x11a02, 0x11a0a, 1},
        {0x11a33, 0x11a38, 1},
        {0x11a3b, 0x11a3e, 1},
        {0x11a47, 0x11a51, 10},
        {0x11a52, 0x11a56, 1},
        {0x11a59, 0x11a5b, 1},
        {0x11a8a, 0x11a96, 1},
        {0x11a98, 0x11a99, 1},
        {0x11c30, 0x11c36, 1},
        {0x11c38, 0x11c3d, 1},
        {0x11c3f, 0x11c92, 83},
        {0x11c93, 0x11ca7, 1},
        {0x11caa, 0x11cb0, 1},
        {0x11cb2, 0x11cb3, 1},
        {0x11cb5, 0x11cb6, 1},
        {0x11d31, 0x11d36, 1},
        {0x11d3a, 0x11d3c, 2},
        {0x11d3d, 0x11d3f, 2},
        {0x11d40, 0x11d45, 1},
        {0x11d47, 0x11d90, 73},
        {0x11d91, 0x11d95, 4},
        {0x11d97, 0x11ef3, 348},
        {0x11ef4, 0x16af0, 19452},
        {0x16af1, 0x16af4, 1},
        {0x16b30, 0x16b36, 1},
        {0x16f4f, 0x16f8f, 64},
        {0x16f90, 0x16f92, 1},
        {0x16fe4, 0x1bc9d, 19641},
        {0x1bc9e, 0x1d167, 5321},
        {0x1d168, 0x1d169, 1},
        {0x1d17b, 0x1d182, 1},
        {0x1d185, 0x1d18b, 1},
        {0x1d1aa, 0x1d1ad, 1},
        {0x1d242, 0x1d244, 1},
        {0x1da00, 0x1da36, 1},
        {0x1da3b, 0x1da6c, 1},
        {0x1da75, 0x1da84, 15},
        {0x1da9b, 0x1da9f, 1},
        {0x1daa1, 0x1daaf, 1},
        {0x1e000, 0x1e006, 1},
        {0x1e008, 0x1e018, 1},
        {0x1e01b, 0x1e021, 1},
        {0x1e023, 0x1e024, 1},
        {0x1e026, 0x1e02a, 1},
        {0x1e130, 0x1e136, 1},
        {0x1e2ec, 0x1e2ef, 1},
        {0x1e8d0, 0x1e8d6, 1},
        {0x1e944, 0x1e94a, 1},
        {0xe0100, 0xe01ef, 1},
    }
}

rt! {
    name: _N,
    r16: {
        {0x0030, 0x0039, 1},
        {0x00b2, 0x00b3, 1},
        {0x00b9, 0x00bc, 3},
        {0x00bd, 0x00be, 1},
        {0x0660, 0x0669, 1},
        {0x06f0, 0x06f9, 1},
        {0x07c0, 0x07c9, 1},
        {0x0966, 0x096f, 1},
        {0x09e6, 0x09ef, 1},
        {0x09f4, 0x09f9, 1},
        {0x0a66, 0x0a6f, 1},
        {0x0ae6, 0x0aef, 1},
        {0x0b66, 0x0b6f, 1},
        {0x0b72, 0x0b77, 1},
        {0x0be6, 0x0bf2, 1},
        {0x0c66, 0x0c6f, 1},
        {0x0c78, 0x0c7e, 1},
        {0x0ce6, 0x0cef, 1},
        {0x0d58, 0x0d5e, 1},
        {0x0d66, 0x0d78, 1},
        {0x0de6, 0x0def, 1},
        {0x0e50, 0x0e59, 1},
        {0x0ed0, 0x0ed9, 1},
        {0x0f20, 0x0f33, 1},
        {0x1040, 0x1049, 1},
        {0x1090, 0x1099, 1},
        {0x1369, 0x137c, 1},
        {0x16ee, 0x16f0, 1},
        {0x17e0, 0x17e9, 1},
        {0x17f0, 0x17f9, 1},
        {0x1810, 0x1819, 1},
        {0x1946, 0x194f, 1},
        {0x19d0, 0x19da, 1},
        {0x1a80, 0x1a89, 1},
        {0x1a90, 0x1a99, 1},
        {0x1b50, 0x1b59, 1},
        {0x1bb0, 0x1bb9, 1},
        {0x1c40, 0x1c49, 1},
        {0x1c50, 0x1c59, 1},
        {0x2070, 0x2074, 4},
        {0x2075, 0x2079, 1},
        {0x2080, 0x2089, 1},
        {0x2150, 0x2182, 1},
        {0x2185, 0x2189, 1},
        {0x2460, 0x249b, 1},
        {0x24ea, 0x24ff, 1},
        {0x2776, 0x2793, 1},
        {0x2cfd, 0x3007, 778},
        {0x3021, 0x3029, 1},
        {0x3038, 0x303a, 1},
        {0x3192, 0x3195, 1},
        {0x3220, 0x3229, 1},
        {0x3248, 0x324f, 1},
        {0x3251, 0x325f, 1},
        {0x3280, 0x3289, 1},
        {0x32b1, 0x32bf, 1},
        {0xa620, 0xa629, 1},
        {0xa6e6, 0xa6ef, 1},
        {0xa830, 0xa835, 1},
        {0xa8d0, 0xa8d9, 1},
        {0xa900, 0xa909, 1},
        {0xa9d0, 0xa9d9, 1},
        {0xa9f0, 0xa9f9, 1},
        {0xaa50, 0xaa59, 1},
        {0xabf0, 0xabf9, 1},
        {0xff10, 0xff19, 1},
    },
    r32: {
        {0x10107, 0x10133, 1},
        {0x10140, 0x10178, 1},
        {0x1018a, 0x1018b, 1},
        {0x102e1, 0x102fb, 1},
        {0x10320, 0x10323, 1},
        {0x10341, 0x1034a, 9},
        {0x103d1, 0x103d5, 1},
        {0x104a0, 0x104a9, 1},
        {0x10858, 0x1085f, 1},
        {0x10879, 0x1087f, 1},
        {0x108a7, 0x108af, 1},
        {0x108fb, 0x108ff, 1},
        {0x10916, 0x1091b, 1},
        {0x109bc, 0x109bd, 1},
        {0x109c0, 0x109cf, 1},
        {0x109d2, 0x109ff, 1},
        {0x10a40, 0x10a48, 1},
        {0x10a7d, 0x10a7e, 1},
        {0x10a9d, 0x10a9f, 1},
        {0x10aeb, 0x10aef, 1},
        {0x10b58, 0x10b5f, 1},
        {0x10b78, 0x10b7f, 1},
        {0x10ba9, 0x10baf, 1},
        {0x10cfa, 0x10cff, 1},
        {0x10d30, 0x10d39, 1},
        {0x10e60, 0x10e7e, 1},
        {0x10f1d, 0x10f26, 1},
        {0x10f51, 0x10f54, 1},
        {0x10fc5, 0x10fcb, 1},
        {0x11052, 0x1106f, 1},
        {0x110f0, 0x110f9, 1},
        {0x11136, 0x1113f, 1},
        {0x111d0, 0x111d9, 1},
        {0x111e1, 0x111f4, 1},
        {0x112f0, 0x112f9, 1},
        {0x11450, 0x11459, 1},
        {0x114d0, 0x114d9, 1},
        {0x11650, 0x11659, 1},
        {0x116c0, 0x116c9, 1},
        {0x11730, 0x1173b, 1},
        {0x118e0, 0x118f2, 1},
        {0x11950, 0x11959, 1},
        {0x11c50, 0x11c6c, 1},
        {0x11d50, 0x11d59, 1},
        {0x11da0, 0x11da9, 1},
        {0x11fc0, 0x11fd4, 1},
        {0x12400, 0x1246e, 1},
        {0x16a60, 0x16a69, 1},
        {0x16b50, 0x16b59, 1},
        {0x16b5b, 0x16b61, 1},
        {0x16e80, 0x16e96, 1},
        {0x1d2e0, 0x1d2f3, 1},
        {0x1d360, 0x1d378, 1},
        {0x1d7ce, 0x1d7ff, 1},
        {0x1e140, 0x1e149, 1},
        {0x1e2f0, 0x1e2f9, 1},
        {0x1e8c7, 0x1e8cf, 1},
        {0x1e950, 0x1e959, 1},
        {0x1ec71, 0x1ecab, 1},
        {0x1ecad, 0x1ecaf, 1},
        {0x1ecb1, 0x1ecb4, 1},
        {0x1ed01, 0x1ed2d, 1},
        {0x1ed2f, 0x1ed3d, 1},
        {0x1f100, 0x1f10c, 1},
        {0x1fbf0, 0x1fbf9, 1},
    },
    latin_offset: 4,
}

rt! {
    name: _ND,
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
}

rt! {
    name: _NL,
    r16: {
        {0x16ee, 0x16f0, 1},
        {0x2160, 0x2182, 1},
        {0x2185, 0x2188, 1},
        {0x3007, 0x3021, 26},
        {0x3022, 0x3029, 1},
        {0x3038, 0x303a, 1},
        {0xa6e6, 0xa6ef, 1},
    },
    r32: {
        {0x10140, 0x10174, 1},
        {0x10341, 0x1034a, 9},
        {0x103d1, 0x103d5, 1},
        {0x12400, 0x1246e, 1},
    }
}

rt! {
    name: _NO,
    r16: {
        {0x00b2, 0x00b3, 1},
        {0x00b9, 0x00bc, 3},
        {0x00bd, 0x00be, 1},
        {0x09f4, 0x09f9, 1},
        {0x0b72, 0x0b77, 1},
        {0x0bf0, 0x0bf2, 1},
        {0x0c78, 0x0c7e, 1},
        {0x0d58, 0x0d5e, 1},
        {0x0d70, 0x0d78, 1},
        {0x0f2a, 0x0f33, 1},
        {0x1369, 0x137c, 1},
        {0x17f0, 0x17f9, 1},
        {0x19da, 0x2070, 1686},
        {0x2074, 0x2079, 1},
        {0x2080, 0x2089, 1},
        {0x2150, 0x215f, 1},
        {0x2189, 0x2460, 727},
        {0x2461, 0x249b, 1},
        {0x24ea, 0x24ff, 1},
        {0x2776, 0x2793, 1},
        {0x2cfd, 0x3192, 1173},
        {0x3193, 0x3195, 1},
        {0x3220, 0x3229, 1},
        {0x3248, 0x324f, 1},
        {0x3251, 0x325f, 1},
        {0x3280, 0x3289, 1},
        {0x32b1, 0x32bf, 1},
        {0xa830, 0xa835, 1},
    },
    r32: {
        {0x10107, 0x10133, 1},
        {0x10175, 0x10178, 1},
        {0x1018a, 0x1018b, 1},
        {0x102e1, 0x102fb, 1},
        {0x10320, 0x10323, 1},
        {0x10858, 0x1085f, 1},
        {0x10879, 0x1087f, 1},
        {0x108a7, 0x108af, 1},
        {0x108fb, 0x108ff, 1},
        {0x10916, 0x1091b, 1},
        {0x109bc, 0x109bd, 1},
        {0x109c0, 0x109cf, 1},
        {0x109d2, 0x109ff, 1},
        {0x10a40, 0x10a48, 1},
        {0x10a7d, 0x10a7e, 1},
        {0x10a9d, 0x10a9f, 1},
        {0x10aeb, 0x10aef, 1},
        {0x10b58, 0x10b5f, 1},
        {0x10b78, 0x10b7f, 1},
        {0x10ba9, 0x10baf, 1},
        {0x10cfa, 0x10cff, 1},
        {0x10e60, 0x10e7e, 1},
        {0x10f1d, 0x10f26, 1},
        {0x10f51, 0x10f54, 1},
        {0x10fc5, 0x10fcb, 1},
        {0x11052, 0x11065, 1},
        {0x111e1, 0x111f4, 1},
        {0x1173a, 0x1173b, 1},
        {0x118ea, 0x118f2, 1},
        {0x11c5a, 0x11c6c, 1},
        {0x11fc0, 0x11fd4, 1},
        {0x16b5b, 0x16b61, 1},
        {0x16e80, 0x16e96, 1},
        {0x1d2e0, 0x1d2f3, 1},
        {0x1d360, 0x1d378, 1},
        {0x1e8c7, 0x1e8cf, 1},
        {0x1ec71, 0x1ecab, 1},
        {0x1ecad, 0x1ecaf, 1},
        {0x1ecb1, 0x1ecb4, 1},
        {0x1ed01, 0x1ed2d, 1},
        {0x1ed2f, 0x1ed3d, 1},
        {0x1f100, 0x1f10c, 1},
    },
    latin_offset: 3,
}

rt! {
    name: _P,
    r16: {
        {0x0021, 0x0023, 1},
        {0x0025, 0x002a, 1},
        {0x002c, 0x002f, 1},
        {0x003a, 0x003b, 1},
        {0x003f, 0x0040, 1},
        {0x005b, 0x005d, 1},
        {0x005f, 0x007b, 28},
        {0x007d, 0x00a1, 36},
        {0x00a7, 0x00ab, 4},
        {0x00b6, 0x00b7, 1},
        {0x00bb, 0x00bf, 4},
        {0x037e, 0x0387, 9},
        {0x055a, 0x055f, 1},
        {0x0589, 0x058a, 1},
        {0x05be, 0x05c0, 2},
        {0x05c3, 0x05c6, 3},
        {0x05f3, 0x05f4, 1},
        {0x0609, 0x060a, 1},
        {0x060c, 0x060d, 1},
        {0x061b, 0x061e, 3},
        {0x061f, 0x066a, 75},
        {0x066b, 0x066d, 1},
        {0x06d4, 0x0700, 44},
        {0x0701, 0x070d, 1},
        {0x07f7, 0x07f9, 1},
        {0x0830, 0x083e, 1},
        {0x085e, 0x0964, 262},
        {0x0965, 0x0970, 11},
        {0x09fd, 0x0a76, 121},
        {0x0af0, 0x0c77, 391},
        {0x0c84, 0x0df4, 368},
        {0x0e4f, 0x0e5a, 11},
        {0x0e5b, 0x0f04, 169},
        {0x0f05, 0x0f12, 1},
        {0x0f14, 0x0f3a, 38},
        {0x0f3b, 0x0f3d, 1},
        {0x0f85, 0x0fd0, 75},
        {0x0fd1, 0x0fd4, 1},
        {0x0fd9, 0x0fda, 1},
        {0x104a, 0x104f, 1},
        {0x10fb, 0x1360, 613},
        {0x1361, 0x1368, 1},
        {0x1400, 0x166e, 622},
        {0x169b, 0x169c, 1},
        {0x16eb, 0x16ed, 1},
        {0x1735, 0x1736, 1},
        {0x17d4, 0x17d6, 1},
        {0x17d8, 0x17da, 1},
        {0x1800, 0x180a, 1},
        {0x1944, 0x1945, 1},
        {0x1a1e, 0x1a1f, 1},
        {0x1aa0, 0x1aa6, 1},
        {0x1aa8, 0x1aad, 1},
        {0x1b5a, 0x1b60, 1},
        {0x1bfc, 0x1bff, 1},
        {0x1c3b, 0x1c3f, 1},
        {0x1c7e, 0x1c7f, 1},
        {0x1cc0, 0x1cc7, 1},
        {0x1cd3, 0x2010, 829},
        {0x2011, 0x2027, 1},
        {0x2030, 0x2043, 1},
        {0x2045, 0x2051, 1},
        {0x2053, 0x205e, 1},
        {0x207d, 0x207e, 1},
        {0x208d, 0x208e, 1},
        {0x2308, 0x230b, 1},
        {0x2329, 0x232a, 1},
        {0x2768, 0x2775, 1},
        {0x27c5, 0x27c6, 1},
        {0x27e6, 0x27ef, 1},
        {0x2983, 0x2998, 1},
        {0x29d8, 0x29db, 1},
        {0x29fc, 0x29fd, 1},
        {0x2cf9, 0x2cfc, 1},
        {0x2cfe, 0x2cff, 1},
        {0x2d70, 0x2e00, 144},
        {0x2e01, 0x2e2e, 1},
        {0x2e30, 0x2e4f, 1},
        {0x2e52, 0x3001, 431},
        {0x3002, 0x3003, 1},
        {0x3008, 0x3011, 1},
        {0x3014, 0x301f, 1},
        {0x3030, 0x303d, 13},
        {0x30a0, 0x30fb, 91},
        {0xa4fe, 0xa4ff, 1},
        {0xa60d, 0xa60f, 1},
        {0xa673, 0xa67e, 11},
        {0xa6f2, 0xa6f7, 1},
        {0xa874, 0xa877, 1},
        {0xa8ce, 0xa8cf, 1},
        {0xa8f8, 0xa8fa, 1},
        {0xa8fc, 0xa92e, 50},
        {0xa92f, 0xa95f, 48},
        {0xa9c1, 0xa9cd, 1},
        {0xa9de, 0xa9df, 1},
        {0xaa5c, 0xaa5f, 1},
        {0xaade, 0xaadf, 1},
        {0xaaf0, 0xaaf1, 1},
        {0xabeb, 0xfd3e, 20819},
        {0xfd3f, 0xfe10, 209},
        {0xfe11, 0xfe19, 1},
        {0xfe30, 0xfe52, 1},
        {0xfe54, 0xfe61, 1},
        {0xfe63, 0xfe68, 5},
        {0xfe6a, 0xfe6b, 1},
        {0xff01, 0xff03, 1},
        {0xff05, 0xff0a, 1},
        {0xff0c, 0xff0f, 1},
        {0xff1a, 0xff1b, 1},
        {0xff1f, 0xff20, 1},
        {0xff3b, 0xff3d, 1},
        {0xff3f, 0xff5b, 28},
        {0xff5d, 0xff5f, 2},
        {0xff60, 0xff65, 1},
    },
    r32: {
        {0x10100, 0x10102, 1},
        {0x1039f, 0x103d0, 49},
        {0x1056f, 0x10857, 744},
        {0x1091f, 0x1093f, 32},
        {0x10a50, 0x10a58, 1},
        {0x10a7f, 0x10af0, 113},
        {0x10af1, 0x10af6, 1},
        {0x10b39, 0x10b3f, 1},
        {0x10b99, 0x10b9c, 1},
        {0x10ead, 0x10f55, 168},
        {0x10f56, 0x10f59, 1},
        {0x11047, 0x1104d, 1},
        {0x110bb, 0x110bc, 1},
        {0x110be, 0x110c1, 1},
        {0x11140, 0x11143, 1},
        {0x11174, 0x11175, 1},
        {0x111c5, 0x111c8, 1},
        {0x111cd, 0x111db, 14},
        {0x111dd, 0x111df, 1},
        {0x11238, 0x1123d, 1},
        {0x112a9, 0x1144b, 418},
        {0x1144c, 0x1144f, 1},
        {0x1145a, 0x1145b, 1},
        {0x1145d, 0x114c6, 105},
        {0x115c1, 0x115d7, 1},
        {0x11641, 0x11643, 1},
        {0x11660, 0x1166c, 1},
        {0x1173c, 0x1173e, 1},
        {0x1183b, 0x11944, 265},
        {0x11945, 0x11946, 1},
        {0x119e2, 0x11a3f, 93},
        {0x11a40, 0x11a46, 1},
        {0x11a9a, 0x11a9c, 1},
        {0x11a9e, 0x11aa2, 1},
        {0x11c41, 0x11c45, 1},
        {0x11c70, 0x11c71, 1},
        {0x11ef7, 0x11ef8, 1},
        {0x11fff, 0x12470, 1137},
        {0x12471, 0x12474, 1},
        {0x16a6e, 0x16a6f, 1},
        {0x16af5, 0x16b37, 66},
        {0x16b38, 0x16b3b, 1},
        {0x16b44, 0x16e97, 851},
        {0x16e98, 0x16e9a, 1},
        {0x16fe2, 0x1bc9f, 19645},
        {0x1da87, 0x1da8b, 1},
        {0x1e95e, 0x1e95f, 1},
    },
    latin_offset: 11,
}

rt! {
    name: _PC,
    r16: {
        {0x005f, 0x203f, 8160},
        {0x2040, 0x2054, 20},
        {0xfe33, 0xfe34, 1},
        {0xfe4d, 0xfe4f, 1},
        {0xff3f, 0xff3f, 1},
    }
}

rt! {
    name: _PD,
    r16: {
        {0x002d, 0x058a, 1373},
        {0x05be, 0x1400, 3650},
        {0x1806, 0x2010, 2058},
        {0x2011, 0x2015, 1},
        {0x2e17, 0x2e1a, 3},
        {0x2e3a, 0x2e3b, 1},
        {0x2e40, 0x301c, 476},
        {0x3030, 0x30a0, 112},
        {0xfe31, 0xfe32, 1},
        {0xfe58, 0xfe63, 11},
        {0xff0d, 0xff0d, 1},
    },
    r32: {
        {0x10ead, 0x10ead, 1},
    }
}

rt! {
    name: _PE,
    r16: {
        {0x0029, 0x005d, 52},
        {0x007d, 0x0f3b, 3774},
        {0x0f3d, 0x169c, 1887},
        {0x2046, 0x207e, 56},
        {0x208e, 0x2309, 635},
        {0x230b, 0x232a, 31},
        {0x2769, 0x2775, 2},
        {0x27c6, 0x27e7, 33},
        {0x27e9, 0x27ef, 2},
        {0x2984, 0x2998, 2},
        {0x29d9, 0x29db, 2},
        {0x29fd, 0x2e23, 1062},
        {0x2e25, 0x2e29, 2},
        {0x3009, 0x3011, 2},
        {0x3015, 0x301b, 2},
        {0x301e, 0x301f, 1},
        {0xfd3e, 0xfe18, 218},
        {0xfe36, 0xfe44, 2},
        {0xfe48, 0xfe5a, 18},
        {0xfe5c, 0xfe5e, 2},
        {0xff09, 0xff3d, 52},
        {0xff5d, 0xff63, 3},
    },
    latin_offset: 1,
}

rt! {
    name: _PF,
    r16: {
        {0x00bb, 0x2019, 8030},
        {0x201d, 0x203a, 29},
        {0x2e03, 0x2e05, 2},
        {0x2e0a, 0x2e0d, 3},
        {0x2e1d, 0x2e21, 4},
    }
}

rt! {
    name: _PI,
    r16: {
        {0x00ab, 0x2018, 8045},
        {0x201b, 0x201c, 1},
        {0x201f, 0x2039, 26},
        {0x2e02, 0x2e04, 2},
        {0x2e09, 0x2e0c, 3},
        {0x2e1c, 0x2e20, 4},
    }
}

rt! {
    name: _PO,
    r16: {
        {0x0021, 0x0023, 1},
        {0x0025, 0x0027, 1},
        {0x002a, 0x002e, 2},
        {0x002f, 0x003a, 11},
        {0x003b, 0x003f, 4},
        {0x0040, 0x005c, 28},
        {0x00a1, 0x00a7, 6},
        {0x00b6, 0x00b7, 1},
        {0x00bf, 0x037e, 703},
        {0x0387, 0x055a, 467},
        {0x055b, 0x055f, 1},
        {0x0589, 0x05c0, 55},
        {0x05c3, 0x05c6, 3},
        {0x05f3, 0x05f4, 1},
        {0x0609, 0x060a, 1},
        {0x060c, 0x060d, 1},
        {0x061b, 0x061e, 3},
        {0x061f, 0x066a, 75},
        {0x066b, 0x066d, 1},
        {0x06d4, 0x0700, 44},
        {0x0701, 0x070d, 1},
        {0x07f7, 0x07f9, 1},
        {0x0830, 0x083e, 1},
        {0x085e, 0x0964, 262},
        {0x0965, 0x0970, 11},
        {0x09fd, 0x0a76, 121},
        {0x0af0, 0x0c77, 391},
        {0x0c84, 0x0df4, 368},
        {0x0e4f, 0x0e5a, 11},
        {0x0e5b, 0x0f04, 169},
        {0x0f05, 0x0f12, 1},
        {0x0f14, 0x0f85, 113},
        {0x0fd0, 0x0fd4, 1},
        {0x0fd9, 0x0fda, 1},
        {0x104a, 0x104f, 1},
        {0x10fb, 0x1360, 613},
        {0x1361, 0x1368, 1},
        {0x166e, 0x16eb, 125},
        {0x16ec, 0x16ed, 1},
        {0x1735, 0x1736, 1},
        {0x17d4, 0x17d6, 1},
        {0x17d8, 0x17da, 1},
        {0x1800, 0x1805, 1},
        {0x1807, 0x180a, 1},
        {0x1944, 0x1945, 1},
        {0x1a1e, 0x1a1f, 1},
        {0x1aa0, 0x1aa6, 1},
        {0x1aa8, 0x1aad, 1},
        {0x1b5a, 0x1b60, 1},
        {0x1bfc, 0x1bff, 1},
        {0x1c3b, 0x1c3f, 1},
        {0x1c7e, 0x1c7f, 1},
        {0x1cc0, 0x1cc7, 1},
        {0x1cd3, 0x2016, 835},
        {0x2017, 0x2020, 9},
        {0x2021, 0x2027, 1},
        {0x2030, 0x2038, 1},
        {0x203b, 0x203e, 1},
        {0x2041, 0x2043, 1},
        {0x2047, 0x2051, 1},
        {0x2053, 0x2055, 2},
        {0x2056, 0x205e, 1},
        {0x2cf9, 0x2cfc, 1},
        {0x2cfe, 0x2cff, 1},
        {0x2d70, 0x2e00, 144},
        {0x2e01, 0x2e06, 5},
        {0x2e07, 0x2e08, 1},
        {0x2e0b, 0x2e0e, 3},
        {0x2e0f, 0x2e16, 1},
        {0x2e18, 0x2e19, 1},
        {0x2e1b, 0x2e1e, 3},
        {0x2e1f, 0x2e2a, 11},
        {0x2e2b, 0x2e2e, 1},
        {0x2e30, 0x2e39, 1},
        {0x2e3c, 0x2e3f, 1},
        {0x2e41, 0x2e43, 2},
        {0x2e44, 0x2e4f, 1},
        {0x2e52, 0x3001, 431},
        {0x3002, 0x3003, 1},
        {0x303d, 0x30fb, 190},
        {0xa4fe, 0xa4ff, 1},
        {0xa60d, 0xa60f, 1},
        {0xa673, 0xa67e, 11},
        {0xa6f2, 0xa6f7, 1},
        {0xa874, 0xa877, 1},
        {0xa8ce, 0xa8cf, 1},
        {0xa8f8, 0xa8fa, 1},
        {0xa8fc, 0xa92e, 50},
        {0xa92f, 0xa95f, 48},
        {0xa9c1, 0xa9cd, 1},
        {0xa9de, 0xa9df, 1},
        {0xaa5c, 0xaa5f, 1},
        {0xaade, 0xaadf, 1},
        {0xaaf0, 0xaaf1, 1},
        {0xabeb, 0xfe10, 21029},
        {0xfe11, 0xfe16, 1},
        {0xfe19, 0xfe30, 23},
        {0xfe45, 0xfe46, 1},
        {0xfe49, 0xfe4c, 1},
        {0xfe50, 0xfe52, 1},
        {0xfe54, 0xfe57, 1},
        {0xfe5f, 0xfe61, 1},
        {0xfe68, 0xfe6a, 2},
        {0xfe6b, 0xff01, 150},
        {0xff02, 0xff03, 1},
        {0xff05, 0xff07, 1},
        {0xff0a, 0xff0e, 2},
        {0xff0f, 0xff1a, 11},
        {0xff1b, 0xff1f, 4},
        {0xff20, 0xff3c, 28},
        {0xff61, 0xff64, 3},
        {0xff65, 0xff65, 1},
    },
    r32: {
        {0x10100, 0x10102, 1},
        {0x1039f, 0x103d0, 49},
        {0x1056f, 0x10857, 744},
        {0x1091f, 0x1093f, 32},
        {0x10a50, 0x10a58, 1},
        {0x10a7f, 0x10af0, 113},
        {0x10af1, 0x10af6, 1},
        {0x10b39, 0x10b3f, 1},
        {0x10b99, 0x10b9c, 1},
        {0x10f55, 0x10f59, 1},
        {0x11047, 0x1104d, 1},
        {0x110bb, 0x110bc, 1},
        {0x110be, 0x110c1, 1},
        {0x11140, 0x11143, 1},
        {0x11174, 0x11175, 1},
        {0x111c5, 0x111c8, 1},
        {0x111cd, 0x111db, 14},
        {0x111dd, 0x111df, 1},
        {0x11238, 0x1123d, 1},
        {0x112a9, 0x1144b, 418},
        {0x1144c, 0x1144f, 1},
        {0x1145a, 0x1145b, 1},
        {0x1145d, 0x114c6, 105},
        {0x115c1, 0x115d7, 1},
        {0x11641, 0x11643, 1},
        {0x11660, 0x1166c, 1},
        {0x1173c, 0x1173e, 1},
        {0x1183b, 0x11944, 265},
        {0x11945, 0x11946, 1},
        {0x119e2, 0x11a3f, 93},
        {0x11a40, 0x11a46, 1},
        {0x11a9a, 0x11a9c, 1},
        {0x11a9e, 0x11aa2, 1},
        {0x11c41, 0x11c45, 1},
        {0x11c70, 0x11c71, 1},
        {0x11ef7, 0x11ef8, 1},
        {0x11fff, 0x12470, 1137},
        {0x12471, 0x12474, 1},
        {0x16a6e, 0x16a6f, 1},
        {0x16af5, 0x16b37, 66},
        {0x16b38, 0x16b3b, 1},
        {0x16b44, 0x16e97, 851},
        {0x16e98, 0x16e9a, 1},
        {0x16fe2, 0x1bc9f, 19645},
        {0x1da87, 0x1da8b, 1},
        {0x1e95e, 0x1e95f, 1},
    },
    latin_offset: 8,
}

rt! {
    name: _PS,
    r16: {
        {0x0028, 0x005b, 51},
        {0x007b, 0x0f3a, 3775},
        {0x0f3c, 0x169b, 1887},
        {0x201a, 0x201e, 4},
        {0x2045, 0x207d, 56},
        {0x208d, 0x2308, 635},
        {0x230a, 0x2329, 31},
        {0x2768, 0x2774, 2},
        {0x27c5, 0x27e6, 33},
        {0x27e8, 0x27ee, 2},
        {0x2983, 0x2997, 2},
        {0x29d8, 0x29da, 2},
        {0x29fc, 0x2e22, 1062},
        {0x2e24, 0x2e28, 2},
        {0x2e42, 0x3008, 454},
        {0x300a, 0x3010, 2},
        {0x3014, 0x301a, 2},
        {0x301d, 0xfd3f, 52514},
        {0xfe17, 0xfe35, 30},
        {0xfe37, 0xfe43, 2},
        {0xfe47, 0xfe59, 18},
        {0xfe5b, 0xfe5d, 2},
        {0xff08, 0xff3b, 51},
        {0xff5b, 0xff5f, 4},
        {0xff62, 0xff62, 1},
    },
    latin_offset: 1,
}

rt! {
    name: _S,
    r16: {
        {0x0024, 0x002b, 7},
        {0x003c, 0x003e, 1},
        {0x005e, 0x0060, 2},
        {0x007c, 0x007e, 2},
        {0x00a2, 0x00a6, 1},
        {0x00a8, 0x00a9, 1},
        {0x00ac, 0x00ae, 2},
        {0x00af, 0x00b1, 1},
        {0x00b4, 0x00b8, 4},
        {0x00d7, 0x00f7, 32},
        {0x02c2, 0x02c5, 1},
        {0x02d2, 0x02df, 1},
        {0x02e5, 0x02eb, 1},
        {0x02ed, 0x02ef, 2},
        {0x02f0, 0x02ff, 1},
        {0x0375, 0x0384, 15},
        {0x0385, 0x03f6, 113},
        {0x0482, 0x058d, 267},
        {0x058e, 0x058f, 1},
        {0x0606, 0x0608, 1},
        {0x060b, 0x060e, 3},
        {0x060f, 0x06de, 207},
        {0x06e9, 0x06fd, 20},
        {0x06fe, 0x07f6, 248},
        {0x07fe, 0x07ff, 1},
        {0x09f2, 0x09f3, 1},
        {0x09fa, 0x09fb, 1},
        {0x0af1, 0x0b70, 127},
        {0x0bf3, 0x0bfa, 1},
        {0x0c7f, 0x0d4f, 208},
        {0x0d79, 0x0e3f, 198},
        {0x0f01, 0x0f03, 1},
        {0x0f13, 0x0f15, 2},
        {0x0f16, 0x0f17, 1},
        {0x0f1a, 0x0f1f, 1},
        {0x0f34, 0x0f38, 2},
        {0x0fbe, 0x0fc5, 1},
        {0x0fc7, 0x0fcc, 1},
        {0x0fce, 0x0fcf, 1},
        {0x0fd5, 0x0fd8, 1},
        {0x109e, 0x109f, 1},
        {0x1390, 0x1399, 1},
        {0x166d, 0x17db, 366},
        {0x1940, 0x19de, 158},
        {0x19df, 0x19ff, 1},
        {0x1b61, 0x1b6a, 1},
        {0x1b74, 0x1b7c, 1},
        {0x1fbd, 0x1fbf, 2},
        {0x1fc0, 0x1fc1, 1},
        {0x1fcd, 0x1fcf, 1},
        {0x1fdd, 0x1fdf, 1},
        {0x1fed, 0x1fef, 1},
        {0x1ffd, 0x1ffe, 1},
        {0x2044, 0x2052, 14},
        {0x207a, 0x207c, 1},
        {0x208a, 0x208c, 1},
        {0x20a0, 0x20bf, 1},
        {0x2100, 0x2101, 1},
        {0x2103, 0x2106, 1},
        {0x2108, 0x2109, 1},
        {0x2114, 0x2116, 2},
        {0x2117, 0x2118, 1},
        {0x211e, 0x2123, 1},
        {0x2125, 0x2129, 2},
        {0x212e, 0x213a, 12},
        {0x213b, 0x2140, 5},
        {0x2141, 0x2144, 1},
        {0x214a, 0x214d, 1},
        {0x214f, 0x218a, 59},
        {0x218b, 0x2190, 5},
        {0x2191, 0x2307, 1},
        {0x230c, 0x2328, 1},
        {0x232b, 0x2426, 1},
        {0x2440, 0x244a, 1},
        {0x249c, 0x24e9, 1},
        {0x2500, 0x2767, 1},
        {0x2794, 0x27c4, 1},
        {0x27c7, 0x27e5, 1},
        {0x27f0, 0x2982, 1},
        {0x2999, 0x29d7, 1},
        {0x29dc, 0x29fb, 1},
        {0x29fe, 0x2b73, 1},
        {0x2b76, 0x2b95, 1},
        {0x2b97, 0x2bff, 1},
        {0x2ce5, 0x2cea, 1},
        {0x2e50, 0x2e51, 1},
        {0x2e80, 0x2e99, 1},
        {0x2e9b, 0x2ef3, 1},
        {0x2f00, 0x2fd5, 1},
        {0x2ff0, 0x2ffb, 1},
        {0x3004, 0x3012, 14},
        {0x3013, 0x3020, 13},
        {0x3036, 0x3037, 1},
        {0x303e, 0x303f, 1},
        {0x309b, 0x309c, 1},
        {0x3190, 0x3191, 1},
        {0x3196, 0x319f, 1},
        {0x31c0, 0x31e3, 1},
        {0x3200, 0x321e, 1},
        {0x322a, 0x3247, 1},
        {0x3250, 0x3260, 16},
        {0x3261, 0x327f, 1},
        {0x328a, 0x32b0, 1},
        {0x32c0, 0x33ff, 1},
        {0x4dc0, 0x4dff, 1},
        {0xa490, 0xa4c6, 1},
        {0xa700, 0xa716, 1},
        {0xa720, 0xa721, 1},
        {0xa789, 0xa78a, 1},
        {0xa828, 0xa82b, 1},
        {0xa836, 0xa839, 1},
        {0xaa77, 0xaa79, 1},
        {0xab5b, 0xab6a, 15},
        {0xab6b, 0xfb29, 20414},
        {0xfbb2, 0xfbc1, 1},
        {0xfdfc, 0xfdfd, 1},
        {0xfe62, 0xfe64, 2},
        {0xfe65, 0xfe66, 1},
        {0xfe69, 0xff04, 155},
        {0xff0b, 0xff1c, 17},
        {0xff1d, 0xff1e, 1},
        {0xff3e, 0xff40, 2},
        {0xff5c, 0xff5e, 2},
        {0xffe0, 0xffe6, 1},
        {0xffe8, 0xffee, 1},
        {0xfffc, 0xfffd, 1},
    },
    r32: {
        {0x10137, 0x1013f, 1},
        {0x10179, 0x10189, 1},
        {0x1018c, 0x1018e, 1},
        {0x10190, 0x1019c, 1},
        {0x101a0, 0x101d0, 48},
        {0x101d1, 0x101fc, 1},
        {0x10877, 0x10878, 1},
        {0x10ac8, 0x1173f, 3191},
        {0x11fd5, 0x11ff1, 1},
        {0x16b3c, 0x16b3f, 1},
        {0x16b45, 0x1bc9c, 20823},
        {0x1d000, 0x1d0f5, 1},
        {0x1d100, 0x1d126, 1},
        {0x1d129, 0x1d164, 1},
        {0x1d16a, 0x1d16c, 1},
        {0x1d183, 0x1d184, 1},
        {0x1d18c, 0x1d1a9, 1},
        {0x1d1ae, 0x1d1e8, 1},
        {0x1d200, 0x1d241, 1},
        {0x1d245, 0x1d300, 187},
        {0x1d301, 0x1d356, 1},
        {0x1d6c1, 0x1d6db, 26},
        {0x1d6fb, 0x1d715, 26},
        {0x1d735, 0x1d74f, 26},
        {0x1d76f, 0x1d789, 26},
        {0x1d7a9, 0x1d7c3, 26},
        {0x1d800, 0x1d9ff, 1},
        {0x1da37, 0x1da3a, 1},
        {0x1da6d, 0x1da74, 1},
        {0x1da76, 0x1da83, 1},
        {0x1da85, 0x1da86, 1},
        {0x1e14f, 0x1e2ff, 432},
        {0x1ecac, 0x1ecb0, 4},
        {0x1ed2e, 0x1eef0, 450},
        {0x1eef1, 0x1f000, 271},
        {0x1f001, 0x1f02b, 1},
        {0x1f030, 0x1f093, 1},
        {0x1f0a0, 0x1f0ae, 1},
        {0x1f0b1, 0x1f0bf, 1},
        {0x1f0c1, 0x1f0cf, 1},
        {0x1f0d1, 0x1f0f5, 1},
        {0x1f10d, 0x1f1ad, 1},
        {0x1f1e6, 0x1f202, 1},
        {0x1f210, 0x1f23b, 1},
        {0x1f240, 0x1f248, 1},
        {0x1f250, 0x1f251, 1},
        {0x1f260, 0x1f265, 1},
        {0x1f300, 0x1f6d7, 1},
        {0x1f6e0, 0x1f6ec, 1},
        {0x1f6f0, 0x1f6fc, 1},
        {0x1f700, 0x1f773, 1},
        {0x1f780, 0x1f7d8, 1},
        {0x1f7e0, 0x1f7eb, 1},
        {0x1f800, 0x1f80b, 1},
        {0x1f810, 0x1f847, 1},
        {0x1f850, 0x1f859, 1},
        {0x1f860, 0x1f887, 1},
        {0x1f890, 0x1f8ad, 1},
        {0x1f8b0, 0x1f8b1, 1},
        {0x1f900, 0x1f978, 1},
        {0x1f97a, 0x1f9cb, 1},
        {0x1f9cd, 0x1fa53, 1},
        {0x1fa60, 0x1fa6d, 1},
        {0x1fa70, 0x1fa74, 1},
        {0x1fa78, 0x1fa7a, 1},
        {0x1fa80, 0x1fa86, 1},
        {0x1fa90, 0x1faa8, 1},
        {0x1fab0, 0x1fab6, 1},
        {0x1fac0, 0x1fac2, 1},
        {0x1fad0, 0x1fad6, 1},
        {0x1fb00, 0x1fb92, 1},
        {0x1fb94, 0x1fbca, 1},
    },
    latin_offset: 10
}

rt! {
    name: _SC,
    r16: {
        {0x0024, 0x00a2, 126},
        {0x00a3, 0x00a5, 1},
        {0x058f, 0x060b, 124},
        {0x07fe, 0x07ff, 1},
        {0x09f2, 0x09f3, 1},
        {0x09fb, 0x0af1, 246},
        {0x0bf9, 0x0e3f, 582},
        {0x17db, 0x20a0, 2245},
        {0x20a1, 0x20bf, 1},
        {0xa838, 0xfdfc, 21956},
        {0xfe69, 0xff04, 155},
        {0xffe0, 0xffe1, 1},
        {0xffe5, 0xffe6, 1},
    },
    r32: {
        {0x11fdd, 0x11fe0, 1},
        {0x1e2ff, 0x1ecb0, 2481},
    },
    latin_offset: 2,
}

rt! {
    name: _SK,
    r16: {
        {0x005e, 0x0060, 2},
        {0x00a8, 0x00af, 7},
        {0x00b4, 0x00b8, 4},
        {0x02c2, 0x02c5, 1},
        {0x02d2, 0x02df, 1},
        {0x02e5, 0x02eb, 1},
        {0x02ed, 0x02ef, 2},
        {0x02f0, 0x02ff, 1},
        {0x0375, 0x0384, 15},
        {0x0385, 0x1fbd, 7224},
        {0x1fbf, 0x1fc1, 1},
        {0x1fcd, 0x1fcf, 1},
        {0x1fdd, 0x1fdf, 1},
        {0x1fed, 0x1fef, 1},
        {0x1ffd, 0x1ffe, 1},
        {0x309b, 0x309c, 1},
        {0xa700, 0xa716, 1},
        {0xa720, 0xa721, 1},
        {0xa789, 0xa78a, 1},
        {0xab5b, 0xab6a, 15},
        {0xab6b, 0xfbb2, 20551},
        {0xfbb3, 0xfbc1, 1},
        {0xff3e, 0xff40, 2},
        {0xffe3, 0xffe3, 1},
    },
    r32: {
        {0x1f3fb, 0x1f3ff, 1},
    },
    latin_offset: 3,
}

rt! {
    name: _SM,
    r16: {
        {0x002b, 0x003c, 17},
        {0x003d, 0x003e, 1},
        {0x007c, 0x007e, 2},
        {0x00ac, 0x00b1, 5},
        {0x00d7, 0x00f7, 32},
        {0x03f6, 0x0606, 528},
        {0x0607, 0x0608, 1},
        {0x2044, 0x2052, 14},
        {0x207a, 0x207c, 1},
        {0x208a, 0x208c, 1},
        {0x2118, 0x2140, 40},
        {0x2141, 0x2144, 1},
        {0x214b, 0x2190, 69},
        {0x2191, 0x2194, 1},
        {0x219a, 0x219b, 1},
        {0x21a0, 0x21a6, 3},
        {0x21ae, 0x21ce, 32},
        {0x21cf, 0x21d2, 3},
        {0x21d4, 0x21f4, 32},
        {0x21f5, 0x22ff, 1},
        {0x2320, 0x2321, 1},
        {0x237c, 0x239b, 31},
        {0x239c, 0x23b3, 1},
        {0x23dc, 0x23e1, 1},
        {0x25b7, 0x25c1, 10},
        {0x25f8, 0x25ff, 1},
        {0x266f, 0x27c0, 337},
        {0x27c1, 0x27c4, 1},
        {0x27c7, 0x27e5, 1},
        {0x27f0, 0x27ff, 1},
        {0x2900, 0x2982, 1},
        {0x2999, 0x29d7, 1},
        {0x29dc, 0x29fb, 1},
        {0x29fe, 0x2aff, 1},
        {0x2b30, 0x2b44, 1},
        {0x2b47, 0x2b4c, 1},
        {0xfb29, 0xfe62, 825},
        {0xfe64, 0xfe66, 1},
        {0xff0b, 0xff1c, 17},
        {0xff1d, 0xff1e, 1},
        {0xff5c, 0xff5e, 2},
        {0xffe2, 0xffe9, 7},
        {0xffea, 0xffec, 1},
    },
    r32: {
        {0x1d6c1, 0x1d6db, 26},
        {0x1d6fb, 0x1d715, 26},
        {0x1d735, 0x1d74f, 26},
        {0x1d76f, 0x1d789, 26},
        {0x1d7a9, 0x1d7c3, 26},
        {0x1eef0, 0x1eef1, 1},
    },
    latin_offset: 5,
}

rt! {
    name: _SO,
    r16: {
        {0x00a6, 0x00a9, 3},
        {0x00ae, 0x00b0, 2},
        {0x0482, 0x058d, 267},
        {0x058e, 0x060e, 128},
        {0x060f, 0x06de, 207},
        {0x06e9, 0x06fd, 20},
        {0x06fe, 0x07f6, 248},
        {0x09fa, 0x0b70, 374},
        {0x0bf3, 0x0bf8, 1},
        {0x0bfa, 0x0c7f, 133},
        {0x0d4f, 0x0d79, 42},
        {0x0f01, 0x0f03, 1},
        {0x0f13, 0x0f15, 2},
        {0x0f16, 0x0f17, 1},
        {0x0f1a, 0x0f1f, 1},
        {0x0f34, 0x0f38, 2},
        {0x0fbe, 0x0fc5, 1},
        {0x0fc7, 0x0fcc, 1},
        {0x0fce, 0x0fcf, 1},
        {0x0fd5, 0x0fd8, 1},
        {0x109e, 0x109f, 1},
        {0x1390, 0x1399, 1},
        {0x166d, 0x1940, 723},
        {0x19de, 0x19ff, 1},
        {0x1b61, 0x1b6a, 1},
        {0x1b74, 0x1b7c, 1},
        {0x2100, 0x2101, 1},
        {0x2103, 0x2106, 1},
        {0x2108, 0x2109, 1},
        {0x2114, 0x2116, 2},
        {0x2117, 0x211e, 7},
        {0x211f, 0x2123, 1},
        {0x2125, 0x2129, 2},
        {0x212e, 0x213a, 12},
        {0x213b, 0x214a, 15},
        {0x214c, 0x214d, 1},
        {0x214f, 0x218a, 59},
        {0x218b, 0x2195, 10},
        {0x2196, 0x2199, 1},
        {0x219c, 0x219f, 1},
        {0x21a1, 0x21a2, 1},
        {0x21a4, 0x21a5, 1},
        {0x21a7, 0x21ad, 1},
        {0x21af, 0x21cd, 1},
        {0x21d0, 0x21d1, 1},
        {0x21d3, 0x21d5, 2},
        {0x21d6, 0x21f3, 1},
        {0x2300, 0x2307, 1},
        {0x230c, 0x231f, 1},
        {0x2322, 0x2328, 1},
        {0x232b, 0x237b, 1},
        {0x237d, 0x239a, 1},
        {0x23b4, 0x23db, 1},
        {0x23e2, 0x2426, 1},
        {0x2440, 0x244a, 1},
        {0x249c, 0x24e9, 1},
        {0x2500, 0x25b6, 1},
        {0x25b8, 0x25c0, 1},
        {0x25c2, 0x25f7, 1},
        {0x2600, 0x266e, 1},
        {0x2670, 0x2767, 1},
        {0x2794, 0x27bf, 1},
        {0x2800, 0x28ff, 1},
        {0x2b00, 0x2b2f, 1},
        {0x2b45, 0x2b46, 1},
        {0x2b4d, 0x2b73, 1},
        {0x2b76, 0x2b95, 1},
        {0x2b97, 0x2bff, 1},
        {0x2ce5, 0x2cea, 1},
        {0x2e50, 0x2e51, 1},
        {0x2e80, 0x2e99, 1},
        {0x2e9b, 0x2ef3, 1},
        {0x2f00, 0x2fd5, 1},
        {0x2ff0, 0x2ffb, 1},
        {0x3004, 0x3012, 14},
        {0x3013, 0x3020, 13},
        {0x3036, 0x3037, 1},
        {0x303e, 0x303f, 1},
        {0x3190, 0x3191, 1},
        {0x3196, 0x319f, 1},
        {0x31c0, 0x31e3, 1},
        {0x3200, 0x321e, 1},
        {0x322a, 0x3247, 1},
        {0x3250, 0x3260, 16},
        {0x3261, 0x327f, 1},
        {0x328a, 0x32b0, 1},
        {0x32c0, 0x33ff, 1},
        {0x4dc0, 0x4dff, 1},
        {0xa490, 0xa4c6, 1},
        {0xa828, 0xa82b, 1},
        {0xa836, 0xa837, 1},
        {0xa839, 0xaa77, 574},
        {0xaa78, 0xaa79, 1},
        {0xfdfd, 0xffe4, 487},
        {0xffe8, 0xffed, 5},
        {0xffee, 0xfffc, 14},
        {0xfffd, 0xfffd, 1},
    },
    r32: {
        {0x10137, 0x1013f, 1},
        {0x10179, 0x10189, 1},
        {0x1018c, 0x1018e, 1},
        {0x10190, 0x1019c, 1},
        {0x101a0, 0x101d0, 48},
        {0x101d1, 0x101fc, 1},
        {0x10877, 0x10878, 1},
        {0x10ac8, 0x1173f, 3191},
        {0x11fd5, 0x11fdc, 1},
        {0x11fe1, 0x11ff1, 1},
        {0x16b3c, 0x16b3f, 1},
        {0x16b45, 0x1bc9c, 20823},
        {0x1d000, 0x1d0f5, 1},
        {0x1d100, 0x1d126, 1},
        {0x1d129, 0x1d164, 1},
        {0x1d16a, 0x1d16c, 1},
        {0x1d183, 0x1d184, 1},
        {0x1d18c, 0x1d1a9, 1},
        {0x1d1ae, 0x1d1e8, 1},
        {0x1d200, 0x1d241, 1},
        {0x1d245, 0x1d300, 187},
        {0x1d301, 0x1d356, 1},
        {0x1d800, 0x1d9ff, 1},
        {0x1da37, 0x1da3a, 1},
        {0x1da6d, 0x1da74, 1},
        {0x1da76, 0x1da83, 1},
        {0x1da85, 0x1da86, 1},
        {0x1e14f, 0x1ecac, 2909},
        {0x1ed2e, 0x1f000, 722},
        {0x1f001, 0x1f02b, 1},
        {0x1f030, 0x1f093, 1},
        {0x1f0a0, 0x1f0ae, 1},
        {0x1f0b1, 0x1f0bf, 1},
        {0x1f0c1, 0x1f0cf, 1},
        {0x1f0d1, 0x1f0f5, 1},
        {0x1f10d, 0x1f1ad, 1},
        {0x1f1e6, 0x1f202, 1},
        {0x1f210, 0x1f23b, 1},
        {0x1f240, 0x1f248, 1},
        {0x1f250, 0x1f251, 1},
        {0x1f260, 0x1f265, 1},
        {0x1f300, 0x1f3fa, 1},
        {0x1f400, 0x1f6d7, 1},
        {0x1f6e0, 0x1f6ec, 1},
        {0x1f6f0, 0x1f6fc, 1},
        {0x1f700, 0x1f773, 1},
        {0x1f780, 0x1f7d8, 1},
        {0x1f7e0, 0x1f7eb, 1},
        {0x1f800, 0x1f80b, 1},
        {0x1f810, 0x1f847, 1},
        {0x1f850, 0x1f859, 1},
        {0x1f860, 0x1f887, 1},
        {0x1f890, 0x1f8ad, 1},
        {0x1f8b0, 0x1f8b1, 1},
        {0x1f900, 0x1f978, 1},
        {0x1f97a, 0x1f9cb, 1},
        {0x1f9cd, 0x1fa53, 1},
        {0x1fa60, 0x1fa6d, 1},
        {0x1fa70, 0x1fa74, 1},
        {0x1fa78, 0x1fa7a, 1},
        {0x1fa80, 0x1fa86, 1},
        {0x1fa90, 0x1faa8, 1},
        {0x1fab0, 0x1fab6, 1},
        {0x1fac0, 0x1fac2, 1},
        {0x1fad0, 0x1fad6, 1},
        {0x1fb00, 0x1fb92, 1},
        {0x1fb94, 0x1fbca, 1},
    },
    latin_offset: 2,
}

rt! {
    name: _Z,
    r16: {
        {0x0020, 0x00a0, 128},
        {0x1680, 0x2000, 2432},
        {0x2001, 0x200a, 1},
        {0x2028, 0x2029, 1},
        {0x202f, 0x205f, 48},
        {0x3000, 0x3000, 1},
    },
    latin_offset: 1,
}

rt! {
    name: _ZL,
    r16: {
        {0x2028, 0x2028, 1},
    }
}

rt! {
    name: _ZP,
    r16: {
        {0x2029, 0x2029, 1},
    }
}

rt! {
    name: _ZS,
    r16: {
        {0x0020, 0x00a0, 128},
        {0x1680, 0x2000, 2432},
        {0x2001, 0x200a, 1},
        {0x202f, 0x205f, 48},
        {0x3000, 0x3000, 1},
    },
    latin_offset: 1,
}

// TODO: Srcipts

/// The set of Unicode script tables.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SrciptTables;

impl core::ops::Index<&str> for SrciptTables {
    type Output = RangeTable;

    fn index(&self, key: &str) -> &Self::Output {
        match key {
            "Adlam" => _ADLAM,
            "Ahom" => _AHOM,
            "Anatolian_Hieroglyphs" => _ANATOLIAN_HIEROGLYPHS,
            "Arabic" => _ARABIC,
            "Armenian" => _ARMENIAN,
            "Avestan" => _AVESTAN,
            "Balinese" => _BALINESE,
            "Bamum" => _BAMUM,
            "Bassa_Vah" => _BASSA_VAH,
            "Batak" => _BATAK,
            "Bengali" => _BENGALI,
            "Bhaiksuki" => _BHAIKSUKI,
            "Bopomofo" => _BOPOMOFO,
            "Brahmi" => _BRAHMI,
            "Braille" => _BRAILLE,
            "Buginese" => _BUGINESE,
            "Buhid" => _BUHID,
            "Canadian_Aboriginal" => _CANADIAN_ABORIGINAL,
            "Carian" => _CARIAN,
            "Caucasian_Albanian" => _CAUCASIAN_ALBANIAN,
            "Chakma" => _CHAKMA,
            "Cham" => _CHAM,
            "Cherokee" => _CHEROKEE,
            "Chorasmian" => _CHORASMIAN,
            "Common" => _COMMON,
            "Coptic" => _COPTIC,
            "Cuneiform" => _CUNEIFORM,
            "Cypriot" => _CYPRIOT,
            "Cyrillic" => _CYRILLIC,
            "Deseret" => _DESERET,
            "Devanagari" => _DEVANAGARI,
            "Dives_Akuru" => _DIVES_AKURU,
            "Dogra" => _DOGRA,
            "Duployan" => _DUPLOYAN,
            "Egyptian_Hieroglyphs" => _EGYPTIAN_HIEROGLYPHS,
            "Elbasan" => _ELBASAN,
            "Elymaic" => _ELYMAIC,
            "Ethiopic" => _ETHIOPIC,
            "Georgian" => _GEORGIAN,
            "Glagolitic" => _GLAGOLITIC,
            "Gothic" => _GOTHIC,
            "Grantha" => _GRANTHA,
            "Greek" => _GREEK,
            "Gujarati" => _GUJARATI,
            "Gunjala_Gondi" => _GUNJALA_GONDI,
            "Gurmukhi" => _GURMUKHI,
            "Han" => _HAN,
            "Hangul" => _HANGUL,
            "Hanifi_Rohingya" => _HANIFI_ROHINGYA,
            "Hanunoo" => _HANUNOO,
            "Hatran" => _HATRAN,
            "Hebrew" => _HEBREW,
            "Hiragana" => _HIRAGANA,
            "Imperial_Aramaic" => _IMPERIAL_ARAMAIC,
            "Inherited" => _INHERITED,
            "Inscriptional_Pahlavi" => _INSCRIPTIONAL_PAHLAVI,
            "Inscriptional_Parthian" => _INSCRIPTIONAL_PARTHIAN,
            "Javanese" => _JAVANESE,
            "Kaithi" => Kaithi,
            "Kannada" => Kannada,
            "Katakana" => Katakana,
            "Kayah_Li" => Kayah_Li,
            "Kharoshthi" => Kharoshthi,
            "Khitan_Small_Script" => Khitan_Small_Script,
            "Khmer" => Khmer,
            "Khojki" => Khojki,
            "Khudawadi" => Khudawadi,
            "Lao" => Lao,
            "Latin" => Latin,
            "Lepcha" => Lepcha,
            "Limbu" => Limbu,
            "Linear_A" => Linear_A,
            "Linear_B" => Linear_B,
            "Lisu" => Lisu,
            "Lycian" => Lycian,
            "Lydian" => Lydian,
            "Mahajani" => Mahajani,
            "Makasar" => Makasar,
            "Malayalam" => Malayalam,
            "Mandaic" => Mandaic,
            "Manichaean" => Manichaean,
            "Marchen" => Marchen,
            "Masaram_Gondi" => Masaram_Gondi,
            "Medefaidrin" => Medefaidrin,
            "Meetei_Mayek" => Meetei_Mayek,
            "Mende_Kikakui" => Mende_Kikakui,
            "Meroitic_Cursive" => Meroitic_Cursive,
            "Meroitic_Hieroglyphs" => Meroitic_Hieroglyphs,
            "Miao" => Miao,
            "Modi" => Modi,
            "Mongolian" => Mongolian,
            "Mro" => Mro,
            "Multani" => Multani,
            "Myanmar" => Myanmar,
            "Nabataean" => Nabataean,
            "Nandinagari" => Nandinagari,
            "New_Tai_Lue" => New_Tai_Lue,
            "Newa" => Newa,
            "Nko" => Nko,
            "Nushu" => Nushu,
            "Nyiakeng_Puachue_Hmong" => Nyiakeng_Puachue_Hmong,
            "Ogham" => Ogham,
            "Ol_Chiki" => Ol_Chiki,
            "Old_Hungarian" => Old_Hungarian,
            "Old_Italic" => Old_Italic,
            "Old_North_Arabian" => Old_North_Arabian,
            "Old_Permic" => Old_Permic,
            "Old_Persian" => Old_Persian,
            "Old_Sogdian" => Old_Sogdian,
            "Old_South_Arabian" => Old_South_Arabian,
            "Old_Turkic" => Old_Turkic,
            "Oriya" => Oriya,
            "Osage" => Osage,
            "Osmanya" => Osmanya,
            "Pahawh_Hmong" => _PAHAWH_HMONG,
            "Palmyrene" => _PALMYRENE,
            "Pau_Cin_Hau" => _PAU_CIN_HAU,
            "Phags_Pa" => _PHAGS_PA,
            "Phoenician" => _PHOENICIAN,
            "Psalter_Pahlavi" => _PSALTER_PAHLAVI,
            "Rejang" => _REJANG,
            "Runic" => _RUNIC,
            "Samaritan" => _SAMARITAN,
            "Saurashtra" => _SAURASHTRA,
            "Sharada" => _SHARADA,
            "Shavian" => _SHAVIAN,
            "Siddham" => _SIDDHAM,
            "SignWriting" => _SIGN_WRITING,
            "Sinhala" => _SINHALA,
            "Sogdian" => _SOGDIAN,
            "Sora_Sompeng" => _SORA_SOMPENG,
            "Soyombo" => _SOYOMBO,
            "Sundanese" => _SUNDANESE,
            "Syloti_Nagri" => _SYLOTI_NAGRI,
            "Syriac" => _SYRIAC,
            "Tagalog" => _TAGALOG,
            "Tagbanwa" => _TAGBANWA,
            "Tai_Le" => _TAI_LE,
            "Tai_Tham" => _TAI_THAM,
            "Tai_Viet" => _TAI_VIET,
            "Takri" => _TAKRI,
            "Tamil" => _TAMIL,
            "Tangut" => _TANGUT,
            "Telugu" => _TELUGU,
            "Thaana" => _THAANA,
            "Thai" => _THAI,
            "Tibetan" => _TIBETAN,
            "Tifinagh" => _TIFINAGH,
            "Tirhuta" => _TIRHUTA,
            "Ugaritic" => _UGARITIC,
            "Vai" => _VAI,
            "Wancho" => _WANCHO,
            "Warang_Citi" => _WARANG_CITI,
            "Yezidi" => _YEZIDI,
            "Yi" => _YI,
            "Zanabazar_Square" => _ZANABAZAR_SQUARE,
            _ => panic!("invalid script name"),
        }
    }
}

impl SrciptTables {
    /// Returns the number of scripts
    pub const LEN: usize = 154;

    /// Create a new iter on scripts
    pub fn iter(&self) -> core::slice::Iter<'_, (&'static str, &'static RangeTable)> {}
}

rt_aliases! {
    _ADLAM {
        /// The set of Unicode characters in script Adlam.
        _ADLAM;
    },
    _AHOM {
        /// The set of Unicode characters in script Ahom.
        _AHOM;
    },
    _ANATOLIAN_HIEROGLYPHS {
        /// The set of Unicode characters in script Anatolian Hieroglyphs.
        _ANATOLIAN_HIEROGLYPHS;
    },
    _ARABIC {
        /// The set of Unicode characters in script Arabic.
        _ARABIC;
    },
    _ARMENIAN {
        /// The set of Unicode characters in script Armenian.
        _ARMENIAN;
    },
    _AVESTAN {
        /// The set of Unicode characters in script Avestan.
        _AVESTAN;
    },
    _BALINESE {
        /// The set of Unicode characters in script Balinese.
        _BALINESE;
    },
    _BAMUM {
        /// The set of Unicode characters in script Bamum.
        _BAMUM;
    },
    _BASSA_VAH {
        /// The set of Unicode characters in script Bassa Vah.
        _BASSA_VAH;
    },
    _BATAK {
        /// The set of Unicode characters in script Batak.
        _BATAK;
    },
    _BENGALI {
        /// The set of Unicode characters in script Bengali.
        _BENGALI;
    },
    _BHAIKSUKI {
        /// The set of Unicode characters in script Bhaiksiki.
        BHAIKSUKI;
    },
    _BOPOMOFO {
        /// The set of Unicode characters in script Bopomofo.
        _BOPOMOFO;
    },
    _BRAHMI {
        /// The set of Unicode characters in script Brahmi.
        _BRAHMI;
    },
    _BRAILLE {
        /// The set of Unicode characters in script Braille.
        _BRAILLE;
    },
    _BUGINESE {
        /// The set of Unicode characters in script Buginese.
        _BUGINESE;
    },
    _BUHID {
        /// The set of Unicode characters in script Buhid.
        _BUHID;
    },
    _CANADIAN_ABORIGINAL {
        /// The set of Unicode characters in script Canadian Aboriginal.
        _CANADIAN_ABORIGINAL;
    },
    _CARIAN {
        /// The set of Unicode characters in script Carian.
        _CARIAN;
    },
    _CAUCASIAN_ALBANIAN {
        /// The set of Unicode characters in script Caucasian Albanian.
        _CAUCASIAN_ALBANIAN;
    },
    _CHAKMA {
        /// The set of Unicode characters in script Chakma.
        _CHAKMA;
    },
    _CHAM {
        /// The set of Unicode characters in script Cham.
        _CHAM;
    },
    _CHEROKEE {
        /// The set of Unicode characters in script Cherokee.
        _CHEROKEE;
    },
    _CHORASMIAN {
        /// The set of Unicode characters in script Chorasmian.
        CHORASMIAN;
    },
    _COMMON {
        /// The set of Unicode characters in script Common.
        _COMMON;
    },
    _COPTIC {
        /// The set of Unicode characters in script Coptic.
        _COPTIC;
    },
    _CUNEIFORM {
        /// The set of Unicode characters in script Cuneiform.
        _CUNEIFORM;
    },
    _CYPRIOT {
        /// The set of Unicode characters in script Cypriot.
        _CYPRIOT;
    },
    _CYRILLIC {
        /// The set of Unicode characters in script Cyrillic.
        _CYRILLIC;
    },
    _DESERET {
        /// The set of Unicode characters in script Deseret.
        _DESERET;
    },
    _DEVANAGARI {
        /// The set of Unicode characters in script Devanagari.
        _DEVANAGARI;
    },
    _DIVES_AKURU {
        /// The set of Unicode characters in script Dives Akuru.
        DIVES_AKURU;
    },
    _DOGRA {
        /// The set of Unicode characters in script Dogra.
        _DOGRA;
    },
    _DUPLOYAN {
        /// The set of Unicode characters in script Duployan.
        _DUPLOYAN;
    },
    _EGYPTIAN_HIEROGLYPHS {
        /// The set of Unicode characters in script Egyptian Hieroglyphs.
        EGYPTIAN_HIEROGLYPHS;
    },
    _ELBASAN {
        /// The set of Unicode characters in script Elbasan.
        ELBASAN;
    },
    _ELYMAIC {
        /// The set of Unicode characters in script Elymaic.
        ELYMAIC;
    },
    _ETHIOPIC {
        /// The set of Unicode characters in script Ethiopic.
        ETHIOPIC;
    },
    _GEORGIAN {
        /// The set of Unicode characters in script Georgian.
        GEORGIAN;
    },
    _GLAGOLITIC {
        /// The set of Unicode characters in script Glagolitic.
        GLAGOLITIC;
    },
    _GOTHIC {
        /// The set of Unicode characters in script Gothic.
        GOTHIC;
    },
    _GRANTHA {
        /// The set of Unicode characters in script Grantha.
        GRANTHA;
    },
    _GREEK {
        /// The set of Unicode characters in script Greek.
        GREEK;
    },
    _GUJARATI {
        /// The set of Unicode characters in script Gujarati.
        GUJARATI;
    },
    _GUNJALA_GONDI {
        /// The set of Unicode characters in script Gunjala Gondi.
        GUNJALA_GONDI;
    },
    _GURMUKHI {
        /// The set of Unicode characters in script Gurmukhi.
        GURMUKHI;
    },
    _HAN {
        /// The set of Unicode characters in script Han.
        HAN;
    },
    _HANGUL {
        /// The set of Unicode characters in script Hangul.
        HANGUL;
    },
    _HANIFI_ROHINGYA {
        /// The set of Unicode characters in script Hanifi Rohingya.
        HANIFI_ROHINGYA;
    },
    _HANUNOO {
        /// The set of Unicode characters in script Hanunoo.
        HANUNOO;
    },
    _HATRAN {
        /// The set of Unicode characters in script Hatran.
        HATRAN;
    },
    _HEBREW {
        /// The set of Unicode characters in script Hebrew.
        HEBREW;
    },
    _HIRAGANA {
        /// The set of Unicode characters in script Hiragana.
        HIRAGANA;
    },
    _IMPERIAL_ARAMAIC {
        /// The set of Unicode characters in script Imperial Aramaic.
        IMPERIAL_ARAMAIC;
    },
    _INHERITED {
        /// The set of Unicode characters in script Inherited.
        INHERITED;
    },
    _INSCRIPTIONAL_PAHLAVI {
        /// The set of Unicode characters in script Inscriptional Pahlavi.
        INSCRIPTIONAL_PAHLAVI;
    },
    _INSCRIPTIONAL_PARTHIAN {
        /// The set of Unicode characters in script Inscriptional Parthian.
        INSCRIPTIONAL_PARTHIAN;
    },
    _JAVANESE {
        /// The set of Unicode characters in script Javanese.
        JAVANESE;
    },
    _KAITHI {
        /// The set of Unicode characters in script Kaithi.
        KAITHI;
    },
    _KANNADA {
        /// The set of Unicode characters in script Kannada.
        KANNADA;
    },
    _KATAKANA {
        /// The set of Unicode characters in script Katakana.
        KATAKANA;
    },
    _KAYAH_LI {
        /// The set of Unicode characters in script Kayah Li.
        KAYAH_LI;
    },
    _KHAROSHTHI {
        /// The set of Unicode characters in script Kharoshthi.
        KHAROSHTHI;
    },
    _KHITAN_SMALL_SCRIPT {
        /// The set of Unicode characters in script Khitan Small Script.
        KHITAN_SMALL_SCRIPT;
    },
    _KHMER {
        /// The set of Unicode characters in script Khmer.
        KHMER;
    },
    _KHOJKI {
        /// The set of Unicode characters in script Khojki.
        KHOJKI;
    },
    _KHUDAWADI {
        /// The set of Unicode characters in script Khudawadi.
        KHUDAWADI;
    },
    _LAO {
        /// The set of Unicode characters in script Lao.
        LAO;
    },
    _LATIN {
        /// The set of Unicode characters in script Latin.
        LATIN;
    },
    _LEPCHA {
        /// The set of Unicode characters in script Lepcha.
        LEPCHA;
    },
    _LIMBU {
        /// The set of Unicode characters in script Limbu.
        LIMBU;
    },
    _LINEAR_A {
        /// The set of Unicode characters in script Linear A.
        LINEAR_A;
    },
    _LINEAR_B {
        /// The set of Unicode characters in script Linear B.
        LINEAR_B;
    },
    _LISU {
        /// The set of Unicode characters in script Lisu.
        LISU;
    },
    _LYCIAN {
        /// The set of Unicode characters in script Lycian.
        LYCIAN;
    },
    _LYDIAN {
        /// The set of Unicode characters in script Lydian.
        LYDIAN;
    },
    _MAHAJANI {
        /// The set of Unicode characters in script Mahajani.
        MAHAJANI;
    },
    _MAKASAR {
        /// The set of Unicode characters in script Makasar.
        MAKASAR;
    },
    _MALAYALAM {
        /// The set of Unicode characters in script Malayalam.
        MALAYALAM;
    },
    _MANDAIC {
        /// The set of Unicode characters in script Mandaic.
        MANDAIC;
    },
    _MANICHAEAN {
        /// The set of Unicode characters in script Manichaean.
        MANICHAEAN;
    },
    _MARCHEN {
        /// The set of Unicode characters in script Marchen.
        MARCHEN;
    },
    _MASARAM_GONDI {
        /// The set of Unicode characters in script Masaram Gondi.
        MASARAM_GONDI;
    },
    _MEDEFAIDRIN {
        /// The set of Unicode characters in script Medefaidrin.
        MEDEFAIDRIN;
    },
    _MEETEI_MAYEK {
        /// The set of Unicode characters in script Meetei Mayek.
        MEETEI_MAYEK;
    },
    _MENDE_KIKAKUI {
        /// The set of Unicode characters in script Mende Kikakui.
        MENDE_KIKAKUI;
    },
    _MEROITIC_CURSIVE {
        /// The set of Unicode characters in script Meroitic Cursive.
        MEROITIC_CURSIVE;
    },
    _MEROITIC_HIEROGLYPHS {
        /// The set of Unicode characters in script Meroitic Hieroglyphs.
        MEROITIC_HIEROGLYPHS;
    },
    _MIAO {
        /// The set of Unicode characters in script Miao.
        MIAO;
    },
    _MODI {
        /// The set of Unicode characters in script Modi.
        MODI;
    },
    _MONGOLIAN {
        /// The set of Unicode characters in script Mongolian.
        MONGOLIAN;
    },
    _MRO {
        /// The set of Unicode characters in script Mro.
        MRO;
    },
    _MULTANI {
        /// The set of Unicode characters in script Multani.
        MULTANI;
    },
    _MYANMAR {
        /// The set of Unicode characters in script Myanmar.
        MYANMAR;
    },
    _NABATAEAN {
        /// The set of Unicode characters in script Nabataean.
        NABATAEAN;
    },
    _NANDINAGARI {
        /// The set of Unicode characters in script Nandinagari.
        NANDINAGARI;
    },
    _NEW_TAI_LUE {
        /// The set of Unicode characters in script New Tai Lue.
        NEW_TAI_LUE;
    },
    _NEWA {
        /// The set of Unicode characters in script Newa.
        NEWA;
    },
    _NKO {
        /// The set of Unicode characters in script Nko.
        NKO;
    },
    _NUSHU {
        /// The set of Unicode characters in script Nushu.
        NUSHU;
    },
    _NYIAKENG_PUACHUE_HMONG {
        /// The set of Unicode characters in script Nyiakeng Puachue Hmong.
        NYIAKENG_PUACHUE_HMONG;
    },
    _OGHAM {
        /// The set of Unicode characters in script Ogham.
        OGHAM;
    },
    _OL_CHIKI {
        /// The set of Unicode characters in script Ol Chiki.
        OL_CHIKI;
    },
    _OLD_HUNGARIAN {
        /// The set of Unicode characters in script Old Hungarian.
        OLD_HUNGARIAN;
    },
    _OLD_ITALIC {
        /// The set of Unicode characters in script Old Italic.
        OLD_ITALIC;
    },
    _OLD_NORTH_ARABIAN {
        /// The set of Unicode characters in script Old North Arabian.
        OLD_NORTH_ARABIAN;
    },
    _OLD_PERMIC {
        /// The set of Unicode characters in script Old Permic.
        OLD_PERMIC;
    },
    _OLD_PERSIAN {
        /// The set of Unicode characters in script Old Persian.
        OLD_PERSIAN;
    },
    _OLD_SOGDIAN {
        /// The set of Unicode characters in script Old Sogdian.
        OLD_SOGDIAN;
    },
    _OLD_SOUTH_ARABIAN {
        /// The set of Unicode characters in script Old South Arabian.
        OLD_SOUTH_ARABIAN;
    },
    _OLD_TURKIC {
        /// The set of Unicode characters in script Old Turkic.
        OLD_TURKIC;
    },
    _ORIYA {
        /// The set of Unicode characters in script Oriya.
        ORIYA;
    },
    _OSAGE {
        /// The set of Unicode characters in script Osage.
        OSAGE;
    },
    _OSMANYA {
        /// The set of Unicode characters in script Osmanya.
        OSMANYA;
    },
    _PAHAWH_HMONG {
        /// The set of Unicode characters in script Pahawh Hmong.
        PAHAWH_HMONG;
    },
    _PALMYRENE {
        /// The set of Unicode characters in script Palmyrene.
        PALMYRENE;
    },
    _PAU_CIN_HAU {
        /// The set of Unicode characters in script Pau Cin Hau.
        PAU_CIN_HAU;
    },
    _PHAGS_PA {
        /// The set of Unicode characters in script Phags Pa.
        PHAGS_PA;
    },
    _PHOENICIAN {
        /// The set of Unicode characters in script Phoenician.
        PHOENICIAN;
    },
    _PSALTER_PAHLAVI {
        /// The set of Unicode characters in script Psalter Pahlavi.
        PSALTER_PAHLAVI;
    },
    _REJANG {
        /// The set of Unicode characters in script Rejang.
        REJANG;
    },
    _RUNIC {
        /// The set of Unicode characters in script Runic.
        RUNIC;
    },
    _SAMARITAN {
        /// The set of Unicode characters in script Samaritan.
        SAMARITAN;
    },
    _SAURASHTRA {
        /// The set of Unicode characters in script Saurashtra.
        SAURASHTRA;
    },
    _SHARADA {
        /// The set of Unicode characters in script Sharada.
        SHARADA;
    },
    _SHAVIAN {
        /// The set of Unicode characters in script Shavian.
        SHAVIAN;
    },
    _SIDDHAM {
        /// The set of Unicode characters in script Siddham.
        SIDDHAM;
    },
    _SIGN_WRITING {
        /// The set of Unicode characters in script SignWriting.
        SIGNWRITING;
    },
    _SINHALA {
        /// The set of Unicode characters in script Sinhala.
        SINHALA;
    },
    _SOGDIAN {
        /// The set of Unicode characters in script Sogdian.
        SOGDIAN;
    },
    _SORA_SOMPENG {
        /// The set of Unicode characters in script Sora Sompeng.
        SORA_SOMPENG;
    },
    _SOYOMBO {
        /// The set of Unicode characters in script Soyombo.
        SOYOMBO;
    },
    _SUNDANESE {
        /// The set of Unicode characters in script Sundanese.
        SUNDANESE;
    },
    _SYLOTI_NAGRI {
        /// The set of Unicode characters in script Syloti Nagri.
        SYLOTI_NAGRI;
    },
    _SYRIAC {
        /// The set of Unicode characters in script Syriac.
        SYRIAC;
    },
    _TAGALOG {
        /// The set of Unicode characters in script Tagalog.
        TAGALOG;
    },
    _TAGBANWA {
        /// The set of Unicode characters in script Tagbanwa.
        TAGBANWA;
    },
    _TAI_LE {
        /// The set of Unicode characters in script Tai Le.
        TAI_LE;
    },
    _TAI_THAM {
        /// The set of Unicode characters in script Tai Tham.
        TAI_THAM;
    },
    _TAI_VIET {
        /// The set of Unicode characters in script Tai Viet.
        TAI_VIET;
    },
    _TAKRI {
        /// The set of Unicode characters in script Takri.
        TAKRI;
    },
    _TAMIL {
        /// The set of Unicode characters in script Tamil.
        TAMIL;
    },
    _TANGUT {
        /// The set of Unicode characters in script Tangut.
        TANGUT;
    },
    _TELUGU {
        /// The set of Unicode characters in script Telugu.
        TELUGU;
    },
    _THAANA {
        /// The set of Unicode characters in script Thaana.
        THAANA;
    },
    _THAI {
        /// The set of Unicode characters in script Thai.
        THAI;
    },
    _TIBETAN {
        /// The set of Unicode characters in script Tibetan.
        TIBETAN;
    },
    _TIFINAGH {
        /// The set of Unicode characters in script Tifinagh.
        TIFINAGH;
    },
    _TIRHUTA {
        /// The set of Unicode characters in script Tirhuta.
        TIRHUTA;
    },
    _UGARITIC {
        /// The set of Unicode characters in script Ugaritic.
        UGARITIC;
    },
    _VAI {
        /// The set of Unicode characters in script Vai.
        VAI;
    },
    _WARANG_CITI {
        /// The set of Unicode characters in script Warang Citi.
        WARANG_CITI;
    },
    _YI {
        /// The set of Unicode characters in script Yi.
        YI;
    },
    _ZANABAZAR_SQUARE {
        /// The set of Unicode characters in script Zanabazar Square.
        ZANABAZAR_SQUARE;
    },
}

rt! {
    name: _ADLAM,
    r32: {
        {0x1e900, 0x1e94b, 1},
        {0x1e950, 0x1e959, 1},
        {0x1e95e, 0x1e95f, 1},
    }
}

rt! {
    name: _AHOM,
    r32: {
        {0x11700, 0x1171a, 1},
        {0x1171d, 0x1172b, 1},
        {0x11730, 0x1173f, 1},
    }
}

rt! {
    name: _ANATOLIAN_HIEROGLYPHS,
    r32: {
        {0x14400, 0x14646, 1},
    }
}

rt! {
    name: _ARABIC,
    r16: {
        {0x0600, 0x0604, 1},
        {0x0606, 0x060b, 1},
        {0x060d, 0x061a, 1},
        {0x061c, 0x0620, 2},
        {0x0621, 0x063f, 1},
        {0x0641, 0x064a, 1},
        {0x0656, 0x066f, 1},
        {0x0671, 0x06dc, 1},
        {0x06de, 0x06ff, 1},
        {0x0750, 0x077f, 1},
        {0x08a0, 0x08b4, 1},
        {0x08b6, 0x08c7, 1},
        {0x08d3, 0x08e1, 1},
        {0x08e3, 0x08ff, 1},
        {0xfb50, 0xfbc1, 1},
        {0xfbd3, 0xfd3d, 1},
        {0xfd50, 0xfd8f, 1},
        {0xfd92, 0xfdc7, 1},
        {0xfdf0, 0xfdfd, 1},
        {0xfe70, 0xfe74, 1},
        {0xfe76, 0xfefc, 1},
    },
    r32: {
        {0x10e60, 0x10e7e, 1},
        {0x1ee00, 0x1ee03, 1},
        {0x1ee05, 0x1ee1f, 1},
        {0x1ee21, 0x1ee22, 1},
        {0x1ee24, 0x1ee27, 3},
        {0x1ee29, 0x1ee32, 1},
        {0x1ee34, 0x1ee37, 1},
        {0x1ee39, 0x1ee3b, 2},
        {0x1ee42, 0x1ee47, 5},
        {0x1ee49, 0x1ee4d, 2},
        {0x1ee4e, 0x1ee4f, 1},
        {0x1ee51, 0x1ee52, 1},
        {0x1ee54, 0x1ee57, 3},
        {0x1ee59, 0x1ee61, 2},
        {0x1ee62, 0x1ee64, 2},
        {0x1ee67, 0x1ee6a, 1},
        {0x1ee6c, 0x1ee72, 1},
        {0x1ee74, 0x1ee77, 1},
        {0x1ee79, 0x1ee7c, 1},
        {0x1ee7e, 0x1ee80, 2},
        {0x1ee81, 0x1ee89, 1},
        {0x1ee8b, 0x1ee9b, 1},
        {0x1eea1, 0x1eea3, 1},
        {0x1eea5, 0x1eea9, 1},
        {0x1eeab, 0x1eebb, 1},
        {0x1eef0, 0x1eef1, 1},
    }
}

rt! {
    name: _ARMENIAN,
    r16: {
        {0x0531, 0x0556, 1},
        {0x0559, 0x058a, 1},
        {0x058d, 0x058f, 1},
        {0xfb13, 0xfb17, 1},
    }
}

rt! {
    name: _AVESTAN,
    r32: {
        {0x10b00, 0x10b35, 1},
        {0x10b39, 0x10b3f, 1},
    }
}

rt! {
    name: _BALINESE,
    r16: {
        {0x1b00, 0x1b4b, 1},
        {0x1b50, 0x1b7c, 1},
    }
}

rt! {
    name: _BAMUM,
    r16: {
        {0xa6a0, 0xa6f7, 1},
    },
    r32: {
        {0x16800, 0x16a38, 1},
    }
}

rt! {
    name: _BASSA_VAH,
    r32: {
        {0x16ad0, 0x16aed, 1},
        {0x16af0, 0x16af5, 1},
    },
}

rt! {
    name: _BATAK,
    r16: {
        {0x1bc0, 0x1bf3, 1},
        {0x1bfc, 0x1bff, 1},
    }
}

rt! {
    name: _BENGALI,
    r16: {
        {0x0980, 0x0983, 1},
        {0x0985, 0x098c, 1},
        {0x098f, 0x0990, 1},
        {0x0993, 0x09a8, 1},
        {0x09aa, 0x09b0, 1},
        {0x09b2, 0x09b6, 4},
        {0x09b7, 0x09b9, 1},
        {0x09bc, 0x09c4, 1},
        {0x09c7, 0x09c8, 1},
        {0x09cb, 0x09ce, 1},
        {0x09d7, 0x09dc, 5},
        {0x09dd, 0x09df, 2},
        {0x09e0, 0x09e3, 1},
        {0x09e6, 0x09fe, 1},
    },
}

rt! {
    name: _BHAIKSUKI,
    r32: {
        {0x11c00, 0x11c08, 1},
        {0x11c0a, 0x11c36, 1},
        {0x11c38, 0x11c45, 1},
        {0x11c50, 0x11c6c, 1},
    }
}

rt! {
    name: _BOPOMOFO,
    r16: {
        {0x02ea, 0x02eb, 1},
        {0x3105, 0x312f, 1},
        {0x31a0, 0x31bf, 1},
    }
}

rt! {
    name: _BRAHMI,
    r32: {
        {0x11000, 0x1104d, 1},
        {0x11052, 0x1106f, 1},
        {0x1107f, 0x1107f, 1},
    }
}

rt! {
    name: _BRAILLE,
    r16: {
        {0x2800, 0x28ff, 1},
    }
}

rt! {
    name: _BUGINESE,
    r16: {
        {0x1a00, 0x1a1b, 1},
        {0x1a1e, 0x1a1f, 1},
    }
}

rt! {
    name: _BUHID,
    r16: {
        {0x1740, 0x1753, 1},
    }
}

rt! {
    name: _CANADIAN_ABORIGINAL,
    r16: {
        {0x1400, 0x167f, 1},
        {0x18b0, 0x18f5, 1},
    }
}

rt! {
    name: _CARIAN,
    r32: {
        {0x102a0, 0x102d0, 1},
    }
}

rt! {
    name: _CAUCASIAN_ALBANIAN,
    r32: {
        {0x10530, 0x10563, 1},
        {0x1056f, 0x1056f, 1},
    }
}

rt! {
    name: _CHAKMA,
    r32: {
        {0x11100, 0x11134, 1},
        {0x11136, 0x11147, 1},
    }
}

rt! {
    name: _CHAM,
    r16: {
        {0xaa00, 0xaa36, 1},
        {0xaa40, 0xaa4d, 1},
        {0xaa50, 0xaa59, 1},
        {0xaa5c, 0xaa5f, 1},
    }
}

rt! {
    name: _CHEROKEE,
    r16: {
        {0x13a0, 0x13f5, 1},
        {0x13f8, 0x13fd, 1},
        {0xab70, 0xabbf, 1},
    }
}

rt! {
    name: _CHORASMIAN,
    r32: {
        {0x10fb0, 0x10fcb, 1},
    }
}

rt! {
    name: _COMMON,
    r16: {
        {0x0000, 0x0040, 1},
        {0x005b, 0x0060, 1},
        {0x007b, 0x00a9, 1},
        {0x00ab, 0x00b9, 1},
        {0x00bb, 0x00bf, 1},
        {0x00d7, 0x00f7, 32},
        {0x02b9, 0x02df, 1},
        {0x02e5, 0x02e9, 1},
        {0x02ec, 0x02ff, 1},
        {0x0374, 0x037e, 10},
        {0x0385, 0x0387, 2},
        {0x0605, 0x060c, 7},
        {0x061b, 0x061f, 4},
        {0x0640, 0x06dd, 157},
        {0x08e2, 0x0964, 130},
        {0x0965, 0x0e3f, 1242},
        {0x0fd5, 0x0fd8, 1},
        {0x10fb, 0x16eb, 1520},
        {0x16ec, 0x16ed, 1},
        {0x1735, 0x1736, 1},
        {0x1802, 0x1803, 1},
        {0x1805, 0x1cd3, 1230},
        {0x1ce1, 0x1ce9, 8},
        {0x1cea, 0x1cec, 1},
        {0x1cee, 0x1cf3, 1},
        {0x1cf5, 0x1cf7, 1},
        {0x1cfa, 0x2000, 774},
        {0x2001, 0x200b, 1},
        {0x200e, 0x2064, 1},
        {0x2066, 0x2070, 1},
        {0x2074, 0x207e, 1},
        {0x2080, 0x208e, 1},
        {0x20a0, 0x20bf, 1},
        {0x2100, 0x2125, 1},
        {0x2127, 0x2129, 1},
        {0x212c, 0x2131, 1},
        {0x2133, 0x214d, 1},
        {0x214f, 0x215f, 1},
        {0x2189, 0x218b, 1},
        {0x2190, 0x2426, 1},
        {0x2440, 0x244a, 1},
        {0x2460, 0x27ff, 1},
        {0x2900, 0x2b73, 1},
        {0x2b76, 0x2b95, 1},
        {0x2b97, 0x2bff, 1},
        {0x2e00, 0x2e52, 1},
        {0x2ff0, 0x2ffb, 1},
        {0x3000, 0x3004, 1},
        {0x3006, 0x3008, 2},
        {0x3009, 0x3020, 1},
        {0x3030, 0x3037, 1},
        {0x303c, 0x303f, 1},
        {0x309b, 0x309c, 1},
        {0x30a0, 0x30fb, 91},
        {0x30fc, 0x3190, 148},
        {0x3191, 0x319f, 1},
        {0x31c0, 0x31e3, 1},
        {0x3220, 0x325f, 1},
        {0x327f, 0x32cf, 1},
        {0x32ff, 0x3358, 89},
        {0x3359, 0x33ff, 1},
        {0x4dc0, 0x4dff, 1},
        {0xa700, 0xa721, 1},
        {0xa788, 0xa78a, 1},
        {0xa830, 0xa839, 1},
        {0xa92e, 0xa9cf, 161},
        {0xab5b, 0xab6a, 15},
        {0xab6b, 0xfd3e, 20947},
        {0xfd3f, 0xfe10, 209},
        {0xfe11, 0xfe19, 1},
        {0xfe30, 0xfe52, 1},
        {0xfe54, 0xfe66, 1},
        {0xfe68, 0xfe6b, 1},
        {0xfeff, 0xff01, 2},
        {0xff02, 0xff20, 1},
        {0xff3b, 0xff40, 1},
        {0xff5b, 0xff65, 1},
        {0xff70, 0xff9e, 46},
        {0xff9f, 0xffe0, 65},
        {0xffe1, 0xffe6, 1},
        {0xffe8, 0xffee, 1},
        {0xfff9, 0xfffd, 1},
    },
    r32: {
        {0x10100, 0x10102, 1},
        {0x10107, 0x10133, 1},
        {0x10137, 0x1013f, 1},
        {0x10190, 0x1019c, 1},
        {0x101d0, 0x101fc, 1},
        {0x102e1, 0x102fb, 1},
        {0x16fe2, 0x16fe3, 1},
        {0x1bca0, 0x1bca3, 1},
        {0x1d000, 0x1d0f5, 1},
        {0x1d100, 0x1d126, 1},
        {0x1d129, 0x1d166, 1},
        {0x1d16a, 0x1d17a, 1},
        {0x1d183, 0x1d184, 1},
        {0x1d18c, 0x1d1a9, 1},
        {0x1d1ae, 0x1d1e8, 1},
        {0x1d2e0, 0x1d2f3, 1},
        {0x1d300, 0x1d356, 1},
        {0x1d360, 0x1d378, 1},
        {0x1d400, 0x1d454, 1},
        {0x1d456, 0x1d49c, 1},
        {0x1d49e, 0x1d49f, 1},
        {0x1d4a2, 0x1d4a5, 3},
        {0x1d4a6, 0x1d4a9, 3},
        {0x1d4aa, 0x1d4ac, 1},
        {0x1d4ae, 0x1d4b9, 1},
        {0x1d4bb, 0x1d4bd, 2},
        {0x1d4be, 0x1d4c3, 1},
        {0x1d4c5, 0x1d505, 1},
        {0x1d507, 0x1d50a, 1},
        {0x1d50d, 0x1d514, 1},
        {0x1d516, 0x1d51c, 1},
        {0x1d51e, 0x1d539, 1},
        {0x1d53b, 0x1d53e, 1},
        {0x1d540, 0x1d544, 1},
        {0x1d546, 0x1d54a, 4},
        {0x1d54b, 0x1d550, 1},
        {0x1d552, 0x1d6a5, 1},
        {0x1d6a8, 0x1d7cb, 1},
        {0x1d7ce, 0x1d7ff, 1},
        {0x1ec71, 0x1ecb4, 1},
        {0x1ed01, 0x1ed3d, 1},
        {0x1f000, 0x1f02b, 1},
        {0x1f030, 0x1f093, 1},
        {0x1f0a0, 0x1f0ae, 1},
        {0x1f0b1, 0x1f0bf, 1},
        {0x1f0c1, 0x1f0cf, 1},
        {0x1f0d1, 0x1f0f5, 1},
        {0x1f100, 0x1f1ad, 1},
        {0x1f1e6, 0x1f1ff, 1},
        {0x1f201, 0x1f202, 1},
        {0x1f210, 0x1f23b, 1},
        {0x1f240, 0x1f248, 1},
        {0x1f250, 0x1f251, 1},
        {0x1f260, 0x1f265, 1},
        {0x1f300, 0x1f6d7, 1},
        {0x1f6e0, 0x1f6ec, 1},
        {0x1f6f0, 0x1f6fc, 1},
        {0x1f700, 0x1f773, 1},
        {0x1f780, 0x1f7d8, 1},
        {0x1f7e0, 0x1f7eb, 1},
        {0x1f800, 0x1f80b, 1},
        {0x1f810, 0x1f847, 1},
        {0x1f850, 0x1f859, 1},
        {0x1f860, 0x1f887, 1},
        {0x1f890, 0x1f8ad, 1},
        {0x1f8b0, 0x1f8b1, 1},
        {0x1f900, 0x1f978, 1},
        {0x1f97a, 0x1f9cb, 1},
        {0x1f9cd, 0x1fa53, 1},
        {0x1fa60, 0x1fa6d, 1},
        {0x1fa70, 0x1fa74, 1},
        {0x1fa78, 0x1fa7a, 1},
        {0x1fa80, 0x1fa86, 1},
        {0x1fa90, 0x1faa8, 1},
        {0x1fab0, 0x1fab6, 1},
        {0x1fac0, 0x1fac2, 1},
        {0x1fad0, 0x1fad6, 1},
        {0x1fb00, 0x1fb92, 1},
        {0x1fb94, 0x1fbca, 1},
        {0x1fbf0, 0x1fbf9, 1},
        {0xe0001, 0xe0020, 31},
        {0xe0021, 0xe007f, 1},
    },
    latin_offset: 6,
}

rt! {
    name: _COPTIC,
    r16: {
        {0x03e2, 0x03ef, 1},
        {0x2c80, 0x2cf3, 1},
        {0x2cf9, 0x2cff, 1},
    }
}

rt! {
    name: _CUNEIFORM,
    r32: {
        {0x12000, 0x12399, 1},
        {0x12400, 0x1246e, 1},
        {0x12470, 0x12474, 1},
        {0x12480, 0x12543, 1},
    }
}

rt! {
    name: _CYPRIOT,
    r32: {
        {0x10800, 0x10805, 1},
        {0x10808, 0x1080a, 2},
        {0x1080b, 0x10835, 1},
        {0x10837, 0x10838, 1},
        {0x1083c, 0x1083f, 3},
    }
}

rt! {
    name: _CYRILLIC,
    r16: {
        {0x0400, 0x0484, 1},
        {0x0487, 0x052f, 1},
        {0x1c80, 0x1c88, 1},
        {0x1d2b, 0x1d78, 77},
        {0x2de0, 0x2dff, 1},
        {0xa640, 0xa69f, 1},
        {0xfe2e, 0xfe2f, 1},
    }
}

rt! {
    name: _DESERET,
    r32: {
        {0x10400, 0x1044f, 1},
    }
}

rt! {
    name: _DEVANAGARI,
    r16: {
        {0x0900, 0x0950, 1},
        {0x0955, 0x0963, 1},
        {0x0966, 0x097f, 1},
        {0xa8e0, 0xa8ff, 1},
    }
}

rt! {
    name: _DIVES_AKURU,
    r32: {
        {0x11900, 0x11906, 1},
        {0x11909, 0x1190c, 3},
        {0x1190d, 0x11913, 1},
        {0x11915, 0x11916, 1},
        {0x11918, 0x11935, 1},
        {0x11937, 0x11938, 1},
        {0x1193b, 0x11946, 1},
        {0x11950, 0x11959, 1},
    }
}

rt! {
    name: _DOGRA,
    r32: {
        {0x11800, 0x1183b, 1},
    }
}

rt! {
    name: _DUPLOYAN,
    r32: {
        {0x1bc00, 0x1bc6a, 1},
        {0x1bc70, 0x1bc7c, 1},
        {0x1bc80, 0x1bc88, 1},
        {0x1bc90, 0x1bc99, 1},
        {0x1bc9c, 0x1bc9f, 1},
    }
}

rt! {
    name: _EGYPTIAN_HIEROGLYPHS,
    r32: {
        {0x13000, 0x1342e, 1},
        {0x13430, 0x13438, 1},
    }
}

rt! {
    name: _ELBASAN,
    r32: {
        {0x10500, 0x10527, 1},
    }
}

rt! {
    name: _ELYMAIC,
    r32: {
        {0x10fe0, 0x10ff6, 1},
    }
}

rt! {
    name: _ETHIOPIC,
    r16: {
        {0x1200, 0x1248, 1},
        {0x124a, 0x124d, 1},
        {0x1250, 0x1256, 1},
        {0x1258, 0x125a, 2},
        {0x125b, 0x125d, 1},
        {0x1260, 0x1288, 1},
        {0x128a, 0x128d, 1},
        {0x1290, 0x12b0, 1},
        {0x12b2, 0x12b5, 1},
        {0x12b8, 0x12be, 1},
        {0x12c0, 0x12c2, 2},
        {0x12c3, 0x12c5, 1},
        {0x12c8, 0x12d6, 1},
        {0x12d8, 0x1310, 1},
        {0x1312, 0x1315, 1},
        {0x1318, 0x135a, 1},
        {0x135d, 0x137c, 1},
        {0x1380, 0x1399, 1},
        {0x2d80, 0x2d96, 1},
        {0x2da0, 0x2da6, 1},
        {0x2da8, 0x2dae, 1},
        {0x2db0, 0x2db6, 1},
        {0x2db8, 0x2dbe, 1},
        {0x2dc0, 0x2dc6, 1},
        {0x2dc8, 0x2dce, 1},
        {0x2dd0, 0x2dd6, 1},
        {0x2dd8, 0x2dde, 1},
        {0xab01, 0xab06, 1},
        {0xab09, 0xab0e, 1},
        {0xab11, 0xab16, 1},
        {0xab20, 0xab26, 1},
        {0xab28, 0xab2e, 1},
    },
}

rt! {
    name: _GEORGIAN,
    r16: {
        {0x10a0, 0x10c5, 1},
        {0x10c7, 0x10cd, 6},
        {0x10d0, 0x10fa, 1},
        {0x10fc, 0x10ff, 1},
        {0x1c90, 0x1cba, 1},
        {0x1cbd, 0x1cbf, 1},
        {0x2d00, 0x2d25, 1},
        {0x2d27, 0x2d2d, 6},
    }
}

rt! {
    name: _GLAGOLITIC,
    r16: {
        {0x2c00, 0x2c2e, 1},
        {0x2c30, 0x2c5e, 1},
    },
    r32: {
        {0x1e000, 0x1e006, 1},
        {0x1e008, 0x1e018, 1},
        {0x1e01b, 0x1e021, 1},
        {0x1e023, 0x1e024, 1},
        {0x1e026, 0x1e02a, 1},
    }
}

rt! {
    name: _GOTHIC,
    r32: {
        {0x10330, 0x1034a, 1},
    }
}

rt! {
    name: _GRANTHA,
    r32: {
        {0x11300, 0x11303, 1},
        {0x11305, 0x1130c, 1},
        {0x1130f, 0x11310, 1},
        {0x11313, 0x11328, 1},
        {0x1132a, 0x11330, 1},
        {0x11332, 0x11333, 1},
        {0x11335, 0x11339, 1},
        {0x1133c, 0x11344, 1},
        {0x11347, 0x11348, 1},
        {0x1134b, 0x1134d, 1},
        {0x11350, 0x11357, 7},
        {0x1135d, 0x11363, 1},
        {0x11366, 0x1136c, 1},
        {0x11370, 0x11374, 1},
    }
}

rt! {
    name: _GREEK,
    r16: {
        {0x0370, 0x0373, 1},
        {0x0375, 0x0377, 1},
        {0x037a, 0x037d, 1},
        {0x037f, 0x0384, 5},
        {0x0386, 0x0388, 2},
        {0x0389, 0x038a, 1},
        {0x038c, 0x038e, 2},
        {0x038f, 0x03a1, 1},
        {0x03a3, 0x03e1, 1},
        {0x03f0, 0x03ff, 1},
        {0x1d26, 0x1d2a, 1},
        {0x1d5d, 0x1d61, 1},
        {0x1d66, 0x1d6a, 1},
        {0x1dbf, 0x1f00, 321},
        {0x1f01, 0x1f15, 1},
        {0x1f18, 0x1f1d, 1},
        {0x1f20, 0x1f45, 1},
        {0x1f48, 0x1f4d, 1},
        {0x1f50, 0x1f57, 1},
        {0x1f59, 0x1f5f, 2},
        {0x1f60, 0x1f7d, 1},
        {0x1f80, 0x1fb4, 1},
        {0x1fb6, 0x1fc4, 1},
        {0x1fc6, 0x1fd3, 1},
        {0x1fd6, 0x1fdb, 1},
        {0x1fdd, 0x1fef, 1},
        {0x1ff2, 0x1ff4, 1},
        {0x1ff6, 0x1ffe, 1},
        {0x2126, 0xab65, 35391},
    },
    r32: {
        {0x10140, 0x1018e, 1},
        {0x101a0, 0x1d200, 53344},
        {0x1d201, 0x1d245, 1},
    }
}

rt! {
    name: _GUJARATI,
    r16: {
        {0x0a81, 0x0a83, 1},
        {0x0a85, 0x0a8d, 1},
        {0x0a8f, 0x0a91, 1},
        {0x0a93, 0x0aa8, 1},
        {0x0aaa, 0x0ab0, 1},
        {0x0ab2, 0x0ab3, 1},
        {0x0ab5, 0x0ab9, 1},
        {0x0abc, 0x0ac5, 1},
        {0x0ac7, 0x0ac9, 1},
        {0x0acb, 0x0acd, 1},
        {0x0ad0, 0x0ae0, 16},
        {0x0ae1, 0x0ae3, 1},
        {0x0ae6, 0x0af1, 1},
        {0x0af9, 0x0aff, 1},
    }
}

rt! {
    name: _GUNJALA_GONDI,
    r32: {
        {0x11d60, 0x11d65, 1},
        {0x11d67, 0x11d68, 1},
        {0x11d6a, 0x11d8e, 1},
        {0x11d90, 0x11d91, 1},
        {0x11d93, 0x11d98, 1},
        {0x11da0, 0x11da9, 1},
    }
}

rt! {
    name: _GURMUKHI,
    r16: {
        {0x0a01, 0x0a03, 1},
        {0x0a05, 0x0a0a, 1},
        {0x0a0f, 0x0a10, 1},
        {0x0a13, 0x0a28, 1},
        {0x0a2a, 0x0a30, 1},
        {0x0a32, 0x0a33, 1},
        {0x0a35, 0x0a36, 1},
        {0x0a38, 0x0a39, 1},
        {0x0a3c, 0x0a3e, 2},
        {0x0a3f, 0x0a42, 1},
        {0x0a47, 0x0a48, 1},
        {0x0a4b, 0x0a4d, 1},
        {0x0a51, 0x0a59, 8},
        {0x0a5a, 0x0a5c, 1},
        {0x0a5e, 0x0a66, 8},
        {0x0a67, 0x0a76, 1},
    }
}

rt! {
    name: _HAN,
    r16: {
        {0x2e80, 0x2e99, 1},
        {0x2e9b, 0x2ef3, 1},
        {0x2f00, 0x2fd5, 1},
        {0x3005, 0x3007, 2},
        {0x3021, 0x3029, 1},
        {0x3038, 0x303b, 1},
        {0x3400, 0x4dbf, 1},
        {0x4e00, 0x9ffc, 1},
        {0xf900, 0xfa6d, 1},
        {0xfa70, 0xfad9, 1},
    },
    r32: {
        {0x16ff0, 0x16ff1, 1},
        {0x20000, 0x2a6dd, 1},
        {0x2a700, 0x2b734, 1},
        {0x2b740, 0x2b81d, 1},
        {0x2b820, 0x2cea1, 1},
        {0x2ceb0, 0x2ebe0, 1},
        {0x2f800, 0x2fa1d, 1},
        {0x30000, 0x3134a, 1},
    }
}

rt! {
    name: _HANGUL,
    r16: {
        {0x1100, 0x11ff, 1},
        {0x302e, 0x302f, 1},
        {0x3131, 0x318e, 1},
        {0x3200, 0x321e, 1},
        {0x3260, 0x327e, 1},
        {0xa960, 0xa97c, 1},
        {0xac00, 0xd7a3, 1},
        {0xd7b0, 0xd7c6, 1},
        {0xd7cb, 0xd7fb, 1},
        {0xffa0, 0xffbe, 1},
        {0xffc2, 0xffc7, 1},
        {0xffca, 0xffcf, 1},
        {0xffd2, 0xffd7, 1},
        {0xffda, 0xffdc, 1},
    }
}

rt! {
    name: _HANIFI_ROHINGYA,
    r32: {
        {0x10d00, 0x10d27, 1},
        {0x10d30, 0x10d39, 1},
    }
}

rt! {
    name: _HANUNOO,
    r16: {
        {0x1720, 0x1734, 1},
    }
}

rt! {
    name: _HATRAN,
    r32: {
        {0x108e0, 0x108f2, 1},
        {0x108f4, 0x108f5, 1},
        {0x108fb, 0x108ff, 1},
    }
}

rt! {
    name: _HEBREW,
    r16: {
        {0x0591, 0x05c7, 1},
        {0x05d0, 0x05ea, 1},
        {0x05ef, 0x05f4, 1},
        {0xfb1d, 0xfb36, 1},
        {0xfb38, 0xfb3c, 1},
        {0xfb3e, 0xfb40, 2},
        {0xfb41, 0xfb43, 2},
        {0xfb44, 0xfb46, 2},
        {0xfb47, 0xfb4f, 1},
    }
}

rt! {
    name: _HIRAGANA,
    r16: {
        {0x3041, 0x3096, 1},
        {0x309d, 0x309f, 1},
    },
    r32: {
        {0x1b001, 0x1b11e, 1},
        {0x1b150, 0x1b152, 1},
        {0x1f200, 0x1f200, 1},
    }
}

rt! {
    name: _IMPERIAL_ARAMAIC,
    r32: {
        {0x10840, 0x10855, 1},
        {0x10857, 0x1085f, 1},
    }
}

rt! {
    name: _INHERITED,
    r16: {
        {0x0300, 0x036f, 1},
        {0x0485, 0x0486, 1},
        {0x064b, 0x0655, 1},
        {0x0670, 0x0951, 737},
        {0x0952, 0x0954, 1},
        {0x1ab0, 0x1ac0, 1},
        {0x1cd0, 0x1cd2, 1},
        {0x1cd4, 0x1ce0, 1},
        {0x1ce2, 0x1ce8, 1},
        {0x1ced, 0x1cf4, 7},
        {0x1cf8, 0x1cf9, 1},
        {0x1dc0, 0x1df9, 1},
        {0x1dfb, 0x1dff, 1},
        {0x200c, 0x200d, 1},
        {0x20d0, 0x20f0, 1},
        {0x302a, 0x302d, 1},
        {0x3099, 0x309a, 1},
        {0xfe00, 0xfe0f, 1},
        {0xfe20, 0xfe2d, 1},
    },
    r32: {
        {0x101fd, 0x102e0, 227},
        {0x1133b, 0x1d167, 48684},
        {0x1d168, 0x1d169, 1},
        {0x1d17b, 0x1d182, 1},
        {0x1d185, 0x1d18b, 1},
        {0x1d1aa, 0x1d1ad, 1},
        {0xe0100, 0xe01ef, 1},
    }
}

rt! {
    name: _INSCRIPTIONAL_PAHLAVI,
    r32: {
        {0x10b60, 0x10b72, 1},
        {0x10b78, 0x10b7f, 1},
    }
}

rt! {
    name: _INSCRIPTIONAL_PARTHIAN,
    r32: {
        {0x10b40, 0x10b55, 1},
        {0x10b58, 0x10b5f, 1},
    }
}

rt! {
    name: _JAVANESE,
    r16: {
        {0xa980, 0xa9cd, 1},
        {0xa9d0, 0xa9d9, 1},
        {0xa9de, 0xa9df, 1},
    }
}

rt! {
    name: _KAITHI,
    r32: {
        {0x11080, 0x110c1, 1},
        {0x110cd, 0x110cd, 1},
    }
}

rt! {
    name: _KANNADA,
    r16: {
        {0x0c80, 0x0c8c, 1},
        {0x0c8e, 0x0c90, 1},
        {0x0c92, 0x0ca8, 1},
        {0x0caa, 0x0cb3, 1},
        {0x0cb5, 0x0cb9, 1},
        {0x0cbc, 0x0cc4, 1},
        {0x0cc6, 0x0cc8, 1},
        {0x0cca, 0x0ccd, 1},
        {0x0cd5, 0x0cd6, 1},
        {0x0cde, 0x0ce0, 2},
        {0x0ce1, 0x0ce3, 1},
        {0x0ce6, 0x0cef, 1},
        {0x0cf1, 0x0cf2, 1},
    }
}

rt! {
    name: _KATAKANA,
    r16: {
        {0x30a1, 0x30fa, 1},
        {0x30fd, 0x30ff, 1},
        {0x31f0, 0x31ff, 1},
        {0x32d0, 0x32fe, 1},
        {0x3300, 0x3357, 1},
        {0xff66, 0xff6f, 1},
        {0xff71, 0xff9d, 1},
    },
    r32: {
        {0x1b000, 0x1b164, 356},
        {0x1b165, 0x1b167, 1},
    }
}

rt! {
    name: _KAYAH_LI,
    r16: {
        {0xa900, 0xa92d, 1},
        {0xa92f, 0xa92f, 1},
    }
}

rt! {
    name: _KHAROSHTHI,
    r32: {
        {0x10a00, 0x10a03, 1},
        {0x10a05, 0x10a06, 1},
        {0x10a0c, 0x10a13, 1},
        {0x10a15, 0x10a17, 1},
        {0x10a19, 0x10a35, 1},
        {0x10a38, 0x10a3a, 1},
        {0x10a3f, 0x10a48, 1},
        {0x10a50, 0x10a58, 1},
    }
}

rt! {
    name: _KHITAN_SMALL_SCRIPT,
    r32: {
        {0x16fe4, 0x18b00, 6940},
        {0x18b01, 0x18cd5, 1},
    }
}

rt! {
    name: _KHMER,
    r16: {
        {0x1780, 0x17dd, 1},
        {0x17e0, 0x17e9, 1},
        {0x17f0, 0x17f9, 1},
        {0x19e0, 0x19ff, 1},
    }
}

rt! {
    name: _KHOJKI,
    r32: {
        {0x11200, 0x11211, 1},
        {0x11213, 0x1123e, 1},
    }
}

rt! {
    name: _KHUDAWADI,
    r32: {
        {0x112b0, 0x112ea, 1},
        {0x112f0, 0x112f9, 1},
    }
}

rt! {
    name: _LAO,
    r16: {
        {0x0e81, 0x0e82, 1},
        {0x0e84, 0x0e86, 2},
        {0x0e87, 0x0e8a, 1},
        {0x0e8c, 0x0ea3, 1},
        {0x0ea5, 0x0ea7, 2},
        {0x0ea8, 0x0ebd, 1},
        {0x0ec0, 0x0ec4, 1},
        {0x0ec6, 0x0ec8, 2},
        {0x0ec9, 0x0ecd, 1},
        {0x0ed0, 0x0ed9, 1},
        {0x0edc, 0x0edf, 1},
    }
}

rt! {
    name: _LATIN,
    r16: {
        {0x0041, 0x005a, 1},
        {0x0061, 0x007a, 1},
        {0x00aa, 0x00ba, 16},
        {0x00c0, 0x00d6, 1},
        {0x00d8, 0x00f6, 1},
        {0x00f8, 0x02b8, 1},
        {0x02e0, 0x02e4, 1},
        {0x1d00, 0x1d25, 1},
        {0x1d2c, 0x1d5c, 1},
        {0x1d62, 0x1d65, 1},
        {0x1d6b, 0x1d77, 1},
        {0x1d79, 0x1dbe, 1},
        {0x1e00, 0x1eff, 1},
        {0x2071, 0x207f, 14},
        {0x2090, 0x209c, 1},
        {0x212a, 0x212b, 1},
        {0x2132, 0x214e, 28},
        {0x2160, 0x2188, 1},
        {0x2c60, 0x2c7f, 1},
        {0xa722, 0xa787, 1},
        {0xa78b, 0xa7bf, 1},
        {0xa7c2, 0xa7ca, 1},
        {0xa7f5, 0xa7ff, 1},
        {0xab30, 0xab5a, 1},
        {0xab5c, 0xab64, 1},
        {0xab66, 0xab69, 1},
        {0xfb00, 0xfb06, 1},
        {0xff21, 0xff3a, 1},
        {0xff41, 0xff5a, 1},
    },
    latin_offset: 5,
}

rt! {
    name: _LEPCHA,
    r16: {
        {0x1c00, 0x1c37, 1},
        {0x1c3b, 0x1c49, 1},
        {0x1c4d, 0x1c4f, 1},
    }
}

rt! {
    name: _LIMBU,
    r16: {
        {0x1900, 0x191e, 1},
        {0x1920, 0x192b, 1},
        {0x1930, 0x193b, 1},
        {0x1940, 0x1944, 4},
        {0x1945, 0x194f, 1},
    }
}

rt! {
    name: _LINEAR_A,
    r32: {
        {0x10600, 0x10736, 1},
        {0x10740, 0x10755, 1},
        {0x10760, 0x10767, 1},
    }
}

rt! {
    name: _LINEAR_B,
    r32: {
        {0x10000, 0x1000b, 1},
        {0x1000d, 0x10026, 1},
        {0x10028, 0x1003a, 1},
        {0x1003c, 0x1003d, 1},
        {0x1003f, 0x1004d, 1},
        {0x10050, 0x1005d, 1},
        {0x10080, 0x100fa, 1},
    }
}

rt! {
    name: _LISU,
    r16: {
        {0xa4d0, 0xa4ff, 1},
    },
    r32: {
        {0x11fb0, 0x11fb0, 1},
    }
}

rt! {
    name: _LYCIAN,
    r32: {
        {0x10280, 0x1029c, 1},
    }
}

rt! {
    name: _LYDIAN,
    r32: {
        {0x10920, 0x10939, 1},
        {0x1093f, 0x1093f, 1},
    }
}

rt! {
    name: _MAHAJANI,
    r32: {
        {0x11150, 0x11176, 1},
    }
}

rt! {
    name: _MAKASAR,
    r32: {
        {0x11ee0, 0x11ef8, 1},
    }
}

rt! {
    name: _MALAYALAM,
    r16: {
        {0x0d00, 0x0d0c, 1},
        {0x0d0e, 0x0d10, 1},
        {0x0d12, 0x0d44, 1},
        {0x0d46, 0x0d48, 1},
        {0x0d4a, 0x0d4f, 1},
        {0x0d54, 0x0d63, 1},
        {0x0d66, 0x0d7f, 1},
    }
}

rt! {
    name: _MANDAIC,
    r16: {
        {0x0840, 0x085b, 1},
        {0x085e, 0x085e, 1},
    }
}

rt! {
    name: _MANICHAEAN,
    r32: {
        {0x10ac0, 0x10ae6, 1},
        {0x10aeb, 0x10af6, 1},
    }
}

rt! {
    name: _MARCHEN,
    r32: {
        {0x11c70, 0x11c8f, 1},
        {0x11c92, 0x11ca7, 1},
        {0x11ca9, 0x11cb6, 1},
    }
}

rt! {
    name: _MASARAM_GONDI,
    r32: {
        {0x11d00, 0x11d06, 1},
        {0x11d08, 0x11d09, 1},
        {0x11d0b, 0x11d36, 1},
        {0x11d3a, 0x11d3c, 2},
        {0x11d3d, 0x11d3f, 2},
        {0x11d40, 0x11d47, 1},
        {0x11d50, 0x11d59, 1},
    }
}

rt! {
    name: _MEDEFAIDRIN,
    r32: {
        {0x16e40, 0x16e9a, 1},
    }
}

rt! {
    name: _MEETEI_MAYEK,
    r16: {
        {0xaae0, 0xaaf6, 1},
        {0xabc0, 0xabed, 1},
        {0xabf0, 0xabf9, 1},
    },
}

rt! {
    name: _MENDE_KIKAKUI,
    r32: {
        {0x1e800, 0x1e8c4, 1},
        {0x1e8c7, 0x1e8d6, 1},
    }
}

rt! {
    name: _MEROITIC_CURSIVE,
    r32: {
        {0x109a0, 0x109b7, 1},
        {0x109bc, 0x109cf, 1},
        {0x109d2, 0x109ff, 1},
    }
}

rt! {
    name: _MEROITIC_HIEROGLYPHS,
    r32: {
        {0x10980, 0x1099f, 1},
    }
}

rt! {
    name: _MIAO,
    r32: {
        {0x16f00, 0x16f4a, 1},
        {0x16f4f, 0x16f87, 1},
        {0x16f8f, 0x16f9f, 1},
    }
}

rt! {
    name: _MODI,
    r32: {
        {0x11600, 0x11644, 1},
        {0x11650, 0x11659, 1},
    }
}

rt! {
    name: _MONGOLIAN,
    r16: {
        {0x1800, 0x1801, 1},
        {0x1804, 0x1806, 2},
        {0x1807, 0x180e, 1},
        {0x1810, 0x1819, 1},
        {0x1820, 0x1878, 1},
        {0x1880, 0x18aa, 1},
    },
    r32: {
        {0x11660, 0x1166c, 1},
    }
}

rt! {
    name: _MRO,
    r32: {
        {0x16a40, 0x16a5e, 1},
        {0x16a60, 0x16a69, 1},
        {0x16a6e, 0x16a6f, 1},
    }
}

rt! {
    name: _MULTANI,
    r32: {
        {0x11280, 0x11286, 1},
        {0x11288, 0x1128a, 2},
        {0x1128b, 0x1128d, 1},
        {0x1128f, 0x1129d, 1},
        {0x1129f, 0x112a9, 1},
    }
}

rt! {
    name: _MYANMAR,
    r16: {
        {0x1000, 0x109f, 1},
        {0xa9e0, 0xa9fe, 1},
        {0xaa60, 0xaa7f, 1},
    }
}

rt! {
    name: _NABATAEAN,
    r32: {
        {0x10880, 0x1089e, 1},
        {0x108a7, 0x108af, 1},
    }
}

rt! {
    name: _NANDINAGARI,
    r32: {
        {0x119a0, 0x119a7, 1},
        {0x119aa, 0x119d7, 1},
        {0x119da, 0x119e4, 1},
    }
}

rt! {
    name: _NEW_TAI_LUE,
    r16: {
        {0x1980, 0x19ab, 1},
        {0x19b0, 0x19c9, 1},
        {0x19d0, 0x19da, 1},
        {0x19de, 0x19df, 1},
    }
}

rt! {
    name: _NEWA,
    r32: {
        {0x11400, 0x1145b, 1},
        {0x1145d, 0x11461, 1},
    }
}

rt! {
    name: _NKO,
    r16: {
        {0x07c0, 0x07fa, 1},
        {0x07fd, 0x07ff, 1},
    }
}

rt! {
    name: _NUSHU,
    r32: {
        {0x16fe1, 0x1b170, 16783},
        {0x1b171, 0x1b2fb, 1},
    }
}

rt! {
    name: _NYIAKENG_PUACHUE_HMONG,
    r32: {
        {0x1e100, 0x1e12c, 1},
        {0x1e130, 0x1e13d, 1},
        {0x1e140, 0x1e149, 1},
        {0x1e14e, 0x1e14f, 1},
    }
}

rt! {
    name: _OGHAM,
    r16: {
        {0x1680, 0x169c, 1},
    }
}

rt! {
    name: _OL_CHIKI,
    r32: {
        {0x1c50, 0x1c7f, 1},
    }
}

rt! {
    name: _OLD_HUNGARIAN,
    r32: {
        {0x10c80, 0x10cb2, 1},
        {0x10cc0, 0x10cf2, 1},
        {0x10cfa, 0x10cff, 1},
    }
}

rt! {
    name: _OLD_ITALIC,
    r32: {
        {0x10300, 0x10323, 1},
        {0x1032d, 0x1032f, 1},
    }
}

rt! {
    name: _OLD_NORTH_ARABIAN,
    r32: {
        {0x10a80, 0x10a9f, 1},
    }
}

rt! {
    name: _OLD_PERMIC,
    r32: {
        {0x10350, 0x1037a, 1},
    }
}

rt! {
    name: _OLD_PERSIAN,
    r32: {
        {0x103a0, 0x103c3, 1},
        {0x103c8, 0x103d5, 1},
    }
}

rt! {
    name: _OLD_SOGDIAN,
    r32: {
        {0x10f00, 0x10f27, 1},
    }
}

rt! {
    name: _OLD_SOUTH_ARABIAN,
    r32: {
        {0x10a60, 0x10a7f, 1},
    }
}

rt! {
    name: _OLD_TURKIC,
    r32: {
        {0x10c00, 0x10c48, 1},
    }
}

rt! {
    name: _ORIYA,
    r16: {
        {0x0b01, 0x0b03, 1},
        {0x0b05, 0x0b0c, 1},
        {0x0b0f, 0x0b10, 1},
        {0x0b13, 0x0b28, 1},
        {0x0b2a, 0x0b30, 1},
        {0x0b32, 0x0b33, 1},
        {0x0b35, 0x0b39, 1},
        {0x0b3c, 0x0b44, 1},
        {0x0b47, 0x0b48, 1},
        {0x0b4b, 0x0b4d, 1},
        {0x0b55, 0x0b57, 1},
        {0x0b5c, 0x0b5d, 1},
        {0x0b5f, 0x0b63, 1},
        {0x0b66, 0x0b77, 1},
    }
}

rt! {
    name: _OSAGE,
    r32: {
        {0x104b0, 0x104d3, 1},
        {0x104d8, 0x104fb, 1},
    }
}

rt! {
    name: _OSMANYA,
    r32: {
        {0x10480, 0x1049d, 1},
        {0x104a0, 0x104a9, 1},
    }
}

rt! {
    name: _PAHAWH_HMONG,
    r32: {
        {0x16b00, 0x16b45, 1},
        {0x16b50, 0x16b59, 1},
        {0x16b5b, 0x16b61, 1},
        {0x16b63, 0x16b77, 1},
        {0x16b7d, 0x16b8f, 1},
    }
}

rt! {
    name: _PALMYRENE,
    r32: {
        {0x10860, 0x1087f, 1},
    }
}

rt! {
    name: _PAU_CIN_HAU,
    r32: {
        {0x11ac0, 0x11af8, 1},
    }
}

rt! {
    name: _PHAGS_PA,
    r16: {
        {0xa840, 0xa877, 1},
    }
}

rt! {
    name: _PHOENICIAN,
    r32: {
        {0x10900, 0x1091b, 1},
        {0x1091f, 0x1091f, 1},
    }
}

rt! {
    name: _PSALTER_PAHLAVI,
    r32: {
        {0x10b80, 0x10b91, 1},
        {0x10b99, 0x10b9c, 1},
        {0x10ba9, 0x10baf, 1},
    }
}

rt! {
    name: _REJANG,
    r16: {
        {0xa930, 0xa953, 1},
        {0xa95f, 0xa95f, 1},
    }
}

rt! {
    name: _RUNIC,
    r16: {
        {0x16a0, 0x16ea, 1},
        {0x16ee, 0x16f8, 1},
    }
}

rt! {
    name: _SAMARITAN,
    r16: {
        {0x0800, 0x082d, 1},
        {0x0830, 0x083e, 1},
    }
}

rt! {
    name: _SAURASHTRA,
    r32: {
        {0xa880, 0xa8c5, 1},
        {0xa8ce, 0xa8d9, 1},
    }
}

rt! {
    name: _SHARADA,
    r32: {
        {0x11180, 0x111df, 1},
    }
}

rt! {
    name: _SHAVIAN,
    r32: {
        {0x10450, 0x1047f, 1},
    }
}

rt! {
    name: _SIDDHAM,
    r32: {
        {0x11580, 0x115b5, 1},
        {0x115b8, 0x115dd, 1},
    }
}

rt! {
    name: _SIGN_WRITING,
    r32: {
        {0x1d800, 0x1da8b, 1},
        {0x1da9b, 0x1da9f, 1},
        {0x1daa1, 0x1daaf, 1},
    }
}

rt! {
    name: _SINHALA,
    r16: {
        {0x0d81, 0x0d83, 1},
        {0x0d85, 0x0d96, 1},
        {0x0d9a, 0x0db1, 1},
        {0x0db3, 0x0dbb, 1},
        {0x0dbd, 0x0dc0, 3},
        {0x0dc1, 0x0dc6, 1},
        {0x0dca, 0x0dcf, 5},
        {0x0dd0, 0x0dd4, 1},
        {0x0dd6, 0x0dd8, 2},
        {0x0dd9, 0x0ddf, 1},
        {0x0de6, 0x0def, 1},
        {0x0df2, 0x0df4, 1},
    },
    r32: {
        {0x111e1, 0x111f4, 1},
    }
}

rt! {
    name: _SOGDIAN,
    r32: {
        {0x10f30, 0x10f59, 1},
    }
}

rt! {
    name: _SORA_SOMPENG,
    r32: {
        {0x110d0, 0x110e8, 1},
        {0x110f0, 0x110f9, 1},
    }
}

rt! {
    name: _SOYOMBO,
    r32: {
        {0x11a50, 0x11aa2, 1},
    }
}

rt! {
    name: _SUNDANESE,
    r16: {
        {0x1b80, 0x1bbf, 1},
        {0x1cc0, 0x1cc7, 1},
    }
}

rt! {
    name: _SYLOTI_NAGRI,
    r16: {
        {0xa800, 0xa82c, 1},
    }
}

rt! {
    name: _SYRIAC,
    r16: {
        {0x0700, 0x070d, 1},
        {0x070f, 0x074a, 1},
        {0x074d, 0x074f, 1},
        {0x0860, 0x086a, 1},
    }
}

rt! {
    name: _TAGALOG,
    r16: {
        {0x1700, 0x170c, 1},
        {0x170e, 0x1714, 1},
    }
}

rt! {
    name: _TAGBANWA,
    r16: {
        {0x1760, 0x176c, 1},
        {0x176e, 0x1770, 1},
        {0x1772, 0x1773, 1},
    }
}

rt! {
    name: _TAI_LE,
    r16: {
        {0x1950, 0x196d, 1},
        {0x1970, 0x1974, 1},
    }
}

rt! {
    name: _TAI_THAM,
    r16: {
        {0x1a20, 0x1a5e, 1},
        {0x1a60, 0x1a7c, 1},
        {0x1a7f, 0x1a89, 1},
        {0x1a90, 0x1a99, 1},
        {0x1aa0, 0x1aad, 1},
    }
}

rt! {
    name: _TAI_VIET,
    r16: {
        {0xaa80, 0xaac2, 1},
        {0xaadb, 0xaadf, 1},
    }
}

rt! {
    name: _TAKRI,
    r32: {
        {0x11680, 0x116b8, 1},
        {0x116c0, 0x116c9, 1},
    }
}

rt! {
    name: _TAMIL,
    r16: {
        {0x0b82, 0x0b83, 1},
        {0x0b85, 0x0b8a, 1},
        {0x0b8e, 0x0b90, 1},
        {0x0b92, 0x0b95, 1},
        {0x0b99, 0x0b9a, 1},
        {0x0b9c, 0x0b9e, 2},
        {0x0b9f, 0x0ba3, 4},
        {0x0ba4, 0x0ba8, 4},
        {0x0ba9, 0x0baa, 1},
        {0x0bae, 0x0bb9, 1},
        {0x0bbe, 0x0bc2, 1},
        {0x0bc6, 0x0bc8, 1},
        {0x0bca, 0x0bcd, 1},
        {0x0bd0, 0x0bd7, 7},
        {0x0be6, 0x0bfa, 1},
    },
    r32: {
        {0x11fc0, 0x11ff1, 1},
        {0x11fff, 0x11fff, 1},
    }
}

rt! {
    name: _TANGUT,
    r32: {
        {0x16fe0, 0x17000, 32},
        {0x17001, 0x187f7, 1},
        {0x18800, 0x18aff, 1},
        {0x18d00, 0x18d08, 1},
    }
}

rt! {
    name: _TELUGU,
    r16: {
        {0x0c00, 0x0c0c, 1},
        {0x0c0e, 0x0c10, 1},
        {0x0c12, 0x0c28, 1},
        {0x0c2a, 0x0c39, 1},
        {0x0c3d, 0x0c44, 1},
        {0x0c46, 0x0c48, 1},
        {0x0c4a, 0x0c4d, 1},
        {0x0c55, 0x0c56, 1},
        {0x0c58, 0x0c5a, 1},
        {0x0c60, 0x0c63, 1},
        {0x0c66, 0x0c6f, 1},
        {0x0c77, 0x0c7f, 1},
    }
}

rt! {
    name: _THAANA,
    r16: {
        {0x0780, 0x07b1, 1},
    }
}

rt! {
    name: _THAI,
    r16: {
        {0x0e01, 0x0e3a, 1},
        {0x0e40, 0x0e5b, 1},
    }
}

rt! {
    name: _TIBETAN,
    r16: {
        {0x0f00, 0x0f47, 1},
        {0x0f49, 0x0f6c, 1},
        {0x0f71, 0x0f97, 1},
        {0x0f99, 0x0fbc, 1},
        {0x0fbe, 0x0fcc, 1},
        {0x0fce, 0x0fd4, 1},
        {0x0fd9, 0x0fda, 1},
    }
}

rt! {
    name: _TIFINAGH,
    r16: {
        {0x2d30, 0x2d67, 1},
        {0x2d6f, 0x2d70, 1},
        {0x2d7f, 0x2d7f, 1},
    }
}

rt! {
    name: _TIRHUTA,
    r32: {
        {0x11480, 0x114c7, 1},
        {0x114d0, 0x114d9, 1},
    }
}

rt! {
    name: _UGARITIC,
    r32: {
        {0x10380, 0x1039d, 1},
        {0x1039f, 0x1039f, 1},
    }
}

rt! {
    name: _VAI,
    r16: {
        {0xa500, 0xa62b, 1},
    }
}

rt! {
    name: _WANCHO,
    r32: {
        {0x1e2c0, 0x1e2f9, 1},
        {0x1e2ff, 0x1e2ff, 1},
    }
}

rt! {
    name: _WARANG_CITI,
    r32: {
        {0x118a0, 0x118f2, 1},
        {0x118ff, 0x118ff, 1},
    }
}

rt! {
    name: _YEZIDI,
    r32: {
        {0x10e80, 0x10ea9, 1},
        {0x10eab, 0x10ead, 1},
        {0x10eb0, 0x10eb1, 1},
    }
}

rt! {
    name: _YI,
    r16: {
        {0xa000, 0xa48c, 1},
        {0xa490, 0xa4c6, 1},
    }
}

rt! {
    name: _ZANABAZAR_SQUARE,
    r32: {
        {0x11a00, 0x11a47, 1},
    }
}

rt_aliases! {
    _ASCII_HEX_DIGIT {
        /// The set of Unicode characters with property ASCII_Hex_Digit.
        ASCII_HEX_DIGIT;
    },
    _BIDI_CONTROL {
        /// The set of Unicode characters with property Bidi_Control.
        BIDI_CONTROL;
    },
    _DASH {
        /// The set of Unicode characters with property Dash.
        DASH;
    },
    _DEPRECATED {
        /// The set of Unicode characters with property Deprecated.
        DEPRECATED;
    },
    _DIACRITIC {
        /// The set of Unicode characters with property Diacritic.
        DIACRITIC;
    },
    _EXTENDER {
        /// The set of Unicode characters with property Extender.
        EXTENDER;
    },
    _HEX_DIGIT {
        /// The set of Unicode characters with property Hex_Digit.
        HEX_DIGIT;
    },
    _HYPHEN {
        /// The set of Unicode characters with property Hyphen.
        HYPHEN;
    },
    _IDS_BINARY_OPERATOR {
        /// The set of Unicode characters with property IDS_Binary_Operator.
        IDS_BINARY_OPERATOR;
    },
    _IDS_TRINARY_OPERATOR {
        /// The set of Unicode characters with property IDS_Trinary_Operator.
        IDS_TRINARY_OPERATOR;
    },
    _IDEOGRAPHIC {
        /// The set of Unicode characters with property Ideographic.
        IDEOGRAPHIC;
    },
    _JOIN_CONTROL {
        /// The set of Unicode characters with property Join_Control.
        JOIN_CONTROL;
    },
    _LOGICAL_ORDER_EXCEPTION {
        /// The set of Unicode characters with property Logical_Order_Exception.
        LOGICAL_ORDER_EXCEPTION;
    },
    _NONCHARACTER_CODE_POINT {
        /// The set of Unicode characters with property Noncharacter_Code_Point.
        NONCHARACTER_CODE_POINT;
    },
    _OTHER_ALPHABETIC {
        /// The set of Unicode characters with property Other_Alphabetic.
        OTHER_ALPHABETIC;
    },
    _OTHER_DEFAULT_IGNORABLE_CODE_POINT {
        /// The set of Unicode characters with property Other_Default_Ignorable_Code_Point.
        OTHER_DEFAULT_IGNORABLE_CODE_POINT;
    },
    _OTHER_GRAPHEME_EXTEND {
        /// The set of Unicode characters with property Other_Grapheme_Extend.
        OTHER_GRAPHEME_EXTEND;
    },
    _OTHER_ID_CONTINUE {
        /// The set of Unicode characters with property Other_ID_Continue.
        OTHER_ID_CONTINUE;
    },
    _OTHER_ID_START {
        /// The set of Unicode characters with property Other_ID_Start.
        OTHER_ID_START;
    },
    _OTHER_LOWERCASE {
        /// The set of Unicode characters with property Other_Lowercase.
        OTHER_LOWERCASE;
    },
    _OTHER_MATH {
        /// The set of Unicode characters with property Other_Math.
        OTHER_MATH;
    },
    _OTHER_UPPERCASE {
        /// The set of Unicode characters with property Other_Uppercase.
        OTHER_UPPERCASE;
    },
    _PATTERN_SYNTAX {
        /// The set of Unicode characters with property Pattern_Syntax.
        PATTERN_SYNTAX;
    },
    _PATTERN_WHITE_SPACE {
        /// The set of Unicode characters with property Pattern_White_Space.
        PATTERN_WHITE_SPACE;
    },
    _PREPENDED_CONCATENATION_MARK {
        /// The set of Unicode characters with property Prepended_Concatenation_Mark.
        PREPENDED_CONCATENATION_MARK;
    },
    _QUOTATION_MARK {
        /// The set of Unicode characters with property Quotation_Mark.
        QUOTATION_MARK;
    },
    _RADICAL {
        /// The set of Unicode characters with property Radical.
        RADICAL;
    },
    _REGIONAL_INDICATOR {
        /// The set of Unicode characters with property Regional_Indicator.
        REGIONAL_INDICATOR;
    },
    _SENTENCE_TERMINAL {
        /// The set of Unicode characters with property Sentence Terminal.
        STERM;
        /// The set of Unicode characters with property Sentence Terminal.
        SENTENCE_TERMINAL;
    },
    _SOFT_DOTTED {
        /// The set of Unicode characters with property Soft_Dotted.
        SOFT_DOTTED;
    },
    _TERMINAL_PUNCTUATION {
        /// The set of Unicode characters with property Terminal_Punctuation.
        TERMINAL_PUNCTUATION;
    },
    _UNIFIED_IDEOGRAPH {
        /// The set of Unicode characters with property Unified_Ideograph.
        UNIFIED_IDEOGRAPH;
    },
    _VARIATION_SELECTOR {
        /// The set of Unicode characters with property Variation_Selector.
        VARIATION_SELECTOR;
    },
    _WHITE_SPACE {
        /// The set of Unicode characters with property White_Space.
        WHITE_SPACE;
    },
}

/// The set of Unicode property tables.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Properties;

impl core::ops::Index<&str> for Properties {
    type Output = RangeTable;

    fn index(&self, key: &str) -> &Self::Output {
        match key {
            "ASCII_Hex_Digit" => _ASCII_HEX_DIGIT,
            "Bidi_Control" => _BIDI_CONTROL,
            "Dash" => _DASH,
            "Deprecated" => _DEPRECATED,
            "Diacritic" => _DIACRITIC,
            "Extender" => _EXTENDER,
            "Hex_Digit" => _HEX_DIGIT,
            "Hyphen" => _HYPHEN,
            "IDS_Binary_Operator" => _IDS_BINARY_OPERATOR,
            "IDS_Trinary_Operator" => _IDS_TRINARY_OPERATOR,
            "Ideographic" => _IDEOGRAPHIC,
            "Join_Control" => _JOIN_CONTROL,
            "Logical_Order_Exception" => _LOGICAL_ORDER_EXCEPTION,
            "Noncharacter_Code_Point" => _NONCHARACTER_CODE_POINT,
            "Other_Alphabetic" => _OTHER_ALPHABETIC,
            "Other_Default_Ignorable_Code_Point" => _OTHER_DEFAULT_IGNORABLE_CODE_POINT,
            "Other_Grapheme_Extend" => _OTHER_GRAPHEME_EXTEND,
            "Other_ID_Continue" => _OTHER_ID_CONTINUE,
            "Other_ID_Start" => _OTHER_ID_START,
            "Other_Lowercase" => _OTHER_LOWERCASE,
            "Other_Math" => _OTHER_MATH,
            "Other_Uppercase" => _OTHER_UPPERCASE,
            "Pattern_Syntax" => _PATTERN_SYNTAX,
            "Pattern_White_Space" => _PATTERN_WHITE_SPACE,
            "Prepended_Concatenation_Mark" => _PREPENDED_CONCATENATION_MARK,
            "Quotation_Mark" => _QUOTATION_MARK,
            "Radical" => _RADICAL,
            "Regional_Indicator" => _REGIONAL_INDICATOR,
            "STerm" | "Sentence_Terminal" => _SENTENCE_TERMINAL,
            "Soft_Dotted" => _SOFT_DOTTED,
            "Terminal_Punctuation" => _TERMINAL_PUNCTUATION,
            "Unified_Ideograph" => _UNIFIED_IDEOGRAPH,
            "Variation_Selector" => _VARIATION_SELECTOR,
            "White_Space" => _WHITE_SPACE,
            _ => panic!("unknown property name"),
        }
    }
}

impl Properties {
    /// Returns the number of properties
    pub const LEN: usize = 35;

    /// Create a new iter on properties
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, (&'static str, &'static RangeTable)> {
        static PROPERTIES: [(&str, &RangeTable); 35] = [
            ("ASCII_Hex_Digit", _ASCII_HEX_DIGIT),
            ("Bidi_Control", _BIDI_CONTROL),
            ("Dash", _DASH),
            ("Deprecated", _DEPRECATED),
            ("Diacritic", _DIACRITIC),
            ("Extender", _EXTENDER),
            ("Hex_Digit", _HEX_DIGIT),
            ("Hyphen", _HYPHEN),
            ("IDS_Binary_Operator", _IDS_BINARY_OPERATOR),
            ("IDS_Trinary_Operator", _IDS_TRINARY_OPERATOR),
            ("Ideographic", _IDEOGRAPHIC),
            ("Join_Control", _JOIN_CONTROL),
            ("Logical_Order_Exception", _LOGICAL_ORDER_EXCEPTION),
            ("Noncharacter_Code_Point", _NONCHARACTER_CODE_POINT),
            ("Other_Alphabetic", _OTHER_ALPHABETIC),
            (
                "Other_Default_Ignorable_Code_Point",
                _OTHER_DEFAULT_IGNORABLE_CODE_POINT,
            ),
            ("Other_Grapheme_Extend", _OTHER_GRAPHEME_EXTEND),
            ("Other_ID_Continue", _OTHER_ID_CONTINUE),
            ("Other_ID_Start", _OTHER_ID_START),
            ("Other_Lowercase", _OTHER_LOWERCASE),
            ("Other_Math", _OTHER_MATH),
            ("Other_Uppercase", _OTHER_UPPERCASE),
            ("Pattern_Syntax", _PATTERN_SYNTAX),
            ("Pattern_White_Space", _PATTERN_WHITE_SPACE),
            (
                "Prepended_Concatenation_Mark",
                _PREPENDED_CONCATENATION_MARK,
            ),
            ("Quotation_Mark", _QUOTATION_MARK),
            ("Radical", _RADICAL),
            ("Regional_Indicator", _REGIONAL_INDICATOR),
            ("STerm", _SENTENCE_TERMINAL),
            ("Sentence_Terminal", _SENTENCE_TERMINAL),
            ("Soft_Dotted", _SOFT_DOTTED),
            ("Terminal_Punctuation", _TERMINAL_PUNCTUATION),
            ("Unified_Ideograph", _UNIFIED_IDEOGRAPH),
            ("Variation_Selector", _VARIATION_SELECTOR),
            ("White_Space", _WHITE_SPACE),
        ];
        PROPERTIES.iter()
    }
}

rt! {
    name: _ASCII_HEX_DIGIT,
    r16: {
        {0x0030, 0x0039, 1},
        {0x0041, 0x0046, 1},
        {0x0061, 0x0066, 1},
    },
    latin_offset: 3,
}

rt! {
    name: _BIDI_CONTROL,
    r16: {
        {0x061c, 0x200e, 6642},
        {0x200f, 0x202a, 27},
        {0x202b, 0x202e, 1},
        {0x2066, 0x2069, 1},
    },
}

rt! {
    name: _DASH,
    r16: {
        {0x002d, 0x058a, 1373},
        {0x05be, 0x1400, 3650},
        {0x1806, 0x2010, 2058},
        {0x2011, 0x2015, 1},
        {0x2053, 0x207b, 40},
        {0x208b, 0x2212, 391},
        {0x2e17, 0x2e1a, 3},
        {0x2e3a, 0x2e3b, 1},
        {0x2e40, 0x301c, 476},
        {0x3030, 0x30a0, 112},
        {0xfe31, 0xfe32, 1},
        {0xfe58, 0xfe63, 11},
        {0xff0d, 0xff0d, 1},
    },
    r32: {
        {0x10ead, 0x10ead, 1},
    }
}

rt! {
    name: _DEPRECATED,
    r16: {
        {0x0149, 0x0673, 1322},
        {0x0f77, 0x0f79, 2},
        {0x17a3, 0x17a4, 1},
        {0x206a, 0x206f, 1},
        {0x2329, 0x232a, 1},
    },
    r32: {
        {0xe0001, 0xe0001, 1},
    }
}

rt! {
    name: _DIACRITIC,
    r16: {
        {0x005e, 0x0060, 2},
        {0x00a8, 0x00af, 7},
        {0x00b4, 0x00b7, 3},
        {0x00b8, 0x02b0, 504},
        {0x02b1, 0x034e, 1},
        {0x0350, 0x0357, 1},
        {0x035d, 0x0362, 1},
        {0x0374, 0x0375, 1},
        {0x037a, 0x0384, 10},
        {0x0385, 0x0483, 254},
        {0x0484, 0x0487, 1},
        {0x0559, 0x0591, 56},
        {0x0592, 0x05a1, 1},
        {0x05a3, 0x05bd, 1},
        {0x05bf, 0x05c1, 2},
        {0x05c2, 0x05c4, 2},
        {0x064b, 0x0652, 1},
        {0x0657, 0x0658, 1},
        {0x06df, 0x06e0, 1},
        {0x06e5, 0x06e6, 1},
        {0x06ea, 0x06ec, 1},
        {0x0730, 0x074a, 1},
        {0x07a6, 0x07b0, 1},
        {0x07eb, 0x07f5, 1},
        {0x0818, 0x0819, 1},
        {0x08e3, 0x08fe, 1},
        {0x093c, 0x094d, 17},
        {0x0951, 0x0954, 1},
        {0x0971, 0x09bc, 75},
        {0x09cd, 0x0a3c, 111},
        {0x0a4d, 0x0abc, 111},
        {0x0acd, 0x0afd, 48},
        {0x0afe, 0x0aff, 1},
        {0x0b3c, 0x0b4d, 17},
        {0x0b55, 0x0bcd, 120},
        {0x0c4d, 0x0cbc, 111},
        {0x0ccd, 0x0d3b, 110},
        {0x0d3c, 0x0d4d, 17},
        {0x0dca, 0x0e47, 125},
        {0x0e48, 0x0e4c, 1},
        {0x0e4e, 0x0eba, 108},
        {0x0ec8, 0x0ecc, 1},
        {0x0f18, 0x0f19, 1},
        {0x0f35, 0x0f39, 2},
        {0x0f3e, 0x0f3f, 1},
        {0x0f82, 0x0f84, 1},
        {0x0f86, 0x0f87, 1},
        {0x0fc6, 0x1037, 113},
        {0x1039, 0x103a, 1},
        {0x1063, 0x1064, 1},
        {0x1069, 0x106d, 1},
        {0x1087, 0x108d, 1},
        {0x108f, 0x109a, 11},
        {0x109b, 0x135d, 706},
        {0x135e, 0x135f, 1},
        {0x17c9, 0x17d3, 1},
        {0x17dd, 0x1939, 348},
        {0x193a, 0x193b, 1},
        {0x1a75, 0x1a7c, 1},
        {0x1a7f, 0x1ab0, 49},
        {0x1ab1, 0x1abd, 1},
        {0x1b34, 0x1b44, 16},
        {0x1b6b, 0x1b73, 1},
        {0x1baa, 0x1bab, 1},
        {0x1c36, 0x1c37, 1},
        {0x1c78, 0x1c7d, 1},
        {0x1cd0, 0x1ce8, 1},
        {0x1ced, 0x1cf4, 7},
        {0x1cf7, 0x1cf9, 1},
        {0x1d2c, 0x1d6a, 1},
        {0x1dc4, 0x1dcf, 1},
        {0x1df5, 0x1df9, 1},
        {0x1dfd, 0x1dff, 1},
        {0x1fbd, 0x1fbf, 2},
        {0x1fc0, 0x1fc1, 1},
        {0x1fcd, 0x1fcf, 1},
        {0x1fdd, 0x1fdf, 1},
        {0x1fed, 0x1fef, 1},
        {0x1ffd, 0x1ffe, 1},
        {0x2cef, 0x2cf1, 1},
        {0x2e2f, 0x302a, 507},
        {0x302b, 0x302f, 1},
        {0x3099, 0x309c, 1},
        {0x30fc, 0xa66f, 30067},
        {0xa67c, 0xa67d, 1},
        {0xa67f, 0xa69c, 29},
        {0xa69d, 0xa6f0, 83},
        {0xa6f1, 0xa700, 15},
        {0xa701, 0xa721, 1},
        {0xa788, 0xa78a, 1},
        {0xa7f8, 0xa7f9, 1},
        {0xa8c4, 0xa8e0, 28},
        {0xa8e1, 0xa8f1, 1},
        {0xa92b, 0xa92e, 1},
        {0xa953, 0xa9b3, 96},
        {0xa9c0, 0xa9e5, 37},
        {0xaa7b, 0xaa7d, 1},
        {0xaabf, 0xaac2, 1},
        {0xaaf6, 0xab5b, 101},
        {0xab5c, 0xab5f, 1},
        {0xab69, 0xab6b, 1},
        {0xabec, 0xabed, 1},
        {0xfb1e, 0xfe20, 770},
        {0xfe21, 0xfe2f, 1},
        {0xff3e, 0xff40, 2},
        {0xff70, 0xff9e, 46},
        {0xff9f, 0xffe3, 68},
    },
    r32: {
        {0x102e0, 0x10ae5, 2053},
        {0x10ae6, 0x10d22, 572},
        {0x10d23, 0x10d27, 1},
        {0x10f46, 0x10f50, 1},
        {0x110b9, 0x110ba, 1},
        {0x11133, 0x11134, 1},
        {0x11173, 0x111c0, 77},
        {0x111ca, 0x111cc, 1},
        {0x11235, 0x11236, 1},
        {0x112e9, 0x112ea, 1},
        {0x1133c, 0x1134d, 17},
        {0x11366, 0x1136c, 1},
        {0x11370, 0x11374, 1},
        {0x11442, 0x11446, 4},
        {0x114c2, 0x114c3, 1},
        {0x115bf, 0x115c0, 1},
        {0x1163f, 0x116b6, 119},
        {0x116b7, 0x1172b, 116},
        {0x11839, 0x1183a, 1},
        {0x1193d, 0x1193e, 1},
        {0x11943, 0x119e0, 157},
        {0x11a34, 0x11a47, 19},
        {0x11a99, 0x11c3f, 422},
        {0x11d42, 0x11d44, 2},
        {0x11d45, 0x11d97, 82},
        {0x16af0, 0x16af4, 1},
        {0x16b30, 0x16b36, 1},
        {0x16f8f, 0x16f9f, 1},
        {0x16ff0, 0x16ff1, 1},
        {0x1d167, 0x1d169, 1},
        {0x1d16d, 0x1d172, 1},
        {0x1d17b, 0x1d182, 1},
        {0x1d185, 0x1d18b, 1},
        {0x1d1aa, 0x1d1ad, 1},
        {0x1e130, 0x1e136, 1},
        {0x1e2ec, 0x1e2ef, 1},
        {0x1e8d0, 0x1e8d6, 1},
        {0x1e944, 0x1e946, 1},
        {0x1e948, 0x1e94a, 1},
    },
    latin_offset: 3,
}

rt! {
    name: _EXTENDER,
    r16: {
        {0x00b7, 0x02d0, 537},
        {0x02d1, 0x0640, 879},
        {0x07fa, 0x0b55, 859},
        {0x0e46, 0x0ec6, 128},
        {0x180a, 0x1843, 57},
        {0x1aa7, 0x1c36, 399},
        {0x1c7b, 0x3005, 5002},
        {0x3031, 0x3035, 1},
        {0x309d, 0x309e, 1},
        {0x30fc, 0x30fe, 1},
        {0xa015, 0xa60c, 1527},
        {0xa9cf, 0xa9e6, 23},
        {0xaa70, 0xaadd, 109},
        {0xaaf3, 0xaaf4, 1},
        {0xff70, 0xff70, 1},
    },
    r32: {
        {0x1135d, 0x115c6, 617},
        {0x115c7, 0x115c8, 1},
        {0x11a98, 0x16b42, 20650},
        {0x16b43, 0x16fe0, 1181},
        {0x16fe1, 0x16fe3, 2},
        {0x1e13c, 0x1e13d, 1},
        {0x1e944, 0x1e946, 1},
    }
}

rt! {
    name: _HEX_DIGIT,
    r16: {
        {0x0030, 0x0039, 1},
        {0x0041, 0x0046, 1},
        {0x0061, 0x0066, 1},
        {0xff10, 0xff19, 1},
        {0xff21, 0xff26, 1},
        {0xff41, 0xff46, 1},
    },
    latin_offset: 3,
}

rt! {
    name: _HYPHEN,
    r16: {
        {0x002d, 0x00ad, 128},
        {0x058a, 0x1806, 4732},
        {0x2010, 0x2011, 1},
        {0x2e17, 0x30fb, 740},
        {0xfe63, 0xff0d, 170},
        {0xff65, 0xff65, 1},
    },
    latin_offset: 1,
}

rt! {
    name: _IDS_BINARY_OPERATOR,
    r16: {
        {0x2ff0, 0x2ff1, 1},
        {0x2ff4, 0x2ffb, 1},
    }
}

rt! {
    name: _IDS_TRINARY_OPERATOR,
    r16: {
        {0x2ff2, 0x2ff3, 1},
    }
}

rt! {
    name: _IDEOGRAPHIC,
    r16: {
        {0x3006, 0x3007, 1},
        {0x3021, 0x3029, 1},
        {0x3038, 0x303a, 1},
        {0x3400, 0x4dbf, 1},
        {0x4e00, 0x9ffc, 1},
        {0xf900, 0xfa6d, 1},
        {0xfa70, 0xfad9, 1},
    },
    r32: {
        {0x16fe4, 0x17000, 28},
        {0x17001, 0x187f7, 1},
        {0x18800, 0x18cd5, 1},
        {0x18d00, 0x18d08, 1},
        {0x1b170, 0x1b2fb, 1},
        {0x20000, 0x2a6dd, 1},
        {0x2a700, 0x2b734, 1},
        {0x2b740, 0x2b81d, 1},
        {0x2b820, 0x2cea1, 1},
        {0x2ceb0, 0x2ebe0, 1},
        {0x2f800, 0x2fa1d, 1},
        {0x30000, 0x3134a, 1},
    }
}

rt! {
    name: _JOIN_CONTROL,
    r16: {
        {0x200c, 0x200d, 1},
    }
}

rt! {
    name: _LOGICAL_ORDER_EXCEPTION,
    r16: {
        {0x0e40, 0x0e44, 1},
        {0x0ec0, 0x0ec4, 1},
        {0x19b5, 0x19b7, 1},
        {0x19ba, 0xaab5, 37115},
        {0xaab6, 0xaab9, 3},
        {0xaabb, 0xaabc, 1},
    }
}

rt! {
    name: _NONCHARACTER_CODE_POINT,
    r16: {
        {0xfdd0, 0xfdef, 1},
        {0xfffe, 0xffff, 1},
    },
    r32: {
        {0x1fffe, 0x1ffff, 1},
        {0x2fffe, 0x2ffff, 1},
        {0x3fffe, 0x3ffff, 1},
        {0x4fffe, 0x4ffff, 1},
        {0x5fffe, 0x5ffff, 1},
        {0x6fffe, 0x6ffff, 1},
        {0x7fffe, 0x7ffff, 1},
        {0x8fffe, 0x8ffff, 1},
        {0x9fffe, 0x9ffff, 1},
        {0xafffe, 0xaffff, 1},
        {0xbfffe, 0xbffff, 1},
        {0xcfffe, 0xcffff, 1},
        {0xdfffe, 0xdffff, 1},
        {0xefffe, 0xeffff, 1},
        {0xffffe, 0xfffff, 1},
        {0x10fffe, 0x10ffff, 1},
    }
}

rt! {
    name: _OTHER_ALPHABETIC,
    r16: {
        {0x0345, 0x05b0, 619},
        {0x05b1, 0x05bd, 1},
        {0x05bf, 0x05c1, 2},
        {0x05c2, 0x05c4, 2},
        {0x05c5, 0x05c7, 2},
        {0x0610, 0x061a, 1},
        {0x064b, 0x0657, 1},
        {0x0659, 0x065f, 1},
        {0x0670, 0x06d6, 102},
        {0x06d7, 0x06dc, 1},
        {0x06e1, 0x06e4, 1},
        {0x06e7, 0x06e8, 1},
        {0x06ed, 0x0711, 36},
        {0x0730, 0x073f, 1},
        {0x07a6, 0x07b0, 1},
        {0x0816, 0x0817, 1},
        {0x081b, 0x0823, 1},
        {0x0825, 0x0827, 1},
        {0x0829, 0x082c, 1},
        {0x08d4, 0x08df, 1},
        {0x08e3, 0x08e9, 1},
        {0x08f0, 0x0903, 1},
        {0x093a, 0x093b, 1},
        {0x093e, 0x094c, 1},
        {0x094e, 0x094f, 1},
        {0x0955, 0x0957, 1},
        {0x0962, 0x0963, 1},
        {0x0981, 0x0983, 1},
        {0x09be, 0x09c4, 1},
        {0x09c7, 0x09c8, 1},
        {0x09cb, 0x09cc, 1},
        {0x09d7, 0x09e2, 11},
        {0x09e3, 0x0a01, 30},
        {0x0a02, 0x0a03, 1},
        {0x0a3e, 0x0a42, 1},
        {0x0a47, 0x0a48, 1},
        {0x0a4b, 0x0a4c, 1},
        {0x0a51, 0x0a70, 31},
        {0x0a71, 0x0a75, 4},
        {0x0a81, 0x0a83, 1},
        {0x0abe, 0x0ac5, 1},
        {0x0ac7, 0x0ac9, 1},
        {0x0acb, 0x0acc, 1},
        {0x0ae2, 0x0ae3, 1},
        {0x0afa, 0x0afc, 1},
        {0x0b01, 0x0b03, 1},
        {0x0b3e, 0x0b44, 1},
        {0x0b47, 0x0b48, 1},
        {0x0b4b, 0x0b4c, 1},
        {0x0b56, 0x0b57, 1},
        {0x0b62, 0x0b63, 1},
        {0x0b82, 0x0bbe, 60},
        {0x0bbf, 0x0bc2, 1},
        {0x0bc6, 0x0bc8, 1},
        {0x0bca, 0x0bcc, 1},
        {0x0bd7, 0x0c00, 41},
        {0x0c01, 0x0c03, 1},
        {0x0c3e, 0x0c44, 1},
        {0x0c46, 0x0c48, 1},
        {0x0c4a, 0x0c4c, 1},
        {0x0c55, 0x0c56, 1},
        {0x0c62, 0x0c63, 1},
        {0x0c81, 0x0c83, 1},
        {0x0cbe, 0x0cc4, 1},
        {0x0cc6, 0x0cc8, 1},
        {0x0cca, 0x0ccc, 1},
        {0x0cd5, 0x0cd6, 1},
        {0x0ce2, 0x0ce3, 1},
        {0x0d00, 0x0d03, 1},
        {0x0d3e, 0x0d44, 1},
        {0x0d46, 0x0d48, 1},
        {0x0d4a, 0x0d4c, 1},
        {0x0d57, 0x0d62, 11},
        {0x0d63, 0x0d81, 30},
        {0x0d82, 0x0d83, 1},
        {0x0dcf, 0x0dd4, 1},
        {0x0dd6, 0x0dd8, 2},
        {0x0dd9, 0x0ddf, 1},
        {0x0df2, 0x0df3, 1},
        {0x0e31, 0x0e34, 3},
        {0x0e35, 0x0e3a, 1},
        {0x0e4d, 0x0eb1, 100},
        {0x0eb4, 0x0eb9, 1},
        {0x0ebb, 0x0ebc, 1},
        {0x0ecd, 0x0f71, 164},
        {0x0f72, 0x0f81, 1},
        {0x0f8d, 0x0f97, 1},
        {0x0f99, 0x0fbc, 1},
        {0x102b, 0x1036, 1},
        {0x1038, 0x103b, 3},
        {0x103c, 0x103e, 1},
        {0x1056, 0x1059, 1},
        {0x105e, 0x1060, 1},
        {0x1062, 0x1064, 1},
        {0x1067, 0x106d, 1},
        {0x1071, 0x1074, 1},
        {0x1082, 0x108d, 1},
        {0x108f, 0x109a, 11},
        {0x109b, 0x109d, 1},
        {0x1712, 0x1713, 1},
        {0x1732, 0x1733, 1},
        {0x1752, 0x1753, 1},
        {0x1772, 0x1773, 1},
        {0x17b6, 0x17c8, 1},
        {0x1885, 0x1886, 1},
        {0x18a9, 0x1920, 119},
        {0x1921, 0x192b, 1},
        {0x1930, 0x1938, 1},
        {0x1a17, 0x1a1b, 1},
        {0x1a55, 0x1a5e, 1},
        {0x1a61, 0x1a74, 1},
        {0x1abf, 0x1ac0, 1},
        {0x1b00, 0x1b04, 1},
        {0x1b35, 0x1b43, 1},
        {0x1b80, 0x1b82, 1},
        {0x1ba1, 0x1ba9, 1},
        {0x1bac, 0x1bad, 1},
        {0x1be7, 0x1bf1, 1},
        {0x1c24, 0x1c36, 1},
        {0x1de7, 0x1df4, 1},
        {0x24b6, 0x24e9, 1},
        {0x2de0, 0x2dff, 1},
        {0xa674, 0xa67b, 1},
        {0xa69e, 0xa69f, 1},
        {0xa802, 0xa80b, 9},
        {0xa823, 0xa827, 1},
        {0xa880, 0xa881, 1},
        {0xa8b4, 0xa8c3, 1},
        {0xa8c5, 0xa8ff, 58},
        {0xa926, 0xa92a, 1},
        {0xa947, 0xa952, 1},
        {0xa980, 0xa983, 1},
        {0xa9b4, 0xa9bf, 1},
        {0xa9e5, 0xaa29, 68},
        {0xaa2a, 0xaa36, 1},
        {0xaa43, 0xaa4c, 9},
        {0xaa4d, 0xaa7b, 46},
        {0xaa7c, 0xaa7d, 1},
        {0xaab0, 0xaab2, 2},
        {0xaab3, 0xaab4, 1},
        {0xaab7, 0xaab8, 1},
        {0xaabe, 0xaaeb, 45},
        {0xaaec, 0xaaef, 1},
        {0xaaf5, 0xabe3, 238},
        {0xabe4, 0xabea, 1},
        {0xfb1e, 0xfb1e, 1},
    },
    r32: {
        {0x10376, 0x1037a, 1},
        {0x10a01, 0x10a03, 1},
        {0x10a05, 0x10a06, 1},
        {0x10a0c, 0x10a0f, 1},
        {0x10d24, 0x10d27, 1},
        {0x10eab, 0x10eac, 1},
        {0x11000, 0x11002, 1},
        {0x11038, 0x11045, 1},
        {0x11082, 0x110b0, 46},
        {0x110b1, 0x110b8, 1},
        {0x11100, 0x11102, 1},
        {0x11127, 0x11132, 1},
        {0x11145, 0x11146, 1},
        {0x11180, 0x11182, 1},
        {0x111b3, 0x111bf, 1},
        {0x111ce, 0x111cf, 1},
        {0x1122c, 0x11234, 1},
        {0x11237, 0x1123e, 7},
        {0x112df, 0x112e8, 1},
        {0x11300, 0x11303, 1},
        {0x1133e, 0x11344, 1},
        {0x11347, 0x11348, 1},
        {0x1134b, 0x1134c, 1},
        {0x11357, 0x11362, 11},
        {0x11363, 0x11435, 210},
        {0x11436, 0x11441, 1},
        {0x11443, 0x11445, 1},
        {0x114b0, 0x114c1, 1},
        {0x115af, 0x115b5, 1},
        {0x115b8, 0x115be, 1},
        {0x115dc, 0x115dd, 1},
        {0x11630, 0x1163e, 1},
        {0x11640, 0x116ab, 107},
        {0x116ac, 0x116b5, 1},
        {0x1171d, 0x1172a, 1},
        {0x1182c, 0x11838, 1},
        {0x11930, 0x11935, 1},
        {0x11937, 0x11938, 1},
        {0x1193b, 0x1193c, 1},
        {0x11940, 0x11942, 2},
        {0x119d1, 0x119d7, 1},
        {0x119da, 0x119df, 1},
        {0x119e4, 0x11a01, 29},
        {0x11a02, 0x11a0a, 1},
        {0x11a35, 0x11a39, 1},
        {0x11a3b, 0x11a3e, 1},
        {0x11a51, 0x11a5b, 1},
        {0x11a8a, 0x11a97, 1},
        {0x11c2f, 0x11c36, 1},
        {0x11c38, 0x11c3e, 1},
        {0x11c92, 0x11ca7, 1},
        {0x11ca9, 0x11cb6, 1},
        {0x11d31, 0x11d36, 1},
        {0x11d3a, 0x11d3c, 2},
        {0x11d3d, 0x11d3f, 2},
        {0x11d40, 0x11d41, 1},
        {0x11d43, 0x11d47, 4},
        {0x11d8a, 0x11d8e, 1},
        {0x11d90, 0x11d91, 1},
        {0x11d93, 0x11d96, 1},
        {0x11ef3, 0x11ef6, 1},
        {0x16f4f, 0x16f51, 2},
        {0x16f52, 0x16f87, 1},
        {0x16f8f, 0x16f92, 1},
        {0x16ff0, 0x16ff1, 1},
        {0x1bc9e, 0x1e000, 9058},
        {0x1e001, 0x1e006, 1},
        {0x1e008, 0x1e018, 1},
        {0x1e01b, 0x1e021, 1},
        {0x1e023, 0x1e024, 1},
        {0x1e026, 0x1e02a, 1},
        {0x1e947, 0x1f130, 2025},
        {0x1f131, 0x1f149, 1},
        {0x1f150, 0x1f169, 1},
        {0x1f170, 0x1f189, 1},
    }
}

rt! {
    name: _OTHER_DEFAULT_IGNORABLE_CODE_POINT,
    r16: {
        {0x034f, 0x115f, 3600},
        {0x1160, 0x17b4, 1620},
        {0x17b5, 0x2065, 2224},
        {0x3164, 0xffa0, 52796},
        {0xfff0, 0xfff8, 1},
    },
    r32: {
        {0xe0000, 0xe0002, 2},
        {0xe0003, 0xe001f, 1},
        {0xe0080, 0xe00ff, 1},
        {0xe01f0, 0xe0fff, 1},
    }
}

rt! {
    name: _OTHER_GRAPHEME_EXTEND,
    r16: {
        {0x09be, 0x09d7, 25},
        {0x0b3e, 0x0b57, 25},
        {0x0bbe, 0x0bd7, 25},
        {0x0cc2, 0x0cd5, 19},
        {0x0cd6, 0x0d3e, 104},
        {0x0d57, 0x0dcf, 120},
        {0x0ddf, 0x1b35, 3414},
        {0x200c, 0x302e, 4130},
        {0x302f, 0xff9e, 53103},
        {0xff9f, 0xff9f, 1},
    },
    r32: {
        {0x1133e, 0x11357, 25},
        {0x114b0, 0x114bd, 13},
        {0x115af, 0x11930, 897},
        {0x1d165, 0x1d16e, 9},
        {0x1d16f, 0x1d172, 1},
        {0xe0020, 0xe007f, 1},
    }
}

rt! {
    name: _OTHER_ID_CONTINUE,
    r16: {
        {0x00b7, 0x0387, 720},
        {0x1369, 0x1371, 1},
        {0x19da, 0x19da, 1},
    },
}

rt! {
    name: _OTHER_ID_START,
    r16: {
        {0x1885, 0x1886, 1},
        {0x2118, 0x212e, 22},
        {0x309b, 0x309c, 1},
    },
}

rt! {
    name: _OTHER_LOWERCASE,
    r16: {
        {0x00aa, 0x00ba, 16},
        {0x02b0, 0x02b8, 1},
        {0x02c0, 0x02c1, 1},
        {0x02e0, 0x02e4, 1},
        {0x0345, 0x037a, 53},
        {0x1d2c, 0x1d6a, 1},
        {0x1d78, 0x1d9b, 35},
        {0x1d9c, 0x1dbf, 1},
        {0x2071, 0x207f, 14},
        {0x2090, 0x209c, 1},
        {0x2170, 0x217f, 1},
        {0x24d0, 0x24e9, 1},
        {0x2c7c, 0x2c7d, 1},
        {0xa69c, 0xa69d, 1},
        {0xa770, 0xa7f8, 136},
        {0xa7f9, 0xab5c, 867},
        {0xab5d, 0xab5f, 1},
    },
    latin_offset: 1,
}

rt! {
    name: _OTHER_MATH,
    r16: {
        {0x005e, 0x03d0, 882},
        {0x03d1, 0x03d2, 1},
        {0x03d5, 0x03f0, 27},
        {0x03f1, 0x03f4, 3},
        {0x03f5, 0x2016, 7201},
        {0x2032, 0x2034, 1},
        {0x2040, 0x2061, 33},
        {0x2062, 0x2064, 1},
        {0x207d, 0x207e, 1},
        {0x208d, 0x208e, 1},
        {0x20d0, 0x20dc, 1},
        {0x20e1, 0x20e5, 4},
        {0x20e6, 0x20eb, 5},
        {0x20ec, 0x20ef, 1},
        {0x2102, 0x2107, 5},
        {0x210a, 0x2113, 1},
        {0x2115, 0x2119, 4},
        {0x211a, 0x211d, 1},
        {0x2124, 0x2128, 4},
        {0x2129, 0x212c, 3},
        {0x212d, 0x212f, 2},
        {0x2130, 0x2131, 1},
        {0x2133, 0x2138, 1},
        {0x213c, 0x213f, 1},
        {0x2145, 0x2149, 1},
        {0x2195, 0x2199, 1},
        {0x219c, 0x219f, 1},
        {0x21a1, 0x21a2, 1},
        {0x21a4, 0x21a5, 1},
        {0x21a7, 0x21a9, 2},
        {0x21aa, 0x21ad, 1},
        {0x21b0, 0x21b1, 1},
        {0x21b6, 0x21b7, 1},
        {0x21bc, 0x21cd, 1},
        {0x21d0, 0x21d1, 1},
        {0x21d3, 0x21d5, 2},
        {0x21d6, 0x21db, 1},
        {0x21dd, 0x21e4, 7},
        {0x21e5, 0x2308, 291},
        {0x2309, 0x230b, 1},
        {0x23b4, 0x23b5, 1},
        {0x23b7, 0x23d0, 25},
        {0x23e2, 0x25a0, 446},
        {0x25a1, 0x25ae, 13},
        {0x25af, 0x25b6, 1},
        {0x25bc, 0x25c0, 1},
        {0x25c6, 0x25c7, 1},
        {0x25ca, 0x25cb, 1},
        {0x25cf, 0x25d3, 1},
        {0x25e2, 0x25e4, 2},
        {0x25e7, 0x25ec, 1},
        {0x2605, 0x2606, 1},
        {0x2640, 0x2642, 2},
        {0x2660, 0x2663, 1},
        {0x266d, 0x266e, 1},
        {0x27c5, 0x27c6, 1},
        {0x27e6, 0x27ef, 1},
        {0x2983, 0x2998, 1},
        {0x29d8, 0x29db, 1},
        {0x29fc, 0x29fd, 1},
        {0xfe61, 0xfe63, 2},
        {0xfe68, 0xff3c, 212},
        {0xff3e, 0xff3e, 1},
    },
    r32: {
        {0x1d400, 0x1d454, 1},
        {0x1d456, 0x1d49c, 1},
        {0x1d49e, 0x1d49f, 1},
        {0x1d4a2, 0x1d4a5, 3},
        {0x1d4a6, 0x1d4a9, 3},
        {0x1d4aa, 0x1d4ac, 1},
        {0x1d4ae, 0x1d4b9, 1},
        {0x1d4bb, 0x1d4bd, 2},
        {0x1d4be, 0x1d4c3, 1},
        {0x1d4c5, 0x1d505, 1},
        {0x1d507, 0x1d50a, 1},
        {0x1d50d, 0x1d514, 1},
        {0x1d516, 0x1d51c, 1},
        {0x1d51e, 0x1d539, 1},
        {0x1d53b, 0x1d53e, 1},
        {0x1d540, 0x1d544, 1},
        {0x1d546, 0x1d54a, 4},
        {0x1d54b, 0x1d550, 1},
        {0x1d552, 0x1d6a5, 1},
        {0x1d6a8, 0x1d6c0, 1},
        {0x1d6c2, 0x1d6da, 1},
        {0x1d6dc, 0x1d6fa, 1},
        {0x1d6fc, 0x1d714, 1},
        {0x1d716, 0x1d734, 1},
        {0x1d736, 0x1d74e, 1},
        {0x1d750, 0x1d76e, 1},
        {0x1d770, 0x1d788, 1},
        {0x1d78a, 0x1d7a8, 1},
        {0x1d7aa, 0x1d7c2, 1},
        {0x1d7c4, 0x1d7cb, 1},
        {0x1d7ce, 0x1d7ff, 1},
        {0x1ee00, 0x1ee03, 1},
        {0x1ee05, 0x1ee1f, 1},
        {0x1ee21, 0x1ee22, 1},
        {0x1ee24, 0x1ee27, 3},
        {0x1ee29, 0x1ee32, 1},
        {0x1ee34, 0x1ee37, 1},
        {0x1ee39, 0x1ee3b, 2},
        {0x1ee42, 0x1ee47, 5},
        {0x1ee49, 0x1ee4d, 2},
        {0x1ee4e, 0x1ee4f, 1},
        {0x1ee51, 0x1ee52, 1},
        {0x1ee54, 0x1ee57, 3},
        {0x1ee59, 0x1ee61, 2},
        {0x1ee62, 0x1ee64, 2},
        {0x1ee67, 0x1ee6a, 1},
        {0x1ee6c, 0x1ee72, 1},
        {0x1ee74, 0x1ee77, 1},
        {0x1ee79, 0x1ee7c, 1},
        {0x1ee7e, 0x1ee80, 2},
        {0x1ee81, 0x1ee89, 1},
        {0x1ee8b, 0x1ee9b, 1},
        {0x1eea1, 0x1eea3, 1},
        {0x1eea5, 0x1eea9, 1},
        {0x1eeab, 0x1eebb, 1},
    }
}

rt! {
    name: _OTHER_UPPERCASE,
    r16: {
        {0x2160, 0x216f, 1},
        {0x24b6, 0x24cf, 1},
    },
    r32: {
        {0x1f130, 0x1f149, 1},
        {0x1f150, 0x1f169, 1},
        {0x1f170, 0x1f189, 1},
    }
}

rt! {
    name: _PATTERN_SYNTAX,
    r16: {
        {0x0021, 0x002f, 1},
        {0x003a, 0x0040, 1},
        {0x005b, 0x005e, 1},
        {0x0060, 0x007b, 27},
        {0x007c, 0x007e, 1},
        {0x00a1, 0x00a7, 1},
        {0x00a9, 0x00ab, 2},
        {0x00ac, 0x00b0, 2},
        {0x00b1, 0x00bb, 5},
        {0x00bf, 0x00d7, 24},
        {0x00f7, 0x2010, 7961},
        {0x2011, 0x2027, 1},
        {0x2030, 0x203e, 1},
        {0x2041, 0x2053, 1},
        {0x2055, 0x205e, 1},
        {0x2190, 0x245f, 1},
        {0x2500, 0x2775, 1},
        {0x2794, 0x2bff, 1},
        {0x2e00, 0x2e7f, 1},
        {0x3001, 0x3003, 1},
        {0x3008, 0x3020, 1},
        {0x3030, 0xfd3e, 52494},
        {0xfd3f, 0xfe45, 262},
        {0xfe46, 0xfe46, 1},
    },
    latin_offset: 10,
}

rt! {
    name: _PATTERN_WHITE_SPACE,
    r16: {
        {0x0009, 0x000d, 1},
        {0x0020, 0x0085, 101},
        {0x200e, 0x200f, 1},
        {0x2028, 0x2029, 1},
    },
    latin_offset: 2,
}

rt! {
    name: _PREPENDED_CONCATENATION_MARK,
    r16: {
        {0x0600, 0x0605, 1},
        {0x06dd, 0x070f, 50},
        {0x08e2, 0x08e2, 1},
    },
    r32: {
        {0x110bd, 0x110cd, 16},
    }
}

rt! {
    name: _QUOTATION_MARK,
    r16: {
        {0x0022, 0x0027, 5},
        {0x00ab, 0x00bb, 16},
        {0x2018, 0x201f, 1},
        {0x2039, 0x203a, 1},
        {0x2e42, 0x300c, 458},
        {0x300d, 0x300f, 1},
        {0x301d, 0x301f, 1},
        {0xfe41, 0xfe44, 1},
        {0xff02, 0xff07, 5},
        {0xff62, 0xff63, 1},
    },
    latin_offset: 2,
}

rt! {
    name: _RADICAL,
    r16: {
        {0x2e80, 0x2e99, 1},
        {0x2e9b, 0x2ef3, 1},
        {0x2f00, 0x2fd5, 1},
    }
}

rt! {
    name: _REGIONAL_INDICATOR,
    r32: {
        {0x1f1e6, 0x1f1ff, 1},
    }
}

rt! {
    name: _SENTENCE_TERMINAL,
    r16: {
        {0x0021, 0x002e, 13},
        {0x003f, 0x0589, 1354},
        {0x061e, 0x061f, 1},
        {0x06d4, 0x0700, 44},
        {0x0701, 0x0702, 1},
        {0x07f9, 0x0837, 62},
        {0x0839, 0x083d, 4},
        {0x083e, 0x0964, 294},
        {0x0965, 0x104a, 1765},
        {0x104b, 0x1362, 791},
        {0x1367, 0x1368, 1},
        {0x166e, 0x1735, 199},
        {0x1736, 0x1803, 205},
        {0x1809, 0x1944, 315},
        {0x1945, 0x1aa8, 355},
        {0x1aa9, 0x1aab, 1},
        {0x1b5a, 0x1b5b, 1},
        {0x1b5e, 0x1b5f, 1},
        {0x1c3b, 0x1c3c, 1},
        {0x1c7e, 0x1c7f, 1},
        {0x203c, 0x203d, 1},
        {0x2047, 0x2049, 1},
        {0x2e2e, 0x2e3c, 14},
        {0x3002, 0xa4ff, 29949},
        {0xa60e, 0xa60f, 1},
        {0xa6f3, 0xa6f7, 4},
        {0xa876, 0xa877, 1},
        {0xa8ce, 0xa8cf, 1},
        {0xa92f, 0xa9c8, 153},
        {0xa9c9, 0xaa5d, 148},
        {0xaa5e, 0xaa5f, 1},
        {0xaaf0, 0xaaf1, 1},
        {0xabeb, 0xfe52, 21095},
        {0xfe56, 0xfe57, 1},
        {0xff01, 0xff0e, 13},
        {0xff1f, 0xff61, 66},
    },
    r32: {
        {0x10a56, 0x10a57, 1},
        {0x10f55, 0x10f59, 1},
        {0x11047, 0x11048, 1},
        {0x110be, 0x110c1, 1},
        {0x11141, 0x11143, 1},
        {0x111c5, 0x111c6, 1},
        {0x111cd, 0x111de, 17},
        {0x111df, 0x11238, 89},
        {0x11239, 0x1123b, 2},
        {0x1123c, 0x112a9, 109},
        {0x1144b, 0x1144c, 1},
        {0x115c2, 0x115c3, 1},
        {0x115c9, 0x115d7, 1},
        {0x11641, 0x11642, 1},
        {0x1173c, 0x1173e, 1},
        {0x11944, 0x11946, 2},
        {0x11a42, 0x11a43, 1},
        {0x11a9b, 0x11a9c, 1},
        {0x11c41, 0x11c42, 1},
        {0x11ef7, 0x11ef8, 1},
        {0x16a6e, 0x16a6f, 1},
        {0x16af5, 0x16b37, 66},
        {0x16b38, 0x16b44, 12},
        {0x16e98, 0x1bc9f, 19975},
        {0x1da88, 0x1da88, 1},
    },
    latin_offset: 1,
}

rt! {
    name: _SOFT_DOTTED,
    r16: {
        {0x0069, 0x006a, 1},
        {0x012f, 0x0249, 282},
        {0x0268, 0x029d, 53},
        {0x02b2, 0x03f3, 321},
        {0x0456, 0x0458, 2},
        {0x1d62, 0x1d96, 52},
        {0x1da4, 0x1da8, 4},
        {0x1e2d, 0x1ecb, 158},
        {0x2071, 0x2148, 215},
        {0x2149, 0x2c7c, 2867},
    },
    r32: {
        {0x1d422, 0x1d423, 1},
        {0x1d456, 0x1d457, 1},
        {0x1d48a, 0x1d48b, 1},
        {0x1d4be, 0x1d4bf, 1},
        {0x1d4f2, 0x1d4f3, 1},
        {0x1d526, 0x1d527, 1},
        {0x1d55a, 0x1d55b, 1},
        {0x1d58e, 0x1d58f, 1},
        {0x1d5c2, 0x1d5c3, 1},
        {0x1d5f6, 0x1d5f7, 1},
        {0x1d62a, 0x1d62b, 1},
        {0x1d65e, 0x1d65f, 1},
        {0x1d692, 0x1d693, 1},
    },
    latin_offset: 1
}

rt! {
    name: _TERMINAL_PUNCTUATION,
    r16: {
        {0x0021, 0x002c, 11},
        {0x002e, 0x003a, 12},
        {0x003b, 0x003f, 4},
        {0x037e, 0x0387, 9},
        {0x0589, 0x05c3, 58},
        {0x060c, 0x061b, 15},
        {0x061e, 0x061f, 1},
        {0x06d4, 0x0700, 44},
        {0x0701, 0x070a, 1},
        {0x070c, 0x07f8, 236},
        {0x07f9, 0x0830, 55},
        {0x0831, 0x083e, 1},
        {0x085e, 0x0964, 262},
        {0x0965, 0x0e5a, 1269},
        {0x0e5b, 0x0f08, 173},
        {0x0f0d, 0x0f12, 1},
        {0x104a, 0x104b, 1},
        {0x1361, 0x1368, 1},
        {0x166e, 0x16eb, 125},
        {0x16ec, 0x16ed, 1},
        {0x1735, 0x1736, 1},
        {0x17d4, 0x17d6, 1},
        {0x17da, 0x1802, 40},
        {0x1803, 0x1805, 1},
        {0x1808, 0x1809, 1},
        {0x1944, 0x1945, 1},
        {0x1aa8, 0x1aab, 1},
        {0x1b5a, 0x1b5b, 1},
        {0x1b5d, 0x1b5f, 1},
        {0x1c3b, 0x1c3f, 1},
        {0x1c7e, 0x1c7f, 1},
        {0x203c, 0x203d, 1},
        {0x2047, 0x2049, 1},
        {0x2e2e, 0x2e3c, 14},
        {0x2e41, 0x2e4c, 11},
        {0x2e4e, 0x2e4f, 1},
        {0x3001, 0x3002, 1},
        {0xa4fe, 0xa4ff, 1},
        {0xa60d, 0xa60f, 1},
        {0xa6f3, 0xa6f7, 1},
        {0xa876, 0xa877, 1},
        {0xa8ce, 0xa8cf, 1},
        {0xa92f, 0xa9c7, 152},
        {0xa9c8, 0xa9c9, 1},
        {0xaa5d, 0xaa5f, 1},
        {0xaadf, 0xaaf0, 17},
        {0xaaf1, 0xabeb, 250},
        {0xfe50, 0xfe52, 1},
        {0xfe54, 0xfe57, 1},
        {0xff01, 0xff0c, 11},
        {0xff0e, 0xff1a, 12},
        {0xff1b, 0xff1f, 4},
        {0xff61, 0xff64, 3},
    },
    r32: {
        {0x1039f, 0x103d0, 49},
        {0x10857, 0x1091f, 200},
        {0x10a56, 0x10a57, 1},
        {0x10af0, 0x10af5, 1},
        {0x10b3a, 0x10b3f, 1},
        {0x10b99, 0x10b9c, 1},
        {0x10f55, 0x10f59, 1},
        {0x11047, 0x1104d, 1},
        {0x110be, 0x110c1, 1},
        {0x11141, 0x11143, 1},
        {0x111c5, 0x111c6, 1},
        {0x111cd, 0x111de, 17},
        {0x111df, 0x11238, 89},
        {0x11239, 0x1123c, 1},
        {0x112a9, 0x1144b, 418},
        {0x1144c, 0x1144d, 1},
        {0x1145a, 0x1145b, 1},
        {0x115c2, 0x115c5, 1},
        {0x115c9, 0x115d7, 1},
        {0x11641, 0x11642, 1},
        {0x1173c, 0x1173e, 1},
        {0x11944, 0x11946, 2},
        {0x11a42, 0x11a43, 1},
        {0x11a9b, 0x11a9c, 1},
        {0x11aa1, 0x11aa2, 1},
        {0x11c41, 0x11c43, 1},
        {0x11c71, 0x11ef7, 646},
        {0x11ef8, 0x12470, 1400},
        {0x12471, 0x12474, 1},
        {0x16a6e, 0x16a6f, 1},
        {0x16af5, 0x16b37, 66},
        {0x16b38, 0x16b39, 1},
        {0x16b44, 0x16e97, 851},
        {0x16e98, 0x1bc9f, 19975},
        {0x1da87, 0x1da8a, 1},
    },
    latin_offset: 3,
}

rt! {
    name: _UNIFIED_IDEOGRAPH,
    r16: {
        {0x3400, 0x4dbf, 1},
        {0x4e00, 0x9ffc, 1},
        {0xfa0e, 0xfa0f, 1},
        {0xfa11, 0xfa13, 2},
        {0xfa14, 0xfa1f, 11},
        {0xfa21, 0xfa23, 2},
        {0xfa24, 0xfa27, 3},
        {0xfa28, 0xfa29, 1},
    },
    r32: {
        {0x20000, 0x2a6dd, 1},
        {0x2a700, 0x2b734, 1},
        {0x2b740, 0x2b81d, 1},
        {0x2b820, 0x2cea1, 1},
        {0x2ceb0, 0x2ebe0, 1},
        {0x30000, 0x3134a, 1},
    }
}

rt! {
    name: _VARIATION_SELECTOR,
    r16: {
        {0x180b, 0x180d, 1},
        {0xfe00, 0xfe0f, 1},
    },
    r32: {
        {0xe0100, 0xe01ef, 1},
    }
}

rt! {
    name: _WHITE_SPACE,
    r16: {
        {0x0009, 0x000d, 1},
        {0x0020, 0x0085, 101},
        {0x00a0, 0x1680, 5600},
        {0x2000, 0x200a, 1},
        {0x2028, 0x2029, 1},
        {0x202f, 0x205f, 48},
        {0x3000, 0x3000, 1},
    },
    latin_offset: 2,
}

rt! {
    name: _FOLD_L,
    r16: {
        {0x0345, 0x0345, 1},
    }
}

rt! {
    name: _FOLD_LL,
    r16: {
        {0x0041, 0x005a, 1},
        {0x00c0, 0x00d6, 1},
        {0x00d8, 0x00de, 1},
        {0x0100, 0x012e, 2},
        {0x0132, 0x0136, 2},
        {0x0139, 0x0147, 2},
        {0x014a, 0x0178, 2},
        {0x0179, 0x017d, 2},
        {0x0181, 0x0182, 1},
        {0x0184, 0x0186, 2},
        {0x0187, 0x0189, 2},
        {0x018a, 0x018b, 1},
        {0x018e, 0x0191, 1},
        {0x0193, 0x0194, 1},
        {0x0196, 0x0198, 1},
        {0x019c, 0x019d, 1},
        {0x019f, 0x01a0, 1},
        {0x01a2, 0x01a6, 2},
        {0x01a7, 0x01a9, 2},
        {0x01ac, 0x01ae, 2},
        {0x01af, 0x01b1, 2},
        {0x01b2, 0x01b3, 1},
        {0x01b5, 0x01b7, 2},
        {0x01b8, 0x01bc, 4},
        {0x01c4, 0x01c5, 1},
        {0x01c7, 0x01c8, 1},
        {0x01ca, 0x01cb, 1},
        {0x01cd, 0x01db, 2},
        {0x01de, 0x01ee, 2},
        {0x01f1, 0x01f2, 1},
        {0x01f4, 0x01f6, 2},
        {0x01f7, 0x01f8, 1},
        {0x01fa, 0x0232, 2},
        {0x023a, 0x023b, 1},
        {0x023d, 0x023e, 1},
        {0x0241, 0x0243, 2},
        {0x0244, 0x0246, 1},
        {0x0248, 0x024e, 2},
        {0x0345, 0x0370, 43},
        {0x0372, 0x0376, 4},
        {0x037f, 0x0386, 7},
        {0x0388, 0x038a, 1},
        {0x038c, 0x038e, 2},
        {0x038f, 0x0391, 2},
        {0x0392, 0x03a1, 1},
        {0x03a3, 0x03ab, 1},
        {0x03cf, 0x03d8, 9},
        {0x03da, 0x03ee, 2},
        {0x03f4, 0x03f7, 3},
        {0x03f9, 0x03fa, 1},
        {0x03fd, 0x042f, 1},
        {0x0460, 0x0480, 2},
        {0x048a, 0x04c0, 2},
        {0x04c1, 0x04cd, 2},
        {0x04d0, 0x052e, 2},
        {0x0531, 0x0556, 1},
        {0x10a0, 0x10c5, 1},
        {0x10c7, 0x10cd, 6},
        {0x13a0, 0x13f5, 1},
        {0x1c90, 0x1cba, 1},
        {0x1cbd, 0x1cbf, 1},
        {0x1e00, 0x1e94, 2},
        {0x1e9e, 0x1efe, 2},
        {0x1f08, 0x1f0f, 1},
        {0x1f18, 0x1f1d, 1},
        {0x1f28, 0x1f2f, 1},
        {0x1f38, 0x1f3f, 1},
        {0x1f48, 0x1f4d, 1},
        {0x1f59, 0x1f5f, 2},
        {0x1f68, 0x1f6f, 1},
        {0x1f88, 0x1f8f, 1},
        {0x1f98, 0x1f9f, 1},
        {0x1fa8, 0x1faf, 1},
        {0x1fb8, 0x1fbc, 1},
        {0x1fc8, 0x1fcc, 1},
        {0x1fd8, 0x1fdb, 1},
        {0x1fe8, 0x1fec, 1},
        {0x1ff8, 0x1ffc, 1},
        {0x2126, 0x212a, 4},
        {0x212b, 0x2132, 7},
        {0x2183, 0x2c00, 2685},
        {0x2c01, 0x2c2e, 1},
        {0x2c60, 0x2c62, 2},
        {0x2c63, 0x2c64, 1},
        {0x2c67, 0x2c6d, 2},
        {0x2c6e, 0x2c70, 1},
        {0x2c72, 0x2c75, 3},
        {0x2c7e, 0x2c80, 1},
        {0x2c82, 0x2ce2, 2},
        {0x2ceb, 0x2ced, 2},
        {0x2cf2, 0xa640, 31054},
        {0xa642, 0xa66c, 2},
        {0xa680, 0xa69a, 2},
        {0xa722, 0xa72e, 2},
        {0xa732, 0xa76e, 2},
        {0xa779, 0xa77d, 2},
        {0xa77e, 0xa786, 2},
        {0xa78b, 0xa78d, 2},
        {0xa790, 0xa792, 2},
        {0xa796, 0xa7aa, 2},
        {0xa7ab, 0xa7ae, 1},
        {0xa7b0, 0xa7b4, 1},
        {0xa7b6, 0xa7be, 2},
        {0xa7c2, 0xa7c4, 2},
        {0xa7c5, 0xa7c7, 1},
        {0xa7c9, 0xa7f5, 44},
        {0xff21, 0xff3a, 1},
    },
    r32: {
        {0x10400, 0x10427, 1},
        {0x104b0, 0x104d3, 1},
        {0x10c80, 0x10cb2, 1},
        {0x118a0, 0x118bf, 1},
        {0x16e40, 0x16e5f, 1},
        {0x1e900, 0x1e921, 1},
    },
    latin_offset: 3,
}

rt! {
    name: _FOLD_LT,
    r16: {
        {0x01c4, 0x01c6, 2},
        {0x01c7, 0x01c9, 2},
        {0x01ca, 0x01cc, 2},
        {0x01f1, 0x01f3, 2},
        {0x1f80, 0x1f87, 1},
        {0x1f90, 0x1f97, 1},
        {0x1fa0, 0x1fa7, 1},
        {0x1fb3, 0x1fc3, 16},
        {0x1ff3, 0x1ff3, 1},
    }
}

rt! {
    name: _FOLD_LU,
    r16: {
        {0x0061, 0x007a, 1},
        {0x00b5, 0x00df, 42},
        {0x00e0, 0x00f6, 1},
        {0x00f8, 0x00ff, 1},
        {0x0101, 0x012f, 2},
        {0x0133, 0x0137, 2},
        {0x013a, 0x0148, 2},
        {0x014b, 0x0177, 2},
        {0x017a, 0x017e, 2},
        {0x017f, 0x0180, 1},
        {0x0183, 0x0185, 2},
        {0x0188, 0x018c, 4},
        {0x0192, 0x0195, 3},
        {0x0199, 0x019a, 1},
        {0x019e, 0x01a1, 3},
        {0x01a3, 0x01a5, 2},
        {0x01a8, 0x01ad, 5},
        {0x01b0, 0x01b4, 4},
        {0x01b6, 0x01b9, 3},
        {0x01bd, 0x01bf, 2},
        {0x01c5, 0x01c6, 1},
        {0x01c8, 0x01c9, 1},
        {0x01cb, 0x01cc, 1},
        {0x01ce, 0x01dc, 2},
        {0x01dd, 0x01ef, 2},
        {0x01f2, 0x01f3, 1},
        {0x01f5, 0x01f9, 4},
        {0x01fb, 0x021f, 2},
        {0x0223, 0x0233, 2},
        {0x023c, 0x023f, 3},
        {0x0240, 0x0242, 2},
        {0x0247, 0x024f, 2},
        {0x0250, 0x0254, 1},
        {0x0256, 0x0257, 1},
        {0x0259, 0x025b, 2},
        {0x025c, 0x0260, 4},
        {0x0261, 0x0265, 2},
        {0x0266, 0x0268, 2},
        {0x0269, 0x026c, 1},
        {0x026f, 0x0271, 2},
        {0x0272, 0x0275, 3},
        {0x027d, 0x0280, 3},
        {0x0282, 0x0283, 1},
        {0x0287, 0x028c, 1},
        {0x0292, 0x029d, 11},
        {0x029e, 0x0345, 167},
        {0x0371, 0x0373, 2},
        {0x0377, 0x037b, 4},
        {0x037c, 0x037d, 1},
        {0x03ac, 0x03af, 1},
        {0x03b1, 0x03ce, 1},
        {0x03d0, 0x03d1, 1},
        {0x03d5, 0x03d7, 1},
        {0x03d9, 0x03ef, 2},
        {0x03f0, 0x03f3, 1},
        {0x03f5, 0x03fb, 3},
        {0x0430, 0x045f, 1},
        {0x0461, 0x0481, 2},
        {0x048b, 0x04bf, 2},
        {0x04c2, 0x04ce, 2},
        {0x04cf, 0x052f, 2},
        {0x0561, 0x0586, 1},
        {0x10d0, 0x10fa, 1},
        {0x10fd, 0x10ff, 1},
        {0x13f8, 0x13fd, 1},
        {0x1c80, 0x1c88, 1},
        {0x1d79, 0x1d7d, 4},
        {0x1d8e, 0x1e01, 115},
        {0x1e03, 0x1e95, 2},
        {0x1e9b, 0x1ea1, 6},
        {0x1ea3, 0x1eff, 2},
        {0x1f00, 0x1f07, 1},
        {0x1f10, 0x1f15, 1},
        {0x1f20, 0x1f27, 1},
        {0x1f30, 0x1f37, 1},
        {0x1f40, 0x1f45, 1},
        {0x1f51, 0x1f57, 2},
        {0x1f60, 0x1f67, 1},
        {0x1f70, 0x1f7d, 1},
        {0x1fb0, 0x1fb1, 1},
        {0x1fbe, 0x1fd0, 18},
        {0x1fd1, 0x1fe0, 15},
        {0x1fe1, 0x1fe5, 4},
        {0x214e, 0x2184, 54},
        {0x2c30, 0x2c5e, 1},
        {0x2c61, 0x2c65, 4},
        {0x2c66, 0x2c6c, 2},
        {0x2c73, 0x2c76, 3},
        {0x2c81, 0x2ce3, 2},
        {0x2cec, 0x2cee, 2},
        {0x2cf3, 0x2d00, 13},
        {0x2d01, 0x2d25, 1},
        {0x2d27, 0x2d2d, 6},
        {0xa641, 0xa66d, 2},
        {0xa681, 0xa69b, 2},
        {0xa723, 0xa72f, 2},
        {0xa733, 0xa76f, 2},
        {0xa77a, 0xa77c, 2},
        {0xa77f, 0xa787, 2},
        {0xa78c, 0xa791, 5},
        {0xa793, 0xa794, 1},
        {0xa797, 0xa7a9, 2},
        {0xa7b5, 0xa7bf, 2},
        {0xa7c3, 0xa7c8, 5},
        {0xa7ca, 0xa7f6, 44},
        {0xab53, 0xab70, 29},
        {0xab71, 0xabbf, 1},
        {0xff41, 0xff5a, 1},
    },
    r32: {
        {0x10428, 0x1044f, 1},
        {0x104d8, 0x104fb, 1},
        {0x10cc0, 0x10cf2, 1},
        {0x118c0, 0x118df, 1},
        {0x16e60, 0x16e7f, 1},
        {0x1e922, 0x1e943, 1},
    },
    latin_offset: 4,
}

rt! {
    name: _FOLD_M,
    r16: {
        {0x0399, 0x03b9, 32},
        {0x1fbe, 0x1fbe, 1},
    }
}

rt! {
    name: _FOLD_MN,
    r16: {
        {0x0399, 0x03b9, 32},
        {0x1fbe, 0x1fbe, 1},
    }
}

/// FoldCategory maps a category name to a table of
/// code points outside the category that are equivalent under
/// simple case folding to code points inside the category.
/// If there is no entry for a category name, there are no such points.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoldCategory;

impl core::ops::Index<&str> for FoldCategory {
    type Output = RangeTable;

    fn index(&self, key: &str) -> &Self::Output {
        match key {
            "L" | "l" => _FOLD_L,
            "Ll" | "ll" | "LL" => _FOLD_LL,
            "Lt" | "lt" | "LT" => _FOLD_LT,
            "Lu" | "lu" | "LU" => _FOLD_LU,
            "M" | "m" => _FOLD_M,
            "Mn" | "mn" | "MN" => _FOLD_MN,
            _ => panic!("unkown category name"),
        }
    }
}

impl FoldCategory {
    /// Returns the number of the category
    #[inline]
    pub const fn len(&self) -> usize {
        6
    }

    /// Returns if the category is empty
    #[inline]
    pub const fn is_empty(&self) -> bool {
        false
    }

    /// Returns if the category contains the category name
    #[inline]
    pub fn contains(&self, key: &str) -> bool {
        matches!(
            key,
            "L" | "l"
                | "Ll"
                | "ll"
                | "LL"
                | "Lt"
                | "lt"
                | "LT"
                | "Lu"
                | "lu"
                | "LU"
                | "M"
                | "m"
                | "Mn"
                | "mn"
                | "MN"
        )
    }

    /// Create a new iter
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, (&'static str, &'static RangeTable)> {
        static FOLD_CATEGORIES: [(&str, &RangeTable); 6] = [
            ("L", _FOLD_L),
            ("Ll", _FOLD_LL),
            ("Lt", _FOLD_LT),
            ("Lu", _FOLD_LU),
            ("M", _FOLD_M),
            ("Mn", _FOLD_MN),
        ];
        FOLD_CATEGORIES.iter()
    }
}

rt! {
    name: _FOLD_COMMON,
    r16: {
        {0x039c, 0x03bc, 32},
    }
}

rt! {
    name: _FOLD_GREEK,
    r16: {
        {0x00b5, 0x0345, 656},
    }
}

rt! {
    name: _FOLD_INHERITED,
    r16: {
        {0x0399, 0x03b9, 32},
        {0x1fbe, 0x1fbe, 1},
    }
}

/// FoldScript maps a script name to a table of
/// code points outside the script that are equivalent under
/// simple case folding to code points inside the script.
/// If there is no entry for a script name, there are no such points.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoldScript;

impl core::ops::Index<&str> for FoldScript {
    type Output = RangeTable;

    fn index(&self, key: &str) -> &Self::Output {
        match key {
            "Common" | "common" | "COMMON" => _FOLD_COMMON,
            "Greek" | "greek" | "GREEK" => _FOLD_GREEK,
            "Inherited" | "inherited" | "INHERITED" => _FOLD_INHERITED,
            _ => panic!("unkown script name"),
        }
    }
}

impl FoldScript {
    /// Returns the number of the script
    #[inline]
    pub const fn len(&self) -> usize {
        3
    }

    /// Returns if the script is empty
    #[inline]
    pub const fn is_empty(&self) -> bool {
        false
    }

    /// Returns if the script contains the script name
    #[inline]
    pub fn contains(&self, key: &str) -> bool {
        matches!(
            key,
            "Common"
                | "common"
                | "COMMON"
                | "Greek"
                | "greek"
                | "GREEK"
                | "Inherited"
                | "inherited"
                | "INHERITED"
        )
    }

    /// Create a new iter
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, (&'static str, &'static RangeTable)> {
        static FOLD_SCRIPTS: [(&str, &RangeTable); 3] = [
            ("Common", _FOLD_COMMON),
            ("Greek", _FOLD_GREEK),
            ("Inherited", _FOLD_INHERITED),
        ];
        FOLD_SCRIPTS.iter()
    }
}
