pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to parse input")]
    Parse,
    #[error("Unexpected input after parsing: {0}")]
    UnexpectedInput(String),
}
