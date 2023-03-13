use super::*;

macro_rules! impl_uvarint {
    ($($name:ident::$size:expr => $rst: ident), +$(,)?) => {
        $(
            /// Encodes a unsigned integer into buf and returns the number of bytes written (> 0).
            #[inline]
            pub fn $name(val: $name, buf: &mut [u8; $size]) -> $rst {
                let mut n = val;
                let mut i = 0;
                for b in buf.iter_mut() {
                    *b = n as u8 | 0x80;
                    n >>= 7;
                    if n == 0 {
                        *b &= 0x7f;
                        break
                    }
                    i += 1
                }
                debug_assert_eq!(n, 0);
                $rst::from(i + 1)
            }
        )*
    }
}

impl_uvarint! {
    u8::U8_LEN => U8Size,
    u16::U16_LEN => U16Size,
    u32::U32_LEN => U32Size,
    u64::U64_LEN => U64Size,
    u128::U128_LEN => U128Size,
    usize::USIZE_LEN => Usize
}
