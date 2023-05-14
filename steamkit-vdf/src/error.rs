use crate::Token;

pub type Result<T> = std::result::Result<T, Error>;


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unexpected end of file")]
    EndOfFile,
    #[error("bad escape sequence: {0}")]
    BadEscape(char),
    #[error("unclosed quote string")]
    UnclosedQuote,
    #[error("unclosed quote string")]
    UnclosedCondition,
    #[error("syntax error caused by: {0}")]
    Syntax(char),
    #[error("unexpected token")]
    UnexpectedToken(Token),
    #[error("cannot perform lookup on value nodes")]
    Lookup,
}
