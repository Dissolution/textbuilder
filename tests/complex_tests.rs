use textbuilder::TextBuilder;

#[test]
pub fn complex_test_a() {
    let string = TextBuilder::build_string(|tb| {
        tb.append("pub struct TestStruct {")?
            .indented("    ", |tb| {
                tb.newline()?
                    .append("pub name: &'static str,")?
                    .newline()?
                    .append("pub value: usize,")
            })?
            .newline()?
            .append('}')?
            .newline()
    });

    let check = r#"pub struct TestStruct {
    pub name: &'static str,
    pub value: usize,
}
"#;

    assert_eq!(string, check);
}
