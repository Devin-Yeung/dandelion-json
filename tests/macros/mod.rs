#[macro_export]
macro_rules! json_assert {
    ($json:expr, $val:expr) => {{
        #[allow(unused_imports)]
        use dandelion_json::data::Value::*;
        assert_eq!(::dandelion_json::parser::Parser::parse(&$json), Ok($val))
    }};
}

#[macro_export]
macro_rules! invalid_assert {
    ($json:expr, $error:expr) => {{
        use dandelion_json::errors::Errors::*;
        assert_eq!(::dandelion_json::parser::Parser::parse(&$json), Err($error))
    }};
}

#[macro_export]
macro_rules! quote {
    ($str:expr) => {
        format!("\"{}\"", $str)
    };
}

#[macro_export]
macro_rules! str {
    ($str:expr) => {
        ::dandelion_json::data::Value::String($str.to_string())
    };
}

#[macro_export]
macro_rules! arr {
    ($($item:expr),* $(,)?) => {{
        use ::dandelion_json::data::Value::*;
        Array(
            vec![$($item),*]
        )
    }};
}

#[macro_export]
macro_rules! nums {
    ($($item:expr),* $(,)?) => {{
        use ::dandelion_json::data::Value::*;
        Array(
            vec![$(Number($item.into())),*]
        )
    }};
}
