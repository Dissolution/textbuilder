use crate::TextBuilder;

pub trait Appendable {
    fn write_value<'a>(&self, textbuilder: &'a mut TextBuilder) -> &'a mut TextBuilder;
}

impl Appendable for String {
    fn write_value<'a>(&self, textbuilder: &'a mut TextBuilder) -> &'a mut TextBuilder {
        textbuilder.bytes.extend_from_slice(self.as_bytes());
        textbuilder
    }
}
