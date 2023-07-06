use crate::TextError;
use std::fmt::Display;

pub struct TextBuilderConfig {
    pub newline: Box<dyn Appendable>,
    pub indents: Vec<Box<dyn Appendable + 'static>>,
}
impl Default for TextBuilderConfig {
    fn default() -> Self {
        Self {
            newline: Box::from('\n'),
            indents: Vec::new(),
        }
    }
}

pub trait Appendable: Display {
    fn get_str(&self) -> &str;
}
impl Appendable for char {
    fn get_str(&self) -> &str {
        &self.to_string()
    }
}
impl Appendable for &str {
    fn get_str(&self) -> &str {
        self
    }
}
impl Appendable for String {
    fn get_str(&self) -> &str {
        self.as_ref()
    }
}

pub trait TextBuilder {
    fn build_string<B>(build: B) -> String
    where
        Self: Sized,
        B: Fn(Self) -> Result<Self, TextError>;

    fn set_newline<N>(self, newline: N) -> Result<Self, TextError>
    where
        Self: Sized,
        N: Appendable + 'static;

    fn newline(self) -> Result<Self, TextError>
    where
        Self: Sized;

    fn append<A>(self, appendable: A) -> Result<Self, TextError>
    where
        Self: Sized,
        A: Appendable;

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
}

pub trait TextFormatterBuilder: TextBuilder {}
