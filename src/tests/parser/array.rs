use crate::arr;
use crate::{invalid_assert, json_assert};

#[test]
fn valid_array() {
    json_assert!("[true]", arr![Bool(true)]);
    json_assert!("[ true]", arr![Bool(true)]);
    json_assert!("[ true ]", arr![Bool(true)]);
    json_assert!("[null,null]", arr!(Null, Null));
    json_assert!("[]", arr!());
    json_assert!("[123]", arr!(Number(123.0)));
    json_assert!("[123 ]", arr!(Number(123.0)));
    json_assert!(
        r#"[ null , false , true , 123 , "abc" ]"#,
        arr!(
            Null,
            Bool(false),
            Bool(true),
            Number(123.0),
            String("abc".to_string())
        )
    );
    json_assert!(
        "[ [ ] , [ 0 ] , [ 0 , 1 ] , [ 0 , 1 , 2 ] ]",
        arr!(
            arr!(),
            arr!(Number(0.0)),
            arr!(Number(0.0), Number(1.0)),
            arr!(Number(0.0), Number(1.0), Number(2.0)),
        )
    )
}

#[test]
fn invalid_array() {
    invalid_assert!("[", MissingCommaOrClosingBracket);
    invalid_assert!("[   ", MissingCommaOrClosingBracket);
    invalid_assert!("[ null  ", MissingCommaOrClosingBracket);
    invalid_assert!("[[] ", MissingCommaOrClosingBracket);
    invalid_assert!("]", InvalidValue);
}
