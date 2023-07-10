use crate::TextError;
use std::fmt::{Arguments, Debug, Display};

/// # A fluent Builder of Text
/// This trait species that its implementer can fluently perform textual operations
/// that build up a `String`
pub trait TextBuilder {
    /// Appends the current newline (and any indents) to this `TextBuilder`
    fn newline(self) -> Result<Self, TextError>
    where
        Self: Sized;

    /// Appends a `&str` to this `TextBuilder`
    fn str(self, str: &str) -> Result<Self, TextError>
    where
        Self: Sized;

    /// Appends a `char` to this `TextBuilder`
    fn char(self, ch: char) -> Result<Self, TextError>
    where
        Self: Sized;

    /// Appends a `value` that implements `Debug`
    fn debug<D>(self, value: &D) -> Result<Self, TextError>
    where
        Self: Sized,
        D: Debug;
    /// Appends a `value` that implements `Display`
    fn display<D>(self, value: &D) -> Result<Self, TextError>
    where
        Self: Sized,
        D: Display;

    /// Appends an argument formatting result  
    /// Note: Use the `format_args!()` macro to generate `Arguments`
    fn args(self, args: Arguments<'_>) -> Result<Self, TextError>
    where
        Self: Sized;

    /// # Enumerate
    /// Enumerate is a powerful function that iterates over `iter`, performing a `Fn` for each item
    /// that uses the `TextBuilder`, the current iteration `index`, and the `item` to perform some
    /// sort of action (generally appending the value in some way)
    fn enumerate<I, B>(mut self, iter: I, build_item: B) -> Result<Self, TextError>
    where
        Self: Sized,
        I: Iterator,
        B: Fn(Self, usize, I::Item) -> Result<Self, TextError>,
    {
        for (i, item) in iter.enumerate() {
            self = (build_item)(self, i, item)?;
        }
        Ok(self)
    }

    /// # Delimit
    /// Delimit is a powerful function that combines `Enumerate` with a `delimit` function that can
    /// add a delimiter between values
    fn delimit<D, I, B>(mut self, delimit: D, iter: I, build_item: B) -> Result<Self, TextError>
    where
        Self: Sized,
        D: Fn(Self) -> Result<Self, TextError>,
        I: Iterator,
        B: Fn(Self, usize, I::Item) -> Result<Self, TextError>,
    {
        for (i, item) in iter.enumerate() {
            if i > 0 {
                self = (delimit)(self)?;
            }
            self = (build_item)(self, i, item)?;
        }
        Ok(self)
    }

    fn indented<B>(self, indent: &str, indented_build: B) -> Result<Self, TextError>
    where
        Self: Sized,
        B: Fn(Self) -> Result<Self, TextError>;
}
