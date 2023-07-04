// use thiserror::Error;
//
// #[derive(Error, Debug)]
// pub enum TextError {
//     #[error("Utf8 Encoding Error")]
//     Utf8Error(#[from] std::str::Utf8Error),
//     #[error("Utf8 Encoding Error")]
//     FromUtf8Error(#[from] std::string::FromUtf8Error),
//     #[error("Utf16 Encoding Error")]
//     FromUtf16Error(#[from] std::string::FromUtf16Error),
//     #[error("Utf16 Encoding Error")]
//     DecodeUtf16Error(#[from] std::char::DecodeUtf16Error),
//     #[error("Miscellaneous error")]
//     MiscError(String),
// }

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TextError {
    #[error("index `{0}` is out of range")]
    IndexOutOfRange(usize),
    #[error("formatting failed")]
    Formatting(#[from] std::fmt::Error),
    #[error("Utf8 Encoding Error")]
    Utf8Error(#[from] std::str::Utf8Error),
}

impl From<TextError> for std::fmt::Error {
    fn from(text_error: TextError) -> Self {
        match text_error {
            TextError::Formatting(err) => err,
            _ => std::fmt::Error::default(),
        }
    }
}
