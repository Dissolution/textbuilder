use crate::text_builder::TextBuilder;
use crate::{Appendable, TextError};
use anyhow::*;
use std::fmt::{Display, Formatter, Result as FmtResult};
use thiserror::__private::AsDynError;

/// # TextDisplay
pub(crate) struct TextDisplay<F>
where
    F: for<'b, 'f> Fn(TextBuilder<'b, 'f>) -> Result<TextBuilder<'b, 'f>, TextError>,
{
    pub(crate) func: F,
}
impl<F> Display for TextDisplay<F>
where
    F: for<'b, 'f> Fn(TextBuilder<'b, 'f>) -> Result<TextBuilder<'b, 'f>, TextError>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Create a builder now that we have a Formatter instance
        let mut fb = TextBuilder::new(f);
        // Run the stored func on the instance
        (self.func)(fb)?;
        FmtResult::Ok(())
    }
}
