// #![allow(dead_code, unused)]
//
// #[derive(Debug, Default)]
// pub(crate) struct NonDisplayPoint {
//     pub x: isize,
//     pub y: isize,
// }
//
// use textbuilder::*;
//
// pub struct TextConsts;
// impl TextConsts {
//     pub const LOWERCASE_LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyz";
//     pub const UPPERCASE_LETTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
//     pub const DIGITS: &'static str = "0123456789";
// }
//
// #[test]
// fn test_len_bytes() {
//     let mut tb = TextBuilder::new();
//
//     tb.append(TextConsts::DIGITS);
//     assert_eq!(tb.len_bytes(), 10);
//
//     tb.append('Ѧ');
//     assert_eq!(tb.len_bytes(), 12);
//
//     tb.append('チ');
//     assert_eq!(tb.len_bytes(), 15);
//
//     tb.append('🦀');
//     assert_eq!(tb.len_bytes(), 19);
//
//     tb.append("🧙‍♀️");
//     assert_eq!(tb.len_bytes(), 32);
// }
//
// #[test]
// fn test_len_chars() {
//     let mut tb = TextBuilder::new();
//
//     tb.append(TextConsts::DIGITS);
//     assert_eq!(tb.len_chars(), 10);
//
//     tb.append('Ѧ');
//     assert_eq!(tb.len_chars(), 11);
//
//     tb.append('チ');
//     assert_eq!(tb.len_chars(), 12);
//
//     tb.append('🦀');
//     assert_eq!(tb.len_chars(), 13);
//
//     tb.append("🧙‍♀️");
//     assert_eq!(tb.len_chars(), 17);
// }
//
// #[test]
// fn test_len_graphemes() {
//     let mut tb = TextBuilder::new();
//
//     tb.append(TextConsts::DIGITS);
//     assert_eq!(tb.len_graphemes(), 10);
//
//     tb.append('Ѧ');
//     assert_eq!(tb.len_graphemes(), 11);
//
//     tb.append('チ');
//     assert_eq!(tb.len_graphemes(), 12);
//
//     tb.append('🦀');
//     assert_eq!(tb.len_graphemes(), 13);
//
//     tb.append("🧙‍♀️");
//     assert_eq!(tb.len_graphemes(), 14);
// }
//
//
//
// #[test]
// fn test_iter_graphemes() {
//     let mut textbuilder = TextBuilder::default();
//     let str: &str = TextConsts::DIGITS;
//     textbuilder.append_ref(&str);
//     let tb_chars = textbuilder
//         .iter_graphemes()
//         .map(|g| g.chars())
//         .filter_map(|mut chs| chs.next())
//         .collect::<Vec<char>>();
//     let chars = str.chars().collect::<Vec<char>>();
//
//     assert_eq!(tb_chars.len(), chars.len());
//     for i in 0..chars.len() {
//         assert_eq!(tb_chars[i], chars[i]);
//     }
// }
//
