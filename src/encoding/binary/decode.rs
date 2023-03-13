use super::*;

macro_rules! impl_uvarint_with_buf {
    ($($fn_name:ident::$name:ident::$size:expr => $rst: ident), +$(,)?) => {
        $(
          /// Decodes a unsigned integer from buf and returns that value and a varint buf and the
          /// number of bytes read (> 0).
          #[inline]
          pub fn $fn_name(buf: &[u8]) -> Result<($name, [u8; $size], $rst), Error> {
              let mut dst = [0; $size];
              let (mut x, mut s) = (0, 0);
              for (i, b) in buf.iter().enumerate() {
                  if i == $size {
                      return Err(Error::Overflow);
                  }

                  let b = *b;
                  if b < 0x80 {
                      if i == $size - 1 && b > 1 {
                          return Err(Error::Overflow);
                      }
                      dst[i] = b;
                      return Ok((x | (b as $name) << s, dst, $rst::from(i + 1)));
                  }
                  x |= ((b & 0x7f) as $name) << s;
                  s += 7;
                  dst[i] = b;
              }
              Err(Error::SmallBuffer)
          }
        )*
    };
}

impl_uvarint_with_buf! {
    u8_with_buf::u8::U8_LEN => U8Size,
    u16_with_buf::u16::U16_LEN => U16Size,
    u32_with_buf::u32::U32_LEN => U32Size,
    u64_with_buf::u64::U64_LEN => U64Size,
    u128_with_buf::u128::U128_LEN => U128Size,
    usize_with_buf::usize::USIZE_LEN => Usize
}


macro_rules! impl_uvarint {
    ($($name:ident::$size:expr => $rst: ident), +$(,)?) => {
        $(
          /// Decodes a unsigned integer from buf and returns that value and the
          /// number of bytes read (> 0).
          #[inline]
          pub fn $name(buf: &[u8]) -> Result<($name, $rst), Error> {
              let (mut x, mut s) = (0, 0);
              for (i, b) in buf.iter().enumerate() {
                  if i == $size {
                      return Err(Error::Overflow);
                  }

                  let b = *b;
                  if b < 0x80 {
                      if i == $size - 1 && b > 1 {
                          return Err(Error::Overflow);
                      }
                      return Ok((x | (b as $name) << s, $rst::from(i + 1)));
                  }
                  x |= ((b & 0x7f) as $name) << s;
                  s += 7;
              }
              Err(Error::SmallBuffer)
          }
        )*
    };
}

impl_uvarint! {
    u8::U8_LEN => U8Size,
    u16::U16_LEN => U16Size,
    u32::U32_LEN => U32Size,
    u64::U64_LEN => U64Size,
    u128::U128_LEN => U128Size,
    usize::USIZE_LEN => Usize
}