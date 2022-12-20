/// Go unicode/utf8 library
pub mod utf8;

/// Go unicode/utf16 library
pub mod utf16;

mod tables;
pub use tables::*;

mod graphic;
pub use graphic::*;

mod case_tables;
pub use case_tables::*;

mod letter;
pub use letter::*;

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

/// Reports whether the char is a decimal digit.
#[inline]
pub const fn is_digit(ch: char) -> bool {
    let c = ch as u32;
    if c <= MAX_LATIN1 as u32 {
        ('0' as u32) <= c && c <= '9' as u32
    } else {
        is_excluding_latin(RangeTable::DIGIT, ch)
    }
}
