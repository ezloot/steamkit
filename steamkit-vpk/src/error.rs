use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("file error")]
    File(#[from] io::Error),
    #[error("failed to parse")]
    Parse,
    #[error("invalid archive name")]
    InvalidName,
}
