use crate::{TextBuilder, TextError};
use std::fmt::{Arguments, Debug, Display, Write};

pub struct StringBuilder {
    pub string: String,
    pub newline: Box<str>,
    pub indents: Vec<Box<str>>,
}
impl StringBuilder {
    pub fn new() -> Self {
        StringBuilder {
            string: String::new(),
            newline: Box::from("\n"),
            indents: Vec::new(),
        }
    }

    pub fn build_string<B>(build: B) -> String
    where
        Self: Sized,
        B: Fn(Self) -> Result<Self, TextError>,
    {
        let mut builder = Self::new();
        builder = (build)(builder).unwrap();
        builder.string
    }
}
impl Default for StringBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TextBuilder for StringBuilder {
    fn newline(mut self) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        self.string.push_str(self.newline.as_ref());
        for indent in self.indents.iter() {
            self.string.push_str(indent.as_ref());
        }
        Ok(self)
    }

    fn str(mut self, str: &str) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        self.string.push_str(str);
        Ok(self)
    }

    fn char(mut self, ch: char) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        self.string.push(ch);
        Ok(self)
    }

    fn debug<D>(mut self, value: &D) -> Result<Self, TextError>
    where
        Self: Sized,
        D: Debug,
    {
        self.string.write_fmt(format_args!("{:?}", value))?;
        Ok(self)
    }

    fn display<D>(mut self, value: &D) -> Result<Self, TextError>
    where
        Self: Sized,
        D: Display,
    {
        self.string.write_fmt(format_args!("{}", value))?;
        Ok(self)
    }

    fn args(mut self, args: Arguments<'_>) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        self.string.write_fmt(args)?;
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
