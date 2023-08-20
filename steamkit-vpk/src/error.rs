use nom::error::ParseError;

pub type Result<T, I> = std::result::Result<T, Error<I>>;

#[derive(Debug, thiserror::Error)]
pub enum Error<I> {
    #[error("")]
    Parse(#[from] nom::error::Error<I>),
    #[error("")]
    InvalidSignature(u32),
    #[error("")]
    UnsupportedVersion(u32),
}

impl<I> ParseError<I> for Error<I> {
    fn from_error_kind(input: I, code: nom::error::ErrorKind) -> Self {
        Error::Parse(nom::error::Error { input, code })
    }

    fn append(input: I, kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}
