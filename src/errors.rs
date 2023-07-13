use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum Errors {
    #[error("Value is Invalid")]
    InvalidValue,
    #[error("Root is not singular")]
    RootNotSingular,
    #[error("EOF is reached")]
    ReachEOF,
}

pub type Result<T> = std::result::Result<T, Errors>;
