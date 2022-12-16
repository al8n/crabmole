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

    ///
    #[inline]
    pub fn encode(&self, src: &[u8], dst: &mut [u8]) {
        let mut index = 0;
        for &val in src {
            let mut carry = val as usize;
            for byte in &mut dst[..index] {
                carry += (*byte as usize) << 8;
                *byte = (carry % BASE) as u8;
                carry /= BASE;
            }
            while carry > 0 {
                dst[index] = (carry % BASE) as u8;
                index += 1;
                carry /= BASE;
            }
        }

        for _ in src.iter().take_while(|v| **v == 0) {
            dst[index] = 0;
            index += 1;
        }

        for val in &mut dst[..index] {
            *val = self.encode[*val as usize];
        }

        dst[..index].reverse();
    }

    /// Returns the max encoded len
    #[inline]
    pub const fn max_encoded_len(n: usize) -> usize {
        // TODO: wait #![feature(const_fn_floating_point_arithmetic)] stable then use
        // ((n as f64) * 1.365658237309761) as usize + 1
        n + (n + 1) / 2
    }

    /// Returns the max decoded len
    #[inline]
    pub const fn max_decoded_len(n: usize) -> usize {
        // TODO: wait #![feature(const_fn_floating_point_arithmetic)] stable then use
        // (((n - 1) as f64) / 1.365658237309761) as usize
        ((2 * n) - 1) / 3 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {}
}
