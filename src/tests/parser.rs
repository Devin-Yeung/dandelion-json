macro_rules! json_assert {
    ($json:expr, $val:expr) => {{
        #[allow(unused_imports)]
        use $crate::data::Value::*;
        assert_eq!($crate::parser::Parser::parse(&$json), Ok($val))
    }};
}

macro_rules! invalid_assert {
    ($json:expr, $error:expr) => {{
        use $crate::errors::Errors::*;
        assert_eq!($crate::parser::Parser::parse(&$json), Err($error))
    }};
}

#[test]
fn not_singular_root() {
    invalid_assert!("null x", RootNotSingular);
    invalid_assert!("1u10", RootNotSingular); /* bad exp field */
    invalid_assert!("0123", RootNotSingular); /* after zero should be '.' or nothing */
    invalid_assert!("0x0", RootNotSingular); /* after zero should be '.' or nothing */
    invalid_assert!("0x123", RootNotSingular); /* after zero should be '.' or nothing */
    invalid_assert!("001", RootNotSingular); /* after zero should be '.' or nothing */
    invalid_assert!("00.1", RootNotSingular); /* after zero should be '.' or nothing */
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

#[test]
fn simple_number() {
    json_assert!("0", Number(0.0));
    json_assert!("-0", Number(0.0));
    json_assert!("-0.0", Number(0.0));
    json_assert!("1", Number(1.0));
    json_assert!("-1", Number(-1.0));
    json_assert!("1.5", Number(1.5));
    json_assert!("-1.5", Number(-1.5));
    json_assert!("3.1416", Number(3.1416));
}

#[test]
fn parse_num_with_exp() {
    json_assert!("1E10", Number(1E10));
    json_assert!("1e10", Number(1e10));
    json_assert!("1E+10", Number(1E+10));
    json_assert!("1E-10", Number(1E-10));
    json_assert!("-1E10", Number(-1E10));
    json_assert!("-1e10", Number(-1e10));
    json_assert!("-1E+10", Number(-1E+10));
    json_assert!("-1E-10", Number(-1E-10));
    json_assert!("1.234E+10", Number(1.234E+10));
    json_assert!("1.234E-10", Number(1.234E-10));
}

#[test]
fn parse_num_edge_case() {
    json_assert!("1e-10000", Number(0.0)); /* must underflow */
    json_assert!("1.0000000000000002", Number(1.0000000000000002)); /* the smallest number > 1 */
    json_assert!("4.9406564584124654e-324", Number(4.9406564584124654e-324)); /* minimum denormal */
    json_assert!("-4.9406564584124654e-324", Number(-4.9406564584124654e-324));
    json_assert!("2.2250738585072009e-308", Number(2.2250738585072009e-308)); /* Max subnormal double */
    json_assert!("-2.2250738585072009e-308", Number(-2.2250738585072009e-308));
    json_assert!("2.2250738585072014e-308", Number(2.2250738585072014e-308)); /* Min normal positive double */
    json_assert!("-2.2250738585072014e-308", Number(-2.2250738585072014e-308));
    json_assert!("1.7976931348623157e+308", Number(1.7976931348623157e+308)); /* Max double */
    json_assert!("-1.7976931348623157e+308", Number(-1.7976931348623157e+308));
}

#[test]
fn parse_invalid_num() {
    /* invalid number */
    invalid_assert!("+0", InvalidValue);
    invalid_assert!("+1", InvalidValue);
    invalid_assert!(".123", InvalidValue); /* at least one digit before '.' */
    invalid_assert!("1.", InvalidValue); /* at least one digit after '.' */
    invalid_assert!("0.", InvalidValue); /* at least one digit after '.' */
    invalid_assert!("INF", InvalidValue);
    invalid_assert!("inf", InvalidValue);
    invalid_assert!("NAN", InvalidValue);
    invalid_assert!("nan", InvalidValue);
    invalid_assert!("-", InvalidValue);
    invalid_assert!("1ee", InvalidValue); /* bad exp field */
    invalid_assert!("1e", InvalidValue); /* bad exp field */
}

#[test]
fn parse_large_num() {
    invalid_assert!("1e309", NumberTooBig);
    // invalid_assert!("1e-618", NumberTooBig); // this should underflow
}

macro_rules! quote {
    ($str:expr) => {
        format!("\"{}\"", $str)
    };
}

#[test]
fn parse_string() {
    json_assert!(quote!("Hello"), String("Hello".to_string()));
    json_assert!(
        quote!(r#"Hello\nWorld"#),
        String("Hello\nWorld".to_string())
    );
    /* special char */
    json_assert!(quote!(r#"\""#), String("\"".to_string()));
    json_assert!(quote!(r#"\\"#), String("\\".to_string()));
    json_assert!(quote!(r#"\/"#), String("/".to_string()));
    json_assert!(quote!(r#"\n"#), String("\n".to_string()));
    json_assert!(quote!(r#"\r"#), String("\r".to_string()));
    json_assert!(quote!(r#"\t"#), String("\t".to_string()));
    json_assert!(quote!(r#"\b"#), String("\x08".to_string()));
    json_assert!(quote!(r#"\f"#), String("\x0C".to_string()));
    json_assert!(
        quote!(r#"\"\\\/\n\r\t\b\f"#),
        String("\"\\/\n\r\t\x08\x0C".to_string())
    )
}

#[test]
fn invalid_string() {
    invalid_assert!(quote!(r#"\v"#), InvalidStringEscape);
    invalid_assert!(quote!(r#"\'"#), InvalidStringEscape);
    invalid_assert!(quote!(r#"\0"#), InvalidStringEscape);
    invalid_assert!(quote!("\x12"), InvalidStringChar);
    invalid_assert!(r#""\"#, MissingQuotationMark); // "\
    invalid_assert!(r#"""#, MissingQuotationMark); // "
}

macro_rules! arr {
    ($($item:expr),* $(,)?) => {{
        use $crate::data::Value::*;
        Array(
            vec![$($item),*]
        )
    }};
}

#[test]
fn valid_array() {
    json_assert!("[true]", arr![Bool(true)]);
    json_assert!("[ true]", arr![Bool(true)]);
    json_assert!("[ true ]", arr![Bool(true)]);
    json_assert!("[null,null]", arr!(Null, Null));
}
