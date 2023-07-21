use crate::{nums, str};
use dandelion_json::data::Value;
use dandelion_json::parser::Parser;

#[test]
fn get_str_index() {
    let value = Parser::parse(r#"{ "name": "John Doe" }"#).unwrap();
    assert_eq!(value.get("name"), Some(&str!("John Doe")));
}

#[test]
fn get_usize_index() {
    let arr = Parser::parse(r#"[1, 2, 3]"#).unwrap();
    assert_eq!(arr.get(0), Some(&Value::Number(1.0)));
}

#[test]
fn get_mut_str_index() {
    let mut obj = Parser::parse(r#"{ "name": "John Doe" }"#).unwrap();
    match obj.get_mut("name") {
        Some(Value::String(s)) => s.clear(),
        _ => {
            unreachable!()
        }
    }
    assert_eq!(obj["name"], Value::String(String::new()))
}

#[test]
fn get_mut_usize_index() {
    let mut arr = Parser::parse(r#"[1, 2, 3]"#).unwrap();
    match arr.get_mut(0) {
        Some(Value::Number(v)) => *v -= 1.0,
        _ => {
            unreachable!()
        }
    }
    assert_eq!(arr, nums!(0, 2, 3));
}
