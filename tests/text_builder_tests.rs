use std::fmt::*;
use textbuilder::*;
use unicode_segmentation::*;

pub struct TextConsts;
impl TextConsts {
    pub const LOWERCASE_LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyz";
    pub const UPPERCASE_LETTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    pub const DIGITS: &'static str = "0123456789";
}

#[test]
fn test_append_newline() {
    let string = TextBuilder::build_string(|f| f.newline());
    assert_eq!(string.as_bytes().len(), 1);
    assert_eq!(string.chars().count(), 1);
    assert_eq!(string.graphemes(false).count(), 1);
    assert_eq!(string, "\n");

    let string = TextBuilder::build_string(|mut tb| {
        tb.config.new_line = Box::new("\r\n");
        tb.newline()
    });
    assert_eq!(string.as_bytes().len(), 2);
    assert_eq!(string.chars().count(), 2);
    assert_eq!(string.graphemes(false).count(), 1);
    assert_eq!(string, "\r\n");
}

// Appendable

#[test]
fn test_append_ascii_char() {
    let string = TextBuilder::build_string(|f| f.append('J'));
    assert_eq!(string.as_bytes().len(), 1);
    assert_eq!(string.chars().count(), 1);
    assert_eq!(string.graphemes(false).count(), 1);
    assert_eq!(string, "J");
    assert_eq!(string, String::from('J'));
}

#[test]
fn test_append_unicode_char() {
    let string = TextBuilder::build_string(|f| f.append('💙'));
    assert_eq!(string.as_bytes().len(), 4);
    assert_eq!(string.chars().count(), 1);
    assert_eq!(string.graphemes(true).count(), 1);
    assert_eq!(string, "💙");
    assert_eq!(string, String::from('💙'));
}

#[test]
fn test_append_str() {
    let string = TextBuilder::build_string(|f| f.append(TextConsts::DIGITS));
    assert_eq!(string.as_bytes().len(), 10);
    assert_eq!(string.chars().count(), 10);
    assert_eq!(string.graphemes(false).count(), 10);
    assert_eq!(string, "0123456789");
    assert_eq!(string, TextConsts::DIGITS);
}

#[test]
fn test_append_string() {
    let string = TextBuilder::build_string(|f| f.append(TextConsts::DIGITS.to_string()));
    assert_eq!(string.as_bytes().len(), 10);
    assert_eq!(string.chars().count(), 10);
    assert_eq!(string.graphemes(false).count(), 10);
    assert_eq!(string, "0123456789");
    assert_eq!(string, TextConsts::DIGITS);
}

// TODO: &OsStr + OsString

#[derive(Default)]
struct TestStruct();
impl Debug for TestStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "DEBUG")
    }
}
impl Display for TestStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "DISPLAY")
    }
}

#[test]
fn test_append_debug() {
    let test: TestStruct = TestStruct::default();

    let string = TextBuilder::build_string(|f| f.debug(&test));

    assert_eq!(string, "DEBUG");
}

#[test]
fn test_append_display() {
    let test: TestStruct = TestStruct::default();

    let string = TextBuilder::build_string(|f| f.display(&test));

    assert_eq!(string, "DISPLAY");
}

#[test]
fn test_args() {
    let string = TextBuilder::build_string(|tb| tb.args(format_args!("{:0>8}", 147)));
    assert_eq!(string, "00000147");
}

#[test]
fn test_enumerate() {
    let numbers = [0, 1, 2, 3, 4];

    let string = TextBuilder::build_string(|tb| {
        tb.enumerate(numbers.iter(), |tb, _, item| tb.display(item)?.append('→'))
    });
    assert_eq!(string.as_bytes().len(), 20);
    assert_eq!(string.chars().count(), 10);
    assert_eq!(string.graphemes(true).count(), 10);
    assert_eq!(string, "0→1→2→3→4→");
}

#[test]
fn test_delimit() {
    let numbers = [0, 1, 2, 3, 4];

    let string = TextBuilder::build_string(|tb| {
        tb.delimit(
            |tb| tb.append(','),
            numbers.iter(),
            |tb, _, item| tb.display(item),
        )
    });
    assert_eq!(string.as_bytes().len(), 9);
    assert_eq!(string.chars().count(), 9);
    assert_eq!(string.graphemes(false).count(), 9);
    assert_eq!(string, "0,1,2,3,4");
}

#[test]
fn test_append_delimit() {
    let numbers = [0, 1, 2, 3, 4];

    let string = TextBuilder::build_string(|tb| {
        tb.append_delimit(',', numbers.iter(), |tb, _, item| tb.display(item))
    });
    assert_eq!(string.as_bytes().len(), 9);
    assert_eq!(string.chars().count(), 9);
    assert_eq!(string.graphemes(false).count(), 9);
    assert_eq!(string, "0,1,2,3,4");
}

/*






#[test]
fn test_append_value() {
    let mut tb = TextBuilder::new();
    let value = NonDisplayPoint::default();
    tb.append_value(&value, |pt| format!("({},{})", pt.x, pt.y));

    assert_eq!(tb.len_bytes(), 5);
    assert_eq!(tb.len_chars(), 5);
    assert_eq!(tb.len_graphemes(), 5);
    assert_eq!(tb.to_str(), "(0,0)");
}



#[test]
fn test_enumerate_append() {
    let mut textbuilder = TextBuilder::default();
    let numbers = [0, 1, 2, 3, 4];
    textbuilder.enumerate_append(numbers.iter());
    assert_eq!(textbuilder.len_bytes(), 5);
    assert_eq!(textbuilder.to_string(), "01234")
}



#[test]
fn test_append_delimit() {
    let mut textbuilder = TextBuilder::default();
    let numbers = [0, 1, 2, 3, 4];
    textbuilder.append_delimit(',', numbers.iter(), |tb, i, item| {
        tb.append(i.to_string())
            .append(':')
            .append(item.to_string());
    });
    assert!(textbuilder.len_bytes() > 5);
    assert_eq!(textbuilder.to_string(), "0:0,1:1,2:2,3:3,4:4")
}

#[test]
fn test_append_delimit_append() {
    let mut textbuilder = TextBuilder::default();
    let numbers = [0, 1, 2, 3, 4];
    textbuilder.append_delimit_append(',', numbers.iter());
    assert_eq!(textbuilder.len_bytes(), 9);
    assert_eq!(textbuilder.to_string(), "0,1,2,3,4")
}
*/
