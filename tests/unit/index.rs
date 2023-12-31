use crate::{nums, str};
use dandelion_json::data::Value;
use dandelion_json::parser::Parser;

#[test]
fn immutable_index() {
    let json = r#"
        {
            "n" : null ,
            "f" : false ,
            "t" : true ,
            "i" : 123 ,
            "s" : "abc",
            "a" : [ 1, 2, 3 ],
            "o" : { "1" : 1, "2" : 2, "3" : 3 }
        }
        "#;
    let value = Parser::parse(json).unwrap();
    assert_eq!(value["n"], Value::Null);
    assert_eq!(value["f"], Value::Bool(false));
    assert_eq!(value["t"], Value::Bool(true));
    assert_eq!(value["i"], Value::Number(123.0));
    assert_eq!(value["s"], str!("abc"));
    assert_eq!(value["a"], nums!(1, 2, 3));
    let object = &value["o"];
    assert_eq!(object[String::from("1")], Value::Number(1.0));
    assert_eq!(object[String::from("2")], Value::Number(2.0));
    assert_eq!(object[String::from("3")], Value::Number(3.0));
}

#[test]
fn mutable_index() {
    let mut value = Parser::parse("{}").unwrap();
    value["foo"] = Value::Bool(true);
    value[String::from("bar")] = Value::Bool(false);
    assert_eq!(value["foo"], Value::Bool(true));
    assert_eq!(value["bar"], Value::Bool(false));
}

#[test]
fn out_of_bound_null() {
    let value = Parser::parse("[0, 1, 2]").unwrap();
    assert_eq!(&value[3], &Value::Null);
}

#[test]
fn empty_null() {
    let mut value = Value::Null;
    value["key"] = Value::Bool(true);
    assert_eq!(value["key"], Value::Bool(true));
}

#[test]
#[should_panic]
fn out_of_bound_panic() {
    let mut value = Parser::parse("[0, 1, 2]").unwrap();
    value[3] = Value::Number(3.0);
}

#[test]
#[should_panic]
fn array_cant_be_changed_by_str_index() {
    let mut value = Parser::parse("[0, 1, 2]").unwrap();
    value["foo"] = Value::Number(3.0);
}
