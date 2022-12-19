/// Go unicode/utf8 library
#[cfg(feature = "utf8")]
#[cfg_attr(docrs, doc(cfg(feature = "utf8")))]
pub mod utf8;

/// Go unicode/utf16 library
#[cfg(feature = "utf16")]
#[cfg_attr(docrs, doc(cfg(feature = "utf16")))]
pub mod utf16;

mod tables;
pub use tables::*;

mod graphic;
pub use graphic::*;

mod case_tables;
pub use case_tables::*;

/// Maximum Latin-1 value.
pub const MAX_LATIN1: char = '\u{00FF}';

/// Maximum ASCII value.
pub const MAX_ASCII: char = '\u{007F}';

/// [`RangeTable`] defines a set of Unicode code points by listing the ranges of
/// code points within the set. The ranges are listed in two slices
/// to save space: a slice of 16-bit ranges and a slice of 32-bit ranges.
/// The two slices must be in sorted order and non-overlapping.
/// Also, R32 should contain only values >= 0x10000 (1<<16).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RangeTable {
    r16: &'static [Range16],
    r32: &'static [Range32],
    latin_offset: usize,
}

impl RangeTable {
    /// Creates a new [`RangeTable`].
    pub const fn new(
        r16: &'static [Range16],
        r32: &'static [Range32],
        latin_offset: usize,
    ) -> Self {
        Self {
            r16,
            r32,
            latin_offset,
        }
    }

    /// Returns the 16-bit ranges of the table.
    pub const fn r16(&self) -> &'static [Range16] {
        self.r16
    }

    /// Returns the 32-bit ranges of the table.
    pub const fn r32(&self) -> &'static [Range32] {
        self.r32
    }

    /// Returns the number of entries in R16 with [`Range16::hi`] <= [`MAX_LATIN1`].
    ///
    /// [`Range16::hi`]: struct.Range16.html#method.hi
    pub const fn latin_offset(&self) -> usize {
        self.latin_offset
    }
}

/// [`Range16`] represents of a range of 16-bit Unicode code points. The range runs from lo to hi
/// inclusive and has the specified stride.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Range16 {
    lo: u16,
    hi: u16,
    stride: u16,
}

impl Range16 {
    /// Creates a new [`Range16`].
    pub const fn new(lo: u16, hi: u16, stride: u16) -> Self {
        Self { lo, hi, stride }
    }

    /// Returns the lower bound of the range.
    pub const fn lo(&self) -> u16 {
        self.lo
    }

    /// Returns the upper bound of the range.
    pub const fn hi(&self) -> u16 {
        self.hi
    }

    /// Returns the stride of the range.
    pub const fn stride(&self) -> u16 {
        self.stride
    }
}

/// [`Range32`] represents of a range of Unicode code points and is used when one or
/// more of the values will not fit in 16 bits. The range runs from Lo to Hi
/// inclusive and has the specified stride. Lo and Hi must always be >= 1<<16.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Range32 {
    lo: u32,
    hi: u32,
    stride: u32,
}

impl Range32 {
    /// Creates a new [`Range32`].
    pub const fn new(lo: u32, hi: u32, stride: u32) -> Self {
        Self { lo, hi, stride }
    }

    /// Returns the lower bound of the range.
    pub const fn lo(&self) -> u32 {
        self.lo
    }

    /// Returns the upper bound of the range.
    pub const fn hi(&self) -> u32 {
        self.hi
    }

    /// Returns the stride of the range.
    pub const fn stride(&self) -> u32 {
        self.stride
    }
}

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
    lo: u32,
    hi: u32,
    delta: D,
}

/// [`SpecialCase`] represents language-specific case mappings such as Turkish.
/// Methods of SpecialCase customize (by overriding) the standard mappings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpecialCase<const N: usize>([CaseRange; N]);

impl<const N: usize> SpecialCase<N> {
    /// Maps the [`char`] to upper case giving priority to the special mapping.
    #[inline]
    pub fn to_upper(&self, ch: char) -> char {
        todo!()
    }

    /// Maps the [`char`] to lower case giving priority to the special mapping.
    #[inline]
    pub fn to_title(&self, ch: char) -> char {
        todo!()
    }

    /// Maps the [`char`] to title case giving priority to the special mapping.
    pub fn to_lower(&self, ch: char) -> char {
        todo!()
    }
}

/// Right now all the entries fit in uint16, so use uint16. If that changes, compilation
/// will fail (the constants in the composite literal will not fit in `u16`)
/// and the types here can change to `u32`.
pub struct FoldPair {
    from: u16,
    to: u16,
}

/// Reports whether the char is a decimal digit.
pub const fn is_digit(ch: char) -> bool {
    let c = ch as u32;
    if c <= MAX_LATIN1 as u32 {
        ('0' as u32) <= c && c <= '9' as u32
    } else {
        is_excluding_latin(RangeTable::DIGIT, ch)
    }
}

/// Reports whether the char is a letter.
pub const fn is_excluding_latin(_rt: &RangeTable, _ch: char) -> bool {
    todo!()
}
