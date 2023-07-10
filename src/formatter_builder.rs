use crate::{TextBuilder, TextError};
use std::fmt::{Arguments, Debug, Display, Formatter, Result as FmtResult, Write};

struct FormatterBuilderDisplay<F>
where
    F: for<'b, 'f> Fn(FormatterBuilder<'b, 'f>) -> Result<FormatterBuilder<'b, 'f>, TextError>,
{
    func: F,
}
impl<F> Display for FormatterBuilderDisplay<F>
where
    F: for<'b, 'f> Fn(FormatterBuilder<'b, 'f>) -> Result<FormatterBuilder<'b, 'f>, TextError>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Create a builder now that we have a Formatter instance
        let text_builder = FormatterBuilder::new(f);
        // Run the stored func on the instance
        (self.func)(text_builder)?;
        Ok(())
    }
}

pub struct FormatterBuilder<'b, 'f>
where
    'f: 'b,
{
    pub formatter: &'b mut Formatter<'f>,
    pub newline: Box<str>,
    pub indents: Vec<Box<str>>,
}
impl<'b, 'f> FormatterBuilder<'b, 'f>
where
    'f: 'b,
{
    /// Create a new `TextBuilder` that wraps a `Formatter`
    pub fn new(formatter: &'b mut Formatter<'f>) -> Self {
        Self {
            formatter,
            newline: Box::from("\n"),
            indents: Vec::new(),
        }
    }
}
impl FormatterBuilder<'_, '_> {
    pub fn build_string<F>(func: F) -> String
    where
        F: for<'b, 'f> Fn(FormatterBuilder<'b, 'f>) -> Result<FormatterBuilder<'b, 'f>, TextError>,
    {
        let display = FormatterBuilderDisplay { func };
        format!("{}", display)
    }
}

impl<'b, 'f> TextBuilder for FormatterBuilder<'b, 'f>
where
    'f: 'b,
{
    fn newline(self) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        self.formatter.write_str(self.newline.as_ref())?;
        for indent in self.indents.iter() {
            self.formatter.write_str(indent.as_ref())?;
        }
        Ok(self)
    }

    fn str(self, str: &str) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        self.formatter.write_str(str)?;
        Ok(self)
    }

    fn char(self, ch: char) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        self.formatter.write_char(ch)?;
        Ok(self)
    }

    fn debug<D>(self, value: &D) -> Result<Self, TextError>
    where
        Self: Sized,
        D: Debug,
    {
        write!(self.formatter, "{:?}", value)?;
        Ok(self)
    }

    fn display<D>(self, value: &D) -> Result<Self, TextError>
    where
        Self: Sized,
        D: Display,
    {
        write!(self.formatter, "{}", value)?;
        Ok(self)
    }

    fn args(self, args: Arguments<'_>) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        self.formatter.write_fmt(args)?;
        Ok(self)
    }

    fn indented<B>(mut self, indent: &str, indented_build: B) -> Result<Self, TextError>
    where
        Self: Sized,
        B: Fn(Self) -> Result<Self, TextError>,
    {
        self.indents.push(Box::from(indent));
        self = (indented_build)(self)?;
        self.indents.pop();
        Ok(self)
    }
}
