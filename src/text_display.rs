use crate::text_builder::TextBuilder;
use crate::Appendable;
use std::fmt::{Debug, Display, Formatter, LowerHex, UpperHex, Write};

pub(crate) struct TextDisplay<F>(F)
where
    F: Fn(&mut TextBuilder);

impl<F> TextDisplay<F>
where
    F: Fn(&mut TextBuilder),
{
    pub fn new(f: F) -> Self {
        TextDisplay(f)
    }
}

impl<F> Display for TextDisplay<F>
where
    F: Fn(&mut TextBuilder),
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut text_builder = TextBuilder::new(f);
        (self.0)(&mut text_builder);
        Ok(())
    }
}
