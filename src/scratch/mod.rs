mod textbuilder;

use crate::TextError;
use std::fmt::{Arguments, Debug, Display, Formatter, LowerHex, UpperHex, Write};

pub trait Appendable: Display {
    fn write_to<T>(&self, text_builder: &mut T) -> Result<(), TextError>
    where
        T: TextBuilder;
}

pub trait TextBuilder {
    fn build_string<F>(func: F) -> String
    where
        Self: Sized,
        F: for<'b, 'f> Fn(Self) -> Result<Self, TextError>;

    fn set_newline<N>(&mut self, newline: N)
    where
        N: Appendable + 'static;

    fn newline(self) -> Result<Self, TextError>
    where
        Self: Sized;

    fn append<A: Appendable>(self, value: A) -> Result<Self, TextError>
    where
        Self: Sized;

    fn debug<D: Debug>(self, value: &D) -> Result<Self, TextError>
    where
        Self: Sized;

    fn display<D: Display>(self, value: &D) -> Result<Self, TextError>
    where
        Self: Sized;

    fn lower_hex<H: LowerHex>(self, value: &H) -> Result<Self, TextError>
    where
        Self: Sized;

    fn upper_hex<H: UpperHex>(self, value: &H) -> Result<Self, TextError>
    where
        Self: Sized;

    fn utf8_bytes<R: AsRef<[u8]>>(self, value: R) -> Result<Self, TextError>
    where
        Self: Sized;

    fn value<V, A: Appendable>(self, value: &V, transform: fn(&V) -> A) -> Result<Self, TextError>
    where
        Self: Sized;

    fn args(self, args: Arguments<'_>) -> Result<Self, TextError>
    where
        Self: Sized;

    fn enumerate<I, F>(mut self, iter: I, per_item: F) -> Result<Self, TextError>
    where
        Self: Sized,
        I: Iterator,
        F: Fn(Self, usize, I::Item) -> Result<Self, TextError>,
    {
        for (i, item) in iter.enumerate() {
            self = (per_item)(self, i, item)?;
        }
        Ok(self)
    }

    fn delimit<D, I, F>(mut self, delimit: D, iter: I, per_item: F) -> Result<Self, TextError>
    where
        Self: Sized,
        D: Fn(Self) -> Result<Self, TextError>,
        I: Iterator,
        F: Fn(Self, usize, I::Item) -> Result<Self, TextError>,
    {
        for (i, item) in iter.enumerate() {
            if i > 0 {
                self = (delimit)(self)?;
            }
            self = (per_item)(self, i, item)?;
        }
        Ok(self)
    }

    fn append_delimit<D, I, F>(
        mut self,
        delimiter: D,
        iter: I,
        per_item: F,
    ) -> Result<Self, TextError>
    where
        Self: Sized,
        D: Appendable,
        I: Iterator,
        F: Fn(Self, usize, I::Item) -> Result<Self, TextError>,
    {
        for (i, item) in iter.enumerate() {
            if i > 0 {
                delimiter.write_to(&mut self)?;
            }
            self = (per_item)(self, i, item)?;
        }
        Ok(self)
    }

    fn indented<I, F>(self, indent: I, indented_build: F) -> Result<Self, TextError>
    where
        Self: Sized,
        I: Appendable + 'static,
        F: Fn(Self) -> Result<Self, TextError>;
}

pub struct StringBuilder {
    string: String,
}
impl TextBuilder for StringBuilder {
    fn build_string<F>(func: F) -> String
    where
        Self: Sized,
        F: for<'b, 'f> Fn(Self) -> Result<Self, TextError>,
    {
        todo!()
    }

    fn set_newline<N>(&mut self, newline: N)
    where
        N: Appendable + 'static,
    {
        todo!()
    }

    fn newline(mut self) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        self.string.push('\n');
        Ok(self)
    }

    fn append<A: Appendable>(self, value: A) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn debug<D: Debug>(self, value: &D) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn display<D: Display>(self, value: &D) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn lower_hex<H: LowerHex>(self, value: &H) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn upper_hex<H: UpperHex>(self, value: &H) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn utf8_bytes<R: AsRef<[u8]>>(self, value: R) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn value<V, A: Appendable>(self, value: &V, transform: fn(&V) -> A) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn args(self, args: Arguments<'_>) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn enumerate<I, F>(self, iter: I, per_item: F) -> Result<Self, TextError>
    where
        Self: Sized,
        I: Iterator,
        F: Fn(Self, usize, I::Item) -> Result<Self, TextError>,
    {
        todo!()
    }

    fn delimit<D, I, F>(self, delimit: D, iter: I, per_item: F) -> Result<Self, TextError>
    where
        Self: Sized,
        D: Fn(Self) -> Result<Self, TextError>,
        I: Iterator,
        F: Fn(Self, usize, I::Item) -> Result<Self, TextError>,
    {
        todo!()
    }

    fn append_delimit<D, I, F>(self, delimiter: D, iter: I, per_item: F) -> Result<Self, TextError>
    where
        Self: Sized,
        D: Appendable,
        I: Iterator,
        F: Fn(Self, usize, I::Item) -> Result<Self, TextError>,
    {
        todo!()
    }

    fn indented<I, F>(self, indent: I, indented_build: F) -> Result<Self, TextError>
    where
        Self: Sized,
        I: Appendable + 'static,
        F: Fn(Self) -> Result<Self, TextError>,
    {
        todo!()
    }
}

pub struct FormatterBuilder<'b, 'f>
where
    'f: 'b,
{
    /// The wrapped `Formatter`
    formatter: &'b mut Formatter<'f>,
    //pub config: TextBuilderConfig,
}
impl TextBuilder for FormatterBuilder<'_, '_> {
    fn build_string<F>(func: F) -> String
    where
        Self: Sized,
        F: for<'b, 'f> Fn(Self) -> Result<Self, TextError>,
    {
        todo!()
    }

    fn set_newline<N>(&mut self, newline: N)
    where
        N: Appendable + 'static,
    {
        todo!()
    }

    fn newline(self) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        self.formatter.write_char('\n')?;
        Ok(self)
    }

    fn append<A: Appendable>(self, value: A) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn debug<D: Debug>(self, value: &D) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn display<D: Display>(self, value: &D) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn lower_hex<H: LowerHex>(self, value: &H) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn upper_hex<H: UpperHex>(self, value: &H) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn utf8_bytes<R: AsRef<[u8]>>(self, value: R) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn value<V, A: Appendable>(self, value: &V, transform: fn(&V) -> A) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn args(self, args: Arguments<'_>) -> Result<Self, TextError>
    where
        Self: Sized,
    {
        todo!()
    }
}
