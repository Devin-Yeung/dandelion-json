use crate::{invalid_assert, json_assert, quote, str};

#[test]
fn parse_string() {
    json_assert!(quote!("Hello"), str!("Hello"));
    json_assert!(
        quote!(r#"Hello\nWorld"#),
        String("Hello\nWorld".to_string())
    );
    /* special char */
    json_assert!(quote!(r#"\""#), str!("\""));
    json_assert!(quote!(r#"\\"#), str!("\\"));
    json_assert!(quote!(r#"\/"#), str!("/"));
    json_assert!(quote!(r#"\n"#), str!("\n"));
    json_assert!(quote!(r#"\r"#), str!("\r"));
    json_assert!(quote!(r#"\t"#), str!("\t"));
    json_assert!(quote!(r#"\b"#), str!("\x08"));
    json_assert!(quote!(r#"\f"#), str!("\x0C"));
    json_assert!(quote!(r#"\"\\\/\n\r\t\b\f"#), str!("\"\\/\n\r\t\x08\x0C"))
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
