use crate::text_builder_config::TextBuilderConfig;
use crate::text_display::TextBuilderDisplay;
use crate::{Appendable, TextError};
use anyhow::anyhow;
use std::fmt::{Arguments, Debug, Display, Formatter, LowerHex, UpperHex};

/// # `TextBuilder`
/// A wrapper around `std::fmt::Formatter` that provides fluent operations.
/// `'b` - the lifetime of the `TextBuilder`  
/// `'f` - the lifetime of the `Formatter`
/// The lifetime of the `Formatter` _must_ outlive the lifetime of the `TextBuilder`
pub struct TextBuilder<'b, 'f>
where
    'f: 'b,
{
    /// The wrapped `Formatter`
    formatter: &'b mut Formatter<'f>,
    pub config: TextBuilderConfig,
}

impl<'b, 'f> TextBuilder<'b, 'f>
where
    'f: 'b,
{
    /// Create a new `TextBuilder` that wraps a `Formatter`
    pub fn new(formatter: &'b mut Formatter<'f>) -> Self {
        TextBuilder {
            formatter,
            config: TextBuilderConfig::default(),
        }
    }
}
impl TextBuilder<'_, '_> {
    pub fn build_string<F>(func: F) -> String
    where
        F: for<'b, 'f> Fn(TextBuilder<'b, 'f>) -> Result<TextBuilder<'b, 'f>, TextError>,
    {
        let text_display = TextBuilderDisplay { func };
        format!("{}", text_display)
    }

    pub fn newline(self) -> Result<Self, TextError> {
        self.config.newline.write_to(self.formatter)?;
        for indent in self.config.indents.iter() {
            indent.write_to(self.formatter)?;
        }
        Ok(self)
    }

    pub fn append<A: Appendable>(self, value: A) -> Result<Self, TextError> {
        value.write_to(self.formatter)?;
        Ok(self)
    }

    pub fn debug<D: Debug>(self, value: &D) -> Result<Self, TextError> {
        Debug::fmt(value, self.formatter)?;
        Ok(self)
    }

    pub fn display<D: Display>(self, value: &D) -> Result<Self, TextError> {
        Display::fmt(value, self.formatter)?;
        Ok(self)
    }

    pub fn lower_hex<H: LowerHex>(self, value: &H) -> Result<Self, TextError> {
        LowerHex::fmt(value, self.formatter)?;
        Ok(self)
    }

    pub fn upper_hex<H: UpperHex>(self, value: &H) -> Result<Self, TextError> {
        UpperHex::fmt(value, self.formatter)?;
        Ok(self)
    }

    pub fn utf8_bytes<R: AsRef<[u8]>>(self, value: R) -> Result<Self, TextError> {
        let bytes = value.as_ref();
        let str = std::str::from_utf8(bytes)?;
        self.formatter.write_str(str)?;
        Ok(self)
    }

    pub fn value<V, A: Appendable>(
        self,
        value: &V,
        transform: fn(&V) -> A,
    ) -> Result<Self, TextError> {
        let appendable = (transform)(value);
        self.append(appendable)
    }

    pub fn args(self, args: Arguments<'_>) -> Result<Self, TextError> {
        self.formatter.write_fmt(args)?;
        Ok(self)
    }

    pub fn enumerate<I, F>(mut self, iter: I, per_item: F) -> Result<Self, TextError>
    where
        I: Iterator,
        F: for<'b, 'f> Fn(
            TextBuilder<'b, 'f>,
            usize,
            I::Item,
        ) -> Result<TextBuilder<'b, 'f>, TextError>,
    {
        for (i, item) in iter.enumerate() {
            self = (per_item)(self, i, item)?;
        }
        Ok(self)
    }

    pub fn delimit<D, I, F>(mut self, delimit: D, iter: I, per_item: F) -> Result<Self, TextError>
    where
        D: for<'b, 'f> Fn(TextBuilder<'b, 'f>) -> Result<TextBuilder<'b, 'f>, TextError>,
        I: Iterator,
        F: for<'b, 'f> Fn(
            TextBuilder<'b, 'f>,
            usize,
            I::Item,
        ) -> Result<TextBuilder<'b, 'f>, TextError>,
    {
        for (i, item) in iter.enumerate() {
            if i > 0 {
                self = (delimit)(self)?;
            }
            self = (per_item)(self, i, item)?;
        }
        Ok(self)
    }

    pub fn append_delimit<D, I, F>(
        mut self,
        delimiter: D,
        iter: I,
        per_item: F,
    ) -> Result<Self, TextError>
    where
        D: Appendable,
        I: Iterator,
        F: for<'b, 'f> Fn(
            TextBuilder<'b, 'f>,
            usize,
            I::Item,
        ) -> Result<TextBuilder<'b, 'f>, TextError>,
    {
        for (i, item) in iter.enumerate() {
            if i > 0 {
                delimiter.write_to(self.formatter)?;
            }
            self = (per_item)(self, i, item)?;
        }
        Ok(self)
    }

    pub fn indented<I, F>(mut self, indent: I, indented_build: F) -> Result<Self, TextError>
    where
        I: Appendable + 'static,
        F: for<'b, 'f> Fn(TextBuilder<'b, 'f>) -> Result<TextBuilder<'b, 'f>, TextError>,
    {
        // push a new indent
        self.config.indents.push(Box::new(indent));
        self = indented_build(self)?;
        // pop it off
        self.config
            .indents
            .pop()
            .ok_or(anyhow!("Could not pop indent"))?;
        Ok(self)
    }
}
