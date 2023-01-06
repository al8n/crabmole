/// Bitcoin's Base58 encoding/decoding scheme, defined by bitcoin base58 alphabet.
pub const BITCOIN: Base58 =
    Base58::new_unchecked(*b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");

/// Ripple's Base58 encoding/decoding scheme, defined by ripple base58 alphabet.
pub const RIPPLE: Base58 =
    Base58::new_unchecked(*b"rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz");

/// Flickr's Base58 encoding/decoding scheme, defined by flickr base58 alphabet.
pub const FLICKR: Base58 =
    Base58::new_unchecked(*b"123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ");

/// Monero's Base58 encoding/decoding scheme, defined by monero base58 alphabet.
pub const MONERO: Base58 =
    Base58::new_unchecked(*b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");

/// IPFS's Base58 encoding/decoding scheme, defined by ipfs base58 alphabet.
pub const IPFS: Base58 =
    Base58::new_unchecked(*b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz");

const DECODE_MAP_INITIALIZE: [u8; 128] = [255; 128];

/// `BASE = 58`
pub const BASE: usize = 58;

/// Error
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Error {
    /// Duplicate character in alphabet
    DuplicateCharacter {
        /// the duplicated character
        character: u8,
        /// the first index the character present
        first: usize,
        /// the second index the character present
        second: usize,
    },
    /// Invalid character in alphabet (not an ASCII character)
    InvalidCharacter(u8),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DuplicateCharacter {
                character,
                first,
                second,
            } => write!(
                f,
                "encoding/base58: duplicate character `{}` at index {} and {}",
                *character as char, first, second
            ),
            Self::InvalidCharacter(c) => write!(
                f,
                "encoding/base58: invalid character `{}` is not an ASCII character",
                *c as char
            ),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Wrong input for decoding
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DecodeError {
    /// High-bit set on invalid digit
    InvalidHighBit,
    /// Invalid base58 digit
    InvalidCharacter(u8),
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DecodeError::InvalidHighBit => {
                write!(f, "encoding/base58: high-bit set on invalid digit")
            }
            DecodeError::InvalidCharacter(ch) => {
                writeln!(f, "encoding/base58: invalid base58 digit {}", ch)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {}

/// An Base58 is a radix 58 encoding/decoding scheme, defined by a
/// 58-character alphabet.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Base58 {
    encode: [u8; BASE],
    decode: [u8; 128],
}

impl Base58 {
    /// Returns a new Base58 defined by the given alphabet,
    /// which must be a 58-byte array and does not contain any non-ASCII character
    /// and no duplicate character.
    #[inline]
    pub const fn new(encoder: [u8; BASE]) -> Result<Self, Error> {
        let mut decode = DECODE_MAP_INITIALIZE;

        let mut idx = 0;
        while idx < encoder.len() {
            let c = encoder[idx];
            if c >= 128 {
                return Err(Error::InvalidCharacter(c));
            }

            if decode[c as usize] != 255 {
                return Err(Error::DuplicateCharacter {
                    character: c,
                    first: decode[c as usize] as usize,
                    second: idx,
                });
            }

            decode[c as usize] = idx as u8;
            idx += 1;
        }

        Ok(Self {
            encode: encoder,
            decode,
        })
    }

    /// Returns a new Base58 defined by the given alphabet,
    /// which must be a 58-byte array and does not contain any non-ASCII character.
    ///
    /// # Panic
    /// - alphabet contains invalid characters (non-ASCII character)
    /// - alphabet contains duplicate characters
    #[inline]
    pub const fn new_unchecked(encoder: [u8; 58]) -> Self {
        let mut decode = DECODE_MAP_INITIALIZE;

        let mut idx = 0;
        while idx < encoder.len() {
            let c = encoder[idx];
            if c >= 128 {
                panic!("encoding/base58: character is not an ASCII character");
            }

            if decode[c as usize] != 255 {
                panic!("encoding/base58: duplicate character is not allowed");
            }

            decode[c as usize] = idx as u8;
            idx += 1;
        }

        Self {
            encode: encoder,
            decode,
        }
    }

    /// Returns the max encoded length of the given src
    #[inline]
    pub const fn max_encoded_len(src: &[u8]) -> usize {
        let (n, _) = Self::max_encoded_len_in(src);
        n
    }

    #[inline]
    const fn max_encoded_len_in(src: &[u8]) -> (usize, usize) {
        let size = src.len();

        let mut zcount = 0;
        while zcount < size && src[zcount] == 0 {
            zcount += 1;
        }
        // It is crucial to make this as short as possible, especially for
        // the usual case of bitcoin addrs
        (
            zcount +
        // This is an integer simplification of
		// ceil(log(256)/log(58))
        (size-zcount)*555/406 + 1,
            zcount,
        )
    }

    // /// Encodes the given src into the given dst, returning the number of bytes written to dst.
    // #[inline]
    // pub fn encode(&self, src: &[u8], dst: &mut [u8]) -> usize {
    //     // if src.is_empty() {
    //     //     return 0;
    //     // }
    //     // let (size, zcount) = Self::max_encoded_len_in(src);
    //     // dst[..size].fill(0);
    //     // let n = self.encode_in(src, &mut dst[..size], size, zcount);
    //     // dst[n..size].fill(0);
    //     // n
    //     todo!()
    // }

    /// Encodes the given src into a new vec.
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn encode_to_vec(&self, src: &[u8]) -> alloc::vec::Vec<u8> {
        if src.is_empty() {
            return alloc::vec![];
        }
        let (size, zcount) = Self::max_encoded_len_in(src);
        let mut out = alloc::vec![0; size];
        let n = self.encode_to_vec_in(src, &mut out, size, zcount);
        out.truncate(n);
        out
    }

    #[inline]
    fn encode_to_vec_in(
        &self,
        src: &[u8],
        dst: &mut [u8],
        mut size: usize,
        zcount: usize,
    ) -> usize {
        let mut high = size - 1;
        for b in src {
            let mut i = size - 1;
            let mut carry = *b as u32;
            while i > high || carry != 0 {
                carry += 256 * (dst[i] as u32);
                dst[i] = (carry % 58) as u8;
                carry /= 58;
                if i == 0 {
                    break;
                }
                i -= 1;
            }
            high = i;
        }

        // Determine the additional "zero-gap" in the buffer (aside from zcount)
        let mut i = zcount;
        while i < size && dst[i] == 0 {
            i += 1;
        }

        // Now encode the values with actual alphabet in-place
        size = dst[i - zcount..].len();
        let mut j = 0;
        while j < size {
            dst[j] = self.encode[dst[i - zcount + j] as usize];
            j += 1;
        }
        size
    }

    /// Returns the max decoded len of the given src
    #[inline]
    pub const fn max_decoded_len(&self, src: &[u8]) -> usize {
        let (n, _) = self.max_decoded_len_in(src);
        n
    }

    #[inline]
    const fn max_decoded_len_in(&self, src: &[u8]) -> (usize, usize) {
        let zero = self.encode[0];
        let b58sz = src.len();

        let mut zcount = 0;
        let mut i = 0;
        while i < b58sz && src[i] == zero {
            zcount += 1;
            i += 1;
        }
        (2 * ((b58sz * 406 / 555) + 1), zcount)
    }

    /// Decodes the given src into the given dst, returning the number of bytes written to dst.
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn decode_to_vec(&self, src: &[u8]) -> Result<alloc::vec::Vec<u8>, DecodeError> {
        if src.is_empty() {
            return Ok(alloc::vec::Vec::new());
        }

        let (_, zcount) = self.max_decoded_len_in(src);

        let mut dst = alloc::vec![0; 2*((src.len() * 406/555)+1)];
        let mut outi = alloc::vec![0; (src.len() + 3) / 4];

        for r in src {
            if *r > 127 {
                return Err(DecodeError::InvalidHighBit);
            }

            if self.decode[*r as usize] == 255 {
                return Err(DecodeError::InvalidCharacter(*r));
            }

            let mut c = self.decode[*r as usize] as u64;
            for j in (0..=outi.len() - 1).rev() {
                let t = 58 * (outi[j] as u64) + c;
                c = t >> 32;
                outi[j] = (t & 0xffffffff) as u32;
            }
        }

        // initial mask depends on b58sz, on further loops it always starts at 24 bits
        let mut mask = (src.len() % 4) * 8;
        if mask == 0 {
            mask = 32;
        }
        mask -= 8;

        let mut out_len = 0;

        for out in &outi {
            while mask < 32 {
                dst[out_len] = (*out >> mask) as u8;
                mask = mask.wrapping_sub(8);
                out_len += 1;
            }
            mask = 24;
        }

        // find the most significant byte post-decode, if any
        let mut msb = zcount;
        while msb < dst.len() {
            if dst[msb] > 0 {
                return Ok(dst
                    .into_iter()
                    .skip(msb - zcount)
                    .take(out_len - (msb - zcount))
                    .collect());
            }
            msb += 1;
        }
        dst.truncate(out_len);
        Ok(dst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIGITS_OF_PI: [u8; 128] = [
        0x03, 0x24, 0x3F, 0x6A, 0x88, 0x85, 0xA3, 0x08, 0xD3, 0x13, 0x19, 0x8A, 0x2E, 0x03, 0x70,
        0x73, 0x44, 0xA4, 0x09, 0x38, 0x22, 0x29, 0x9F, 0x31, 0xD0, 0x08, 0x2E, 0xFA, 0x98, 0xEC,
        0x4E, 0x6C, 0x89, 0x45, 0x28, 0x21, 0xE6, 0x38, 0xD0, 0x13, 0x77, 0xBE, 0x54, 0x66, 0xCF,
        0x34, 0xE9, 0x0C, 0x6C, 0xC0, 0xAC, 0x29, 0xB7, 0xC9, 0x7C, 0x50, 0xDD, 0x3F, 0x84, 0xD5,
        0xB5, 0xB5, 0x47, 0x09, 0x17, 0x92, 0x16, 0xD5, 0xD9, 0x89, 0x79, 0xFB, 0x1B, 0xD1, 0x31,
        0x0B, 0xA6, 0x98, 0xDF, 0xB5, 0xAC, 0x2F, 0xFD, 0x72, 0xDB, 0xD0, 0x1A, 0xDF, 0xB7, 0xB8,
        0xE1, 0xAF, 0xED, 0x6A, 0x26, 0x7E, 0x96, 0xBA, 0x7C, 0x90, 0x45, 0xF1, 0x2C, 0x7F, 0x99,
        0x24, 0xA1, 0x99, 0x47, 0xB3, 0x91, 0x6C, 0xF7, 0x08, 0x01, 0xF2, 0xE2, 0x85, 0x8E, 0xFC,
        0x16, 0x63, 0x69, 0x20, 0xD8, 0x71, 0x57, 0x4E,
    ];

    // Subset of test cases from https://github.com/cryptocoinjs/base-x/blob/master/test/fixtures.json
    pub const TEST_CASES: &[(&[u8], &str)] = &[
        // (&[], ""),
        (&[0x61], "2g"),
        (&[0x62, 0x62, 0x62], "a3gV"),
        (&[0x63, 0x63, 0x63], "aPEr"),
        (&[0x57, 0x2e, 0x47, 0x94], "3EFU7m"),
        (&[0x10, 0xc8, 0x51, 0x1e], "Rt5zm"),
        (&[0x51, 0x6b, 0x6f, 0xcd, 0x0f], "ABnLTmg"),
        (
            &[0xbf, 0x4f, 0x89, 0x00, 0x1e, 0x67, 0x02, 0x74, 0xdd],
            "3SEo3LWLoPntC",
        ),
        (
            &[0xec, 0xac, 0x89, 0xca, 0xd9, 0x39, 0x23, 0xc0, 0x23, 0x21],
            "EJDM8drfXA6uyA",
        ),
        (
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            "1111111111",
        ),
        (
            &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
            "FPBt6CHo3fovdL",
        ),
        (
            &[
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            ],
            "NKioeUVktgzXLJ1B3t",
        ),
        (
            &[
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff,
            ],
            "YcVfxkQb6JRzqk5kF2tNLv",
        ),
        (
            &[
                0x73, 0x69, 0x6d, 0x70, 0x6c, 0x79, 0x20, 0x61, 0x20, 0x6c, 0x6f, 0x6e, 0x67, 0x20,
                0x73, 0x74, 0x72, 0x69, 0x6e, 0x67,
            ],
            "2cFupjhnEsSn59qHXstmK2ffpLv2",
        ),
        (
            &[
                0x00, 0xeb, 0x15, 0x23, 0x1d, 0xfc, 0xeb, 0x60, 0x92, 0x58, 0x86, 0xb6, 0x7d, 0x06,
                0x52, 0x99, 0x92, 0x59, 0x15, 0xae, 0xb1, 0x72, 0xc0, 0x66, 0x47,
            ],
            "1NS17iag9jJgTHD1VXjvLCEnZuQ3rJDE9L",
        ),
        (
            &[
                0x00, 0x3c, 0x17, 0x6e, 0x65, 0x9b, 0xea, 0x0f, 0x29, 0xa3, 0xe9, 0xbf, 0x78, 0x80,
                0xc1, 0x12, 0xb1, 0xb3, 0x1b, 0x4d, 0xc8, 0x26, 0x26, 0x81, 0x87,
            ],
            "16UjcYNBG9GTK4uq2f7yYEbuifqCzoLMGS",
        ),
        (
            &[
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
            "11111111111111111111111111111111",
        ),
        (
            &[
                0x80, 0x11, 0x84, 0xcd, 0x2c, 0xdd, 0x64, 0x0c, 0xa4, 0x2c, 0xfc, 0x3a, 0x09, 0x1c,
                0x51, 0xd5, 0x49, 0xb2, 0xf0, 0x16, 0xd4, 0x54, 0xb2, 0x77, 0x40, 0x19, 0xc2, 0xb2,
                0xd2, 0xe0, 0x85, 0x29, 0xfd, 0x20, 0x6e, 0xc9, 0x7e,
            ],
            "5Hx15HFGyep2CfPxsJKe2fXJsCVn5DEiyoeGGF6JZjGbTRnqfiD",
        ),
        (&DIGITS_OF_PI, "KeThPkHTv5nsa4576Z47NqEtuSfUcKwv7YeueZ8dquGTDeBpimjGEZ1a7k1FCz8m8FEBcoJZjP5Aui6eKfPjdmGooHKtEPRbVotw6mRxNU3WbLtAH41mea9g8AB9Qe1DAFDReBWa67ZEP6ApWGhw9Dfr2vVXkLXEWj6W8HFApw4DKK"),
    ];

    const FILLER: [u8; 512] = [b'~'; 512];

    #[test]
    fn test_encode() {
        // let mut bytes = [b'~'; 32];
        // let x = BITCOIN.encode(&[0x62, 0x62, 0x62], &mut bytes);
        // eprintln!("bytes: {:?}", bytes);
        // eprintln!("{:?}", &bytes["a3gV".len()..]);

        // for &(val, s) in TEST_CASES.iter() {
        //     eprintln!("{s}");
        //     assert_eq!(s.as_bytes(), BITCOIN.encode_to_vec(val));
        //     {
        //         let mut bytes = FILLER;
        //         assert_eq!(s.len(), BITCOIN.encode(val, &mut bytes));
        //         assert_eq!(s.as_bytes(), &bytes[..s.len()]);
        //         assert_eq!(&FILLER[s.len()..], &bytes[s.len()..]);
        //     }

        //     {
        //         let mut bytes = FILLER;
        //         if !s.is_empty() {
        //             bytes[(s.len() - 1)..=s.len()].copy_from_slice("Ę".as_bytes());
        //         }
        //         let string = core::str::from_utf8_mut(&mut bytes[..]).unwrap();
        //         let b = unsafe { string.as_bytes_mut() };

        //         assert_eq!(s.len(), BITCOIN.encode(val, b));
        //         assert_eq!(s.as_bytes(), &bytes[..s.len()]);
        //         assert_eq!(&FILLER[(s.len() + 1)..], &bytes[(s.len() + 1)..]);
        //     }
        // }
    }

    #[test]
    fn test_base58() {
        let test_addr = [
            *b"1QCaxc8hutpdZ62iKZsn1TCG3nh7uPZojq",
            *b"1DhRmSGnhPjUaVPAj48zgPV9e2oRhAQFUb",
            *b"17LN2oPYRYsXS9TdYdXCCDvF2FegshLDU2",
            *b"14h2bDLZSuvRFhUL45VjPHJcW667mmRAAn",
        ];

        for addr in test_addr {
            let bytes = BITCOIN.decode_to_vec(&addr).unwrap();
            let encoded = BITCOIN.encode_to_vec(&bytes);
            assert_eq!(addr.as_slice(), encoded);
        }
    }
}
