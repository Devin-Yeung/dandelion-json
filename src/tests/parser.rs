use crate::data::{Value, ValueType};
use crate::parser::Parser;

#[test]
fn parse_null() {
    let data = vec!["null", " null", "\t null", "\n null"];

    data.into_iter().for_each(|json| {
        assert_eq!(
            Parser::parse(json),
            Ok(Value {
                v_type: ValueType::Null,
            })
        )
    })
}

#[test]
fn parse_true() {
    let data = vec!["true", " true", "\t true", "\n true"];

    data.into_iter().for_each(|json| {
        assert_eq!(
            Parser::parse(json),
            Ok(Value {
                v_type: ValueType::Bool(true),
            })
        )
    })
}

#[test]
fn parse_false() {
    let data = vec!["false", " false", "\t false", "\n false"];

    data.into_iter().for_each(|json| {
        assert_eq!(
            Parser::parse(json),
            Ok(Value {
                v_type: ValueType::Bool(false),
            })
        )
    })
}
