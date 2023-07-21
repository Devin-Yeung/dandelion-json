use crate::json_assert;

#[test]
fn parse_null() {
    json_assert!("null", Null);
    json_assert!("null", Null);
    json_assert!(" null", Null);
    json_assert!("\n null", Null);
    json_assert!("\t null", Null);
}
