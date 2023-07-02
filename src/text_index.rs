// use crate::errors::TextError;
// use crate::TextBuilder;
//
// #[derive(Debug, Eq, PartialEq)]
// pub enum TextIndex {
//     Bytes(usize),
//     Chars(usize),
//     Graphemes(usize),
// }
// impl TextIndex {
//     pub(crate) fn get_byte_index(&self, tb: &TextBuilder) -> Result<usize, TextError> {
//         match self {
//             TextIndex::Bytes(byte_index) => {
//                 let i = *byte_index;
//                 let len = tb.len_bytes();
//                 if i >= len {
//                     Err(TextError::IndexOutOfRange(i))
//                 } else {
//                     Ok(i)
//                 }
//             }
//             TextIndex::Chars(char_index) => {
//                 let i = *char_index;
//                 let char_index = tb.enumerate_chars().nth(i);
//                 if let Some(char_pos) = char_index {
//                     Ok(char_pos.0)
//                 } else {
//                     Err(TextError::IndexOutOfRange(i))
//                 }
//             }
//             TextIndex::Graphemes(grapheme_index) => {
//                 let i = *grapheme_index;
//                 let grapheme_index = tb.enumerate_graphemes().nth(i);
//                 if let Some(grapheme_pos) = grapheme_index {
//                     Ok(grapheme_pos.0)
//                 } else {
//                     Err(TextError::IndexOutOfRange(i))
//                 }
//             }
//             _ => {
//                 unimplemented!()
//             }
//         }
//     }
// }
