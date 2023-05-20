use crate::Token;
#[cfg(feature = "serde")]
use serde::de;


pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unexpected end of file")]
    EndOfFile,
    #[error("bad escape sequence: `{0}`")]
    BadEscape(char),
    #[error("unclosed quote string")]
    UnclosedQuote,
    #[error("unclosed quote string")]
    UnclosedCondition,
    #[error("syntax error caused by: `{0}`")]
    Syntax(char),
    #[error("unexpected token: {0:?}")]
    UnexpectedToken(Token),
    #[error("cannot perform lookup on value nodes")]
    Lookup,
    #[cfg(feature = "serde")]
    #[error("cannot serialize node: `{0}`")]
    Deserialize(String),
}

#[cfg(feature = "serde")]
impl de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Deserialize(msg.to_string())
    }
}
