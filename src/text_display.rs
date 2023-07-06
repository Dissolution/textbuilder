use crate::text_builder::TextBuilder;
use crate::TextError;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// # TextDisplay
/// A helper struct to bridge the gap between `TextBuilder` and actually creating a `String`
pub(crate) struct TextBuilderDisplay<F>
where
    F: for<'b, 'f> Fn(TextBuilder<'b, 'f>) -> Result<TextBuilder<'b, 'f>, TextError>,
{
    pub(crate) func: F,
}
impl<F> Display for TextBuilderDisplay<F>
where
    F: for<'b, 'f> Fn(TextBuilder<'b, 'f>) -> Result<TextBuilder<'b, 'f>, TextError>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // Create a builder now that we have a Formatter instance
        let text_builder = TextBuilder::new(f);
        // Run the stored func on the instance
        (self.func)(text_builder)?;
        FmtResult::Ok(())
    }
}
impl<F> Debug for TextBuilderDisplay<F>
where
    F: for<'b, 'f> Fn(TextBuilder<'b, 'f>) -> Result<TextBuilder<'b, 'f>, TextError>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let tb = TextBuilder::new(f);
        (self.func)(tb)?;
        Ok(())
    }
}
