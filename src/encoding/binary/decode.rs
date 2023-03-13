use super::*;

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
