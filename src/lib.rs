#![allow(dead_code)] // As a library... ;-)

mod appendable;
mod errors;
mod text_builder;
mod text_display;
mod text_builder_config;
mod scratch;

pub use appendable::*;
pub use errors::*;
pub use text_builder::*;

/*
// in-library tests
#[cfg(test)]
mod tests {
    use super::*;

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
*/
