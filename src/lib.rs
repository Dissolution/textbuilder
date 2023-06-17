mod errors;
mod traits;

use crate::errors::TextError;
use std::str::*;
use std::string::*;
pub use traits::*;

// HACK!
#[cfg(windows)]
pub const DEFAULT_NEWLINE: &str = "\r\n";
#[cfg(windows)]
pub const DEFAULT_NEWLINE_BYTES: [u8; 2] = [0x0D, 0x0A]; // \r\n
#[cfg(unix)]
pub const DEFAULT_NEWLINE: &str = "\n";
#[cfg(unix)]
pub const DEFAULT_NEWLINE_BYTES: [u8; 1] = [0x0A]; // \n

pub struct TextBuilder {
    /// These are the utf8-encoded bytes of the characters written thus far
    bytes: Vec<u8>,
}

impl TextBuilder {
    #[inline]
    pub const fn new() -> Self {
        Self { bytes: Vec::new() }
    }
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        String::ut
    }

    pub fn append_utf8_bytes(&mut self, bytes: &[u8]) -> Result<&mut Self, TextError> {
        let str = from_utf8(bytes)?;
        self.bytes.extend_from_slice(str.as_bytes());
        Ok(self)
    }
    pub fn append_utf16_bytes(&mut self, bytes: &[u16]) -> Result<&mut Self, TextError> {
        let mut buffer = [0; 4];

        let char_iter = char::decode_utf16(bytes.to_owned());
        for decode_result in char_iter {
            match decode_result {
                Ok(ch) => {
                    let str = char::encode_utf8(ch, &mut buffer);
                    self.bytes.extend_from_slice(str.as_bytes());
                }
                Err(ex) => {
                    return Err(ex.into());
                }
            }
        }
        Ok(self)
    }

    pub fn append<A>(&mut self, value: &A) -> &mut Self
    where
        A: Appendable,
    {
        value.write_value(self)
    }

    pub fn newline(&mut self) -> &mut Self {
        self.bytes.extend_from_slice(&DEFAULT_NEWLINE_BYTES);
        self
    }
}
//
// impl Debug for TextBuilder {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         write!(f, "\"{}\"", self.string)
//     }
// }
impl std::fmt::Display for TextBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = String::from_utf8(self.bytes.clone()).expect("bad utf8 bytes");
        write!(f, "{}", str)
    }
}
// impl Default for TextBuilder {
//     fn default() -> Self {
//         TextBuilder {
//             string: String::default(),
//         }
//     }
// }
//
// pub struct TextBuilderIterator<'a> {
//     chars: Chars<'a>,
// }
// impl<'a> TextBuilderIterator<'a> {
//     pub fn new(string: &'a String) -> Self {
//         Self {
//             chars: string.chars(),
//         }
//     }
// }
//
// impl<'a> Iterator for TextBuilderIterator<'a> {
//     type Item = char;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.chars.next()
//     }
// }

// in-library tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::TextBuilder;

    const DIGITS_COUNT: usize = 10;
    const DIGITS_STR: &str = "0123456789";
    const DIGITS_UTF8_BYTES: &[u8] = DIGITS_STR.as_bytes();

    #[test]
    fn test_append_utf8_bytes() {
        let mut text = TextBuilder::new();
        text.append_utf8_bytes(&DIGITS_UTF8_BYTES)
            .expect("Invalid Utf8 Bytes!");
        let str = text.to_string();
        assert_eq!(str.as_str(), DIGITS_STR);
    }

    #[test]
    #[should_panic(expected = "Invalid Utf8 Bytes!")]
    fn test_append_utf8_bytes_fails() {
        let mut text = TextBuilder::new();
        let digits_bytes: [u8; 6] = [0xFC, 0x80, 0x80, 0x80, 0x80, 0xAF];
        text.append_utf8_bytes(&digits_bytes)
            .expect("Invalid Utf8 Bytes!");
        let str = text.to_string();
        assert_eq!(str.as_str(), DIGITS_STR);
    }

    // #[test]
    // fn test_append_utf16_bytes() {
    //     let mut text = TextBuilder::new();
    //     let digits_bytes = DIGITS.as_bytes();
    //     text.append_utf16_bytes(digits_bytes)
    //         .expect("Invalid Utf8 Bytes!");
    //     let str = text.to_string();
    //     assert_eq!(str.as_str(), DIGITS);
    // }
    //
    // #[test]
    // #[should_panic(expected = "Invalid Utf8 Bytes!")]
    // fn test_append_utf16_bytes_fails() {
    //     let mut text = TextBuilder::new();
    //     let digits_bytes: [u8; 6] = [0xFC, 0x80, 0x80, 0x80, 0x80, 0xAF];
    //     text.append_utf8_bytes(&digits_bytes)
    //         .expect("Invalid Utf8 Bytes!");
    //     let str = text.to_string();
    //     assert_eq!(str.as_str(), DIGITS);
    // }

    #[test]
    fn test_append() {
        let mut text = TextBuilder::new();
        let string: String = DIGITS_STR.to_string();
        text.append(&string);
        assert_eq!(text.len(), DIGITS_COUNT);
        assert_eq!(text.to_string(), string);
    }

    // #[test]
    // fn test_all_supported_types() {
    //     let mut text = TextBuilder::new();
    //     text.append(String::from("hello"));
    //     text.append(',');
    //     text.append(b' ');
    //     text.append("world");
    //     text.append(" it works".as_bytes());
    //
    //     assert_eq!(text.string().unwrap(), "hello, world it works");
    // }
    //
    // #[test]
    // fn test_individual_unicode_characters() {
    //     let mut b = TextBuilder::default();
    //     b.append('‘');
    //     b.append("starts with and ends with");
    //     b.append('‗');
    //
    //     assert_eq!(b.string().unwrap(), "‘starts with and ends with‗");
    // }
    //
    // #[test]
    // fn test_tool_album() {
    //     let mut b = TextBuilder::default();
    //     b.append('\u{00C6}');
    //     b.append("nima");
    //
    //     assert_eq!(b.string().unwrap(), "\u{00C6}nima");
    // }
}
