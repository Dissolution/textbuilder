//use std::ffi::{OsStr, OsString};
use std::fmt::*;

/// A thing whose representation is easily written to a `Formatter` instance
pub trait Appendable: Display {
    /// Write a representation of `self` to `formatter`
    fn write_to(&self, formatter: &mut Formatter<'_>) -> Result;
}
impl Appendable for char {
    fn write_to(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_char(*self)
    }
}
impl Appendable for &str {
    fn write_to(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_str(self)
    }
}
impl Appendable for String {
    fn write_to(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_str(self)
    }
}

#[derive(Default)]
pub(crate) struct NotAppendable;
impl Appendable for NotAppendable {
    fn write_to(&self, _: &mut Formatter<'_>) -> Result {
        // do nothing
        Ok(())
    }
}
impl Display for NotAppendable {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        // do nothing
        Ok(())
    }
}

// impl Appendable for &OsStr {
//     fn write_to(&self, formatter: &mut Formatter<'_>) -> Result {
//         match self.to_str() {
//             Some(str) => formatter.write_str(str),
//             None => {
//                 write!(formatter, "{:?}", self)
//             }
//         }
//     }
// }
// impl Appendable for OsString {
//     fn write_to(&self, formatter: &mut Formatter<'_>) -> Result {
//         self.as_os_str().write_to(formatter)
//     }
// }
