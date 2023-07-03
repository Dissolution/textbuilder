use crate::text_display::TextDisplay;
use crate::Appendable;
use std::fmt::{Debug, Display, Formatter, LowerHex, UpperHex, Write};

pub struct TextBuilder<'w, 'f>(&'w mut Formatter<'f>)
where
    'f: 'w;

impl<'w, 'f> TextBuilder<'w, 'f>
where
    'f: 'w,
{
    pub fn new(formatter: &'w mut Formatter<'f>) -> Self {
        TextBuilder(formatter)
    }
}

impl TextBuilder<'_, '_> {
    pub fn build_string<F>(build: F) -> String
    where
        F: Fn(&mut TextBuilder),
    {
        let display = TextDisplay::new(build);
        format!("{}", display)
    }

    pub fn newline(&mut self) -> &mut Self {
        self.0.write_char('\n').unwrap();
        self
    }

    pub fn append<A: Appendable>(&mut self, value: A) -> &mut Self {
        value.write_to(self.0).unwrap();
        self
    }

    pub fn debug<D: Debug>(&mut self, value: &D) -> &mut Self {
        Debug::fmt(value, self.0).unwrap();
        self
    }

    pub fn display<D: Display>(&mut self, value: &D) -> &mut Self {
        Display::fmt(value, self.0).unwrap();
        self
    }

    pub fn lower_hex<H: LowerHex>(&mut self, value: &H) -> &mut Self {
        LowerHex::fmt(value, self.0).unwrap();
        self
    }

    pub fn upper_hex<H: UpperHex>(&mut self, value: &H) -> &mut Self {
        UpperHex::fmt(value, self.0).unwrap();
        self
    }

    pub fn append_bytes<R: AsRef<[u8]>>(&mut self, value: R) -> &mut Self {
        let bytes = value.as_ref();
        let str = std::str::from_utf8(bytes).unwrap();
        self.0.write_str(str).unwrap();
        self
    }

    pub fn value<V, A: Appendable>(&mut self, value: &V, transform: fn(&V) -> A) -> &mut Self {
        let appendable = (transform)(value);
        self.append(appendable)
    }

    pub fn enumerate<I: Iterator>(
        &mut self,
        iter: I,
        per_item: fn(&mut Self, usize, I::Item),
    ) -> &mut Self {
        for (i, item) in iter.enumerate() {
            (per_item)(self, i, item);
        }
        self
    }

    pub fn delimit<I: Iterator>(
        &mut self,
        delimit: fn(&mut Self),
        iter: I,
        per_item: fn(&mut Self, usize, I::Item),
    ) -> &mut Self {
        for (i, item) in iter.enumerate() {
            if i > 0 {
                (delimit)(self);
            }
            (per_item)(self, i, item);
        }
        self
    }

    pub fn append_delimit<A: Appendable, I: Iterator>(
        &mut self,
        delimiter: &A,
        iter: I,
        per_item: fn(&mut Self, usize, I::Item),
    ) -> &mut Self {
        for (i, item) in iter.enumerate() {
            if i > 0 {
                delimiter.write_to(self.0);
            }
            (per_item)(self, i, item);
        }
        self
    }
}
