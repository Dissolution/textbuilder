use anyhow;
use std::fmt::*;

/// # `TextBuilder`
/// A builder of text-like values that uses `Formatter` and `FormatterBuilder` extensions
pub struct TextBuilder<F>
where
    F: Fn(&mut Formatter<'_>),
{
    /// a function (can be a closure) that uses (probably writes) to a `Formatter`
    /// and returns a `Result`.  
    /// _note:_ Most `Formatter` methods that would be commonly used follow this pattern.
    build: F,
}
impl<F> TextBuilder<F>
where
    F: Fn(&mut Formatter<'_>),
{
    /// Builds a `String` from a `Formatter` predicate
    pub fn build_string(build: F) -> String {
        let builder = TextBuilder { build };
        format!("{}", builder)
    }
}
impl<F> Display for TextBuilder<F>
where
    F: Fn(&mut Formatter<'_>),
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        (self.build)(f);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_build() {
        let text: String = TextBuilder::build_string(|f| {});
        assert_eq!(text.len(), 0);
        assert_eq!(text, String::from(""));
    }
}
