use crate::json_assert;

#[test]
fn parse_true() {
    json_assert!("true", Bool(true));
    json_assert!("true", Bool(true));
    json_assert!(" true", Bool(true));
    json_assert!("\n true", Bool(true));
    json_assert!("\t true", Bool(true));
}

#[test]
fn parse_false() {
    json_assert!("false", Bool(false));
    json_assert!("false", Bool(false));
    json_assert!(" false", Bool(false));
    json_assert!("\n false", Bool(false));
    json_assert!("\t false", Bool(false));
}
