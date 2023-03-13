/// Provide const encode functions for uvarint
pub mod encode;

/// Provide decode functions for uvarint
pub mod decode;

const U8_LEN: usize = 2;
const U16_LEN: usize = 3;
const U32_LEN: usize = 5;
const U64_LEN: usize = 10;
const U128_LEN: usize = 19;

#[cfg(target_pointer_width = "64")]
const USIZE_LEN: usize = U64_LEN;

#[cfg(target_pointer_width = "32")]
const USIZE_LEN: usize = U32_LEN;

/// The possible size of u8 varint
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum U8Size {
    /// 1
    One,
    /// 2
    Two,
}

impl U8Size {
    /// Returns the val in usize
    #[inline]
    pub const fn val(self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
        }
    }

    /// Returns the max size of u8 varint
    #[inline]
    pub const fn max() -> Self {
        Self::Two
    }

    /// Returns the min size of u8 varint
    #[inline]
    pub const fn min() -> Self {
        Self::One
    }

    #[inline]
    const fn from(val: usize) -> Self {
        match val {
            1 => Self::One,
            2 => Self::Two,
            _ => unreachable!(),
        }
    }
}

/// The possible size of u16 varint
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum U16Size {
    /// 1
    One,
    /// 2
    Two,
    /// 3
    Three,
}

impl U16Size {
    /// Returns the val in usize
    #[inline]
    pub const fn val(self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
        }
    }

    /// Returns the max size of u16 varint
    #[inline]
    pub const fn max() -> Self {
        Self::Three
    }

    /// Returns the min size of u16 varint
    #[inline]
    pub const fn min() -> Self {
        Self::One
    }

    #[inline]
    const fn from(val: usize) -> Self {
        match val {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            _ => unreachable!(),
        }
    }
}

/// The possible size of u32 varint
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum U32Size {
    /// 1
    One,
    /// 2
    Two,
    /// 3
    Three,
    /// 4
    Four,
    /// 5
    Five,
}

impl U32Size {
    /// Returns the val in usize
    #[inline]
    pub const fn val(self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
        }
    }

    /// Returns the max size of u32 varint
    #[inline]
    pub const fn max() -> Self {
        Self::Five
    }

    /// Returns the min size of u32 varint
    #[inline]
    pub const fn min() -> Self {
        Self::One
    }

    #[inline]
    const fn from(val: usize) -> Self {
        match val {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            _ => unreachable!(),
        }
    }
}

/// The possible size of u64 varint
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum U64Size {
    /// 1
    One,
    /// 2
    Two,
    /// 3
    Three,
    /// 4
    Four,
    /// 5
    Five,
    /// 6
    Six,
    /// 7
    Seven,
    /// 8
    Eight,
    /// 9
    Nine,
    /// 10
    Ten,
}

impl U64Size {
    /// Returns the val in usize
    #[inline]
    pub const fn val(&self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten => 10,
        }
    }

    /// Returns the max size of u64 varint
    #[inline]
    pub const fn max() -> Self {
        Self::Ten
    }

    /// Returns the min size of u64 varint
    #[inline]
    pub const fn min() -> Self {
        Self::One
    }

    #[inline]
    const fn from(val: usize) -> Self {
        match val {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            _ => unreachable!(),
        }
    }
}

/// The possible size of u128 varint
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum U128Size {
    /// 1
    One,
    /// 2
    Two,
    /// 3
    Three,
    /// 4
    Four,
    /// 5
    Five,
    /// 6
    Six,
    /// 7
    Seven,
    /// 8
    Eight,
    /// 9
    Nine,
    /// 10
    Ten,
    /// 11
    Eleven,
    /// 12
    Twelve,
    /// 13
    Thirteen,
    /// 14
    Fourteen,
    /// 15
    Fifteen,
    /// 16
    Sixteen,
    /// 17
    Seventeen,
    /// 18
    Eighteen,
    /// 19
    Nineteen,
}

impl U128Size {
    /// Returns the val in usize
    #[inline]
    pub const fn val(&self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten => 10,
            Self::Eleven => 11,
            Self::Twelve => 12,
            Self::Thirteen => 13,
            Self::Fourteen => 14,
            Self::Fifteen => 15,
            Self::Sixteen => 16,
            Self::Seventeen => 17,
            Self::Eighteen => 18,
            Self::Nineteen => 19,
        }
    }

    /// Returns the max size of u128 varint
    #[inline]
    pub const fn max() -> Self {
        Self::Nineteen
    }

    /// Returns the min size of u128 varint
    #[inline]
    pub const fn min() -> Self {
        Self::One
    }

    #[inline]
    const fn from(val: usize) -> Self {
        match val {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            11 => Self::Eleven,
            12 => Self::Twelve,
            13 => Self::Thirteen,
            14 => Self::Fourteen,
            15 => Self::Fifteen,
            16 => Self::Sixteen,
            17 => Self::Seventeen,
            18 => Self::Eighteen,
            19 => Self::Nineteen,
            _ => unreachable!(),
        }
    }
}

/// The possible size of usize varint
#[cfg(target_pointer_width = "64")]
pub type Usize = U64Size;

/// The possible size of usize varint
#[cfg(target_pointer_width = "32")]
pub type Usize = U32Size;

/// The maximum length of a varint-encoded N-bit integer.
const MAX_VARINT_LEN64: usize = 10;

/// Appends the uvarint-encoded form of `x` to the given [`Vec`],
/// as generated by PutUvarint, to buf and returns the extended buffer.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[inline]
pub fn append_uvarint(mut buf: alloc::vec::Vec<u8>, x: impl Unsigned) -> alloc::vec::Vec<u8> {
    let mut x = x.to_u64();
    while x >= 0x80 {
        buf.push((x as u8) | 0x80);
        x >>= 7;
    }
    buf.push(x as u8);
    buf
}

/// Writes the varint-encoded form of `x` to the writer.
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[inline]
pub fn write_uvarint<W: std::io::Write>(
    mut buf: W,
    x: impl Unsigned,
) -> Result<usize, std::io::Error> {
    let mut x = x.to_u64();
    let mut n = 0;
    while x >= 0x80 {
        n += buf.write(&[(x as u8) | 0x80])?;
        x >>= 7;
    }
    buf.write(&[x as u8]).map(|nn| {
        n += nn;
        n
    })
}

/// Reads an encoded unsigned integer from r and returns the value and number of bytes readed.
/// The error is EOF only if no bytes were read.
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[inline]
pub fn read_uvarint<R: std::io::Read, I: Unsigned>(mut r: R) -> std::io::Result<(I, usize)> {
    let mut x = 0;
    let mut s = 0;
    for i in 0..MAX_VARINT_LEN64 {
        let mut b = [0; 1];
        r.read_exact(&mut b)?;

        if b[0] < 0x80 {
            if i == MAX_VARINT_LEN64 - 1 && b[0] > 1 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    Error::Overflow,
                ));
            }
            return Ok((I::from_u64(x | ((b[0] as u64) << s)), i));
        }
        x |= ((b[0] & 0x7f) as u64) << s;
        s += 7;
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        Error::Overflow,
    ))
}

/// Encodes a uint64 into buf and returns the number of bytes written.
///
/// # Panic
/// The buffer is too small.
#[inline]
pub fn put_uvarint(buf: &mut [u8], x: impl Unsigned) -> usize {
    let mut i = 0;
    let mut x = x.to_u64();
    while x >= 0x80 {
        buf[i] = (x as u8) | 0x80;
        x >>= 7;
        i += 1;
    }
    buf[i] = x as u8;
    i + 1
}

/// Error
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    /// Buffer is too small
    SmallBuffer,
    /// Overflow 64-bit
    Overflow,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SmallBuffer => write!(f, "binary: the buffer is too small"),
            Self::Overflow => write!(f, "binary: varint overflows a 64-bit integer"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Decodes a unsigned integer from buf and returns that value and the
/// number of bytes read (> 0).
#[inline]
pub fn uvarint<R: Unsigned>(buf: &[u8]) -> Result<(R, usize), Error> {
    let (mut x, mut s) = (0, 0);
    for (i, b) in buf.iter().enumerate() {
        if i == MAX_VARINT_LEN64 {
            return Err(Error::Overflow);
        }

        let b = *b;
        if b < 0x80 {
            if i == MAX_VARINT_LEN64 - 1 && b > 1 {
                return Err(Error::Overflow);
            }
            return Ok((R::from_u64(x | (b as u64) << s), i + 1));
        }
        x |= ((b & 0x7f) as u64) << s;
        s += 7;
    }
    Err(Error::SmallBuffer)
}

/// Appends the varint-encoded form of `x` to the given [`Vec`],
/// as generated by PutVarint, to buf and returns the extended buffer.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[inline]
pub fn append_varint(buf: alloc::vec::Vec<u8>, x: impl Signed) -> alloc::vec::Vec<u8> {
    let x = x.to_i64();
    let mut ux = (x as u64) << 1;
    if x < 0 {
        ux = !ux;
    }
    append_uvarint(buf, ux)
}

/// Writes the varint-encoded form of `x` to the writer.
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[inline]
pub fn write_varint<W: std::io::Write>(buf: W, x: impl Signed) -> std::io::Result<usize> {
    let x = x.to_i64();
    let mut ux = (x as u64) << 1;
    if x < 0 {
        ux = !ux;
    }
    write_uvarint(buf, ux)
}

/// Reads an encoded unsigned integer from r and returns the value and number of bytes readed.
/// The error is EOF only if no bytes were read.
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[inline]
pub fn read_varint<R: std::io::Read, I: Signed>(r: R) -> std::io::Result<(I, usize)> {
    let (ux, readed) = read_uvarint::<R, u64>(r)?;
    let mut x = (ux >> 1) as i64;
    if ux & 1 != 0 {
        x = !x;
    }
    Ok((I::from_i64(x), readed))
}

/// Encodes an integer into buf and returns the number of bytes written.
///
/// # Panic
/// The buffer is too small.
#[inline]
pub fn put_varint(buf: &mut [u8], x: impl Signed) -> usize {
    let x = x.to_i64();
    let mut ux = (x as u64) << 1;
    if x < 0 {
        ux = !ux;
    }
    put_uvarint(buf, ux)
}

/// Decodes an integer from buf and returns that value and the
/// number of bytes read (> 0).
#[inline]
pub fn varint<R: Signed>(buf: &[u8]) -> Result<(R, usize), Error> {
    let (ux, n) = uvarint::<u64>(buf)?;
    let mut x = (ux >> 1) as i64;
    if ux & 1 != 0 {
        x = !x;
    }
    Ok((R::from_i64(x), n))
}

macro_rules! impl_ {
    ($trait: ident::<$ret:ident>::$fn: ident::$from_fn: ident { $($x:ident),+ $(,)? }) => {
        $(
            impl $trait for $x {
                fn $fn(&self) -> $ret {
                    *self as $ret
                }

                fn $from_fn(val: $ret) -> Self {
                    val as Self
                }
            }
        )*
    };
}

/// A marker trait means this value can be trait as an unsigned integer.
pub trait Unsigned {
    /// Converts self to u64
    fn to_u64(&self) -> u64;

    ///
    fn from_u64(val: u64) -> Self;
}

impl_! {
    Unsigned::<u64>::to_u64::from_u64 {
        u8,
        u16,
        u32,
        usize,
        u64,
        u128,
    }
}

/// A marker trait means this value can be trait as a signed integer.
pub trait Signed {
    /// Converts self to i64
    fn to_i64(&self) -> i64;

    ///
    fn from_i64(val: i64) -> Self;
}

impl_! {
    Signed::<i64>::to_i64::from_i64 {
        i8,
        i16,
        i32,
        isize,
        i64,
        i128,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX_VARINT_LEN16: usize = 3;
    const MAX_VARINT_LEN32: usize = 5;

    fn test_constant(w: u64, max: usize) {
        let mut buf = vec![0; MAX_VARINT_LEN64];
        let n = put_uvarint(&mut buf, 1u64 << (w - 1));
        assert_eq!(n, max);
    }

    #[test]
    fn test_constants() {
        test_constant(16, MAX_VARINT_LEN16);
        test_constant(32, MAX_VARINT_LEN32);
        test_constant(64, MAX_VARINT_LEN64);
    }
}
