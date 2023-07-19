use crate::invalid_assert;

#[test]
fn key_is_missing() {
    invalid_assert!("{:1,", MissingKey);
    invalid_assert!("{1:1,", MissingKey);
    invalid_assert!("{true:1,", MissingKey);
    invalid_assert!("{false:1,", MissingKey);
    invalid_assert!("{null:1,", MissingKey);
    invalid_assert!("{[]:1,", MissingKey);
    invalid_assert!("{{}:1,", MissingKey);
    invalid_assert!(r#"{"a":1,"#, MissingKey);
}

#[test]
fn semicolon_is_missing() {
    invalid_assert!(r#"{"a"}"#, MissingSemicolon);
    invalid_assert!(r#"{"a","b"}"#, MissingSemicolon);
}

#[test]
fn curly_bracket_is_missing() {
    invalid_assert!(r#"{"a":1"#, MissingCommaOrClosingCurlyBracket);
    invalid_assert!(r#"{"a":1]"#, MissingCommaOrClosingCurlyBracket);
    invalid_assert!(r#"{"a":1 "b""#, MissingCommaOrClosingCurlyBracket);
    invalid_assert!(r#"{"a":{}"#, MissingCommaOrClosingCurlyBracket);
}
