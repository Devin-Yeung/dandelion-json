use crate::data::{Value, ValueType};
use crate::errors::Errors;
use crate::parser::Parser;
use std::fmt::Debug;

fn parse_assert<S, E>(json: S, expect: &E)
where
    S: AsRef<str>,
    E: Eq + Debug,
    std::result::Result<Value, Errors>: PartialEq<E>,
{
    assert_eq!(&Parser::parse(&json), expect)
}

#[test]
fn parse_null() {
    let null = Ok(Value {
        v_type: ValueType::Null,
    });
    parse_assert("null", &null);
    parse_assert(" null", &null);
    parse_assert("\n null", &null);
    parse_assert("\t null", &null);
}

#[test]
fn parse_true() {
    let true_ = Ok(Value {
        v_type: ValueType::Bool(true),
    });
    parse_assert("true", &true_);
    parse_assert(" true", &true_);
    parse_assert("\n true", &true_);
    parse_assert("\t true", &true_);
}

#[test]
fn parse_false() {
    let false_ = Ok(Value {
        v_type: ValueType::Bool(false),
    });
    parse_assert("false", &false_);
    parse_assert(" false", &false_);
    parse_assert("\n false", &false_);
    parse_assert("\t false", &false_);
}
