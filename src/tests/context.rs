use crate::parser::Context;

#[test]
fn valid_cur() {
    let json = "012345";
    let context = Context::new(json);

    assert_eq!(context.cur(), Some('0'),)
}

#[test]
fn valid_advance_n() {
    let json = "012345";
    let mut context = Context::new(json);
    context.advance_n(3);

    assert_eq!(context.cur(), Some('3'))
}

#[test]
fn valid_advance() {
    let json = "012345";
    let mut context = Context::new(json);
    context.advance();

    assert_eq!(context.cur(), Some('1'))
}

#[test]
fn valid_range() {
    let json = "-Hello";
    let mut context = Context::new(json);
    context.advance();

    assert_eq!(context.peek(5), "Hello")
}

#[test]
fn invalid_range() {
    let json = "abc";
    let context = Context::new(json);
    assert_eq!(context.peek(5), "")
}
