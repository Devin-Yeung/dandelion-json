use crate::{arr, invalid_assert, json_assert, str};
use dandelion_json::data::Value;
use dandelion_json::data::Value::Number;
use std::collections::HashMap;

#[test]
fn valid_object() {
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
    let expected = Value::Object({
        let mut map = HashMap::<String, Value>::new();
        map.insert(String::from("n"), Value::Null);
        map.insert(String::from("f"), Value::Bool(false));
        map.insert(String::from("t"), Value::Bool(true));
        map.insert(String::from("i"), Number(123.0));
        map.insert(String::from("s"), str!("abc"));
        map.insert(
            String::from("a"),
            arr![Number(1.0), Number(2.0), Number(3.0)],
        );
        map.insert(String::from("o"), {
            let mut map = HashMap::<String, Value>::new();
            map.insert(String::from("1"), Number(1.0));
            map.insert(String::from("2"), Number(2.0));
            map.insert(String::from("3"), Number(3.0));
            Value::Object(map)
        });
        map
    });
    json_assert!(json, expected);
    // maybe I should write a macro to deal with this annoying stuff
    // see [dtolnay's solution](https://github.com/serde-rs/json/blob/master/src/macros.rs)
}

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
