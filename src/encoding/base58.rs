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

    /// Encodes the given src into the given dst, returning the number of bytes written to dst.
    #[inline]
    pub fn encode(&self, src: &[u8], dst: &mut [u8]) -> usize {
        let (size, zcount) = Self::max_encoded_len_in(src);
        self.encode_in(src, dst, size, zcount)
    }

    /// Encodes the given src into a new vec.
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn encode_to_vec(&self, src: &[u8]) -> alloc::vec::Vec<u8> {
        let (size, zcount) = Self::max_encoded_len_in(src);
        let mut out = alloc::vec![0; size];
        let n = self.encode_in(src, &mut out, size, zcount);
        out.truncate(n);
        out
    }

    #[inline]
    fn encode_in(&self, src: &[u8], dst: &mut [u8], mut size: usize, zcount: usize) -> usize {
        let mut high = size - 1;
        for b in src {
            let mut i = size - 1;
            let mut carry = *b as u32;
            while i > high || carry != 0 {
                carry += 256 * (dst[i] as u32);
                dst[i] = (carry % 58) as u8;
                carry /= 58;
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
