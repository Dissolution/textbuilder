use crate::Appendable;

/// Configuration details for a `TextBuilder` instance
pub struct TextBuilderConfig {
    pub newline: Box<dyn Appendable>,
    pub indents: Vec<Box<dyn Appendable>>,
}
impl Default for TextBuilderConfig {
    fn default() -> Self {
        Self {
            newline: Box::from('\n'),
            indents: Vec::new(),
        }
    }
}
