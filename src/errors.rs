use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum Errors {
    #[error("Value is Invalid")]
    InvalidValue,
    #[error("Invalid string escape sequence")]
    InvalidStringEscape,
    #[error("Invalid string character")]
    InvalidStringChar,
    #[error("Quotation mark is missing")]
    MissingQuotationMark,
    #[error("Semicolon is missing")]
    MissingSemicolon,
    #[error("Comma or closing bracket is missing")]
    MissingCommaOrClosingBracket,
    #[error("Comma or closing curly bracket is missing")]
    MissingCommaOrClosingCurlyBracket,
    #[error("Object key is missing")]
    MissingKey,
    #[error("Root is not singular")]
    RootNotSingular,
    #[error("EOF is reached")]
    ReachEOF,
    #[error("Number too big")]
    NumberTooBig,
}

pub type Result<T> = std::result::Result<T, Errors>;
