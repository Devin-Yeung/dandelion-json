#[macro_export]
macro_rules! json_assert {
    ($json:expr, $val:expr) => {{
        #[allow(unused_imports)]
        use $crate::data::Value::*;
        assert_eq!($crate::parser::Parser::parse(&$json), Ok($val))
    }};
}

#[macro_export]
macro_rules! invalid_assert {
    ($json:expr, $error:expr) => {{
        use $crate::errors::Errors::*;
        assert_eq!($crate::parser::Parser::parse(&$json), Err($error))
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
        $crate::data::Value::String($str.to_string())
    };
}

#[macro_export]
macro_rules! arr {
    ($($item:expr),* $(,)?) => {{
        use $crate::data::Value::*;
        Array(
            vec![$($item),*]
        )
    }};
}
