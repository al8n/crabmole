#![allow(non_upper_case_globals)]
use super::{is_excluding_latin, RangeTable, MAX_LATIN1};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unicode::{is, is_digit};

    fn digit_tests() -> Vec<char> {
        vec![
            char::from_u32(0x0030).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0039).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0661).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x06F1).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x07C9).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0966).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x09EF).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0A66).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0AEF).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0B66).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0B6F).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0BE6).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0BEF).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0C66).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0CEF).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0D66).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0D6F).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0E50).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0E59).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0ED0).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0ED9).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0F20).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0F29).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1040).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1049).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1090).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1091).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1099).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x17E0).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x17E9).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1810).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1819).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1946).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x194F).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x19D0).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x19D9).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1B50).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1B59).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1BB0).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1BB9).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1C40).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1C49).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1C50).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1C59).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0xA620).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0xA629).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0xA8D0).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0xA8D9).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0xA900).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0xA909).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0xAA50).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0xAA59).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0xFF10).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0xFF19).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x104A1).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1D7CE).unwrap_or(char::REPLACEMENT_CHARACTER),
        ]
    }

    fn letter_tests() -> Vec<char> {
        vec![
            char::from_u32(0x0041).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0061).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x00AA).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x00BA).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x00C8).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x00DB).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x00F9).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x02EC).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0535).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x06E6).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x093D).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0A15).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0B99).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0DC0).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x0EDD).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1000).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1200).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1312).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1401).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x1885).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x2C00).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0xA800).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0xF900).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0xFA30).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0xFFDA).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0xFFDC).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x10000).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x10300).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x10400).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x20000).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x2F800).unwrap_or(char::REPLACEMENT_CHARACTER),
            char::from_u32(0x2FA1D).unwrap_or(char::REPLACEMENT_CHARACTER),
        ]
    }

    #[test]
    fn test_digit() {
        for r in digit_tests() {
            assert!(is_digit(r));
        }
        for ch in letter_tests() {
            assert!(!is_digit(ch));
        }
    }

    #[test]
    fn test_digit_optimizaiton() {
        let mut i = 0;
        while i <= MAX_LATIN1 as u32 {
            assert_eq!(
                is(
                    RangeTable::DIGIT,
                    char::from_u32(i).unwrap_or(char::REPLACEMENT_CHARACTER)
                ),
                is_digit(char::from_u32(i).unwrap_or(char::REPLACEMENT_CHARACTER))
            );
            i += 1;
        }
    }
}
