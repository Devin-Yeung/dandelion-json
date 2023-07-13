macro_rules! json_assert {
    ($json:expr, $vtype:expr) => {{
        use $crate::data::ValueType::*;
        assert_eq!(
            $crate::parser::Parser::parse(&$json),
            Ok($crate::data::Value { v_type: $vtype })
        )
    }};
}

#[test]
fn parse_null() {
    json_assert!("null", Null);
    json_assert!("null", Null);
    json_assert!(" null", Null);
    json_assert!("\n null", Null);
    json_assert!("\t null", Null);
}

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
