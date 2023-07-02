use crate::appendable::Appendable;
use std::ffi::{OsStr, OsString};
use std::fmt::*;
use std::path::Path;

/// A fluent builder patten on top of `std::fmt::Formatter`
pub trait FormatterBuilder {
    fn append_newline(&mut self) -> &mut Self;

    fn append<A: Appendable>(&mut self, value: A) -> &mut Self;
    fn append_debug<D: Debug>(&mut self, value: &D) -> &mut Self;
    fn append_display<D: Display>(&mut self, value: &D) -> &mut Self;
    fn append_lower_hex<H: LowerHex>(&mut self, value: &H) -> &mut Self;
    fn append_upper_hex<H: UpperHex>(&mut self, value: &H) -> &mut Self;
    fn append_bytes(&mut self, value: &[u8]) -> &mut Self;

    fn write<V, A: Appendable>(&mut self, value: &V, transform: fn(&V) -> A) -> &mut Self;

    fn enumerate<I: Iterator>(
        &mut self,
        iter: I,
        per_item: fn(&mut Self, usize, I::Item),
    ) -> &mut Self;
    fn delimit<I: Iterator>(
        &mut self,
        delimit: fn(&mut Self),
        iter: I,
        per_item: fn(&mut Self, usize, I::Item),
    ) -> &mut Self;
}

impl FormatterBuilder for Formatter<'_> {
    fn append_newline(&mut self) -> &mut Self {
        self.write_char('\n').unwrap();
        self
    }

    fn append<A: Appendable>(&mut self, value: A) -> &mut Self {
        value.write_to(self).unwrap();
        self
    }

    fn append_debug<D: Debug>(&mut self, value: &D) -> &mut Self {
        Debug::fmt(value, self).unwrap();
        self
    }

    fn append_display<D: Display>(&mut self, value: &D) -> &mut Self {
        Display::fmt(value, self).unwrap();
        self
    }

    fn append_lower_hex<H: LowerHex>(&mut self, value: &H) -> &mut Self {
        LowerHex::fmt(value, self).unwrap();
        self
    }

    fn append_upper_hex<H: UpperHex>(&mut self, value: &H) -> &mut Self {
        UpperHex::fmt(value, self).unwrap();
        self
    }

    fn append_bytes(&mut self, value: &[u8]) -> &mut Self {
        self.append(std::str::from_utf8(value).unwrap())
    }

    fn write<V, A: Appendable>(&mut self, value: &V, transform: fn(&V) -> A) -> &mut Self {
        let appendable = (transform)(value);
        self.append(appendable)
    }

    fn enumerate<I: Iterator>(
        &mut self,
        iter: I,
        per_item: fn(&mut Self, usize, I::Item),
    ) -> &mut Self {
        for (i, item) in iter.enumerate() {
            (per_item)(self, i, item);
        }
        self
    }

    fn delimit<I: Iterator>(
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
}
