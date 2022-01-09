use std::fmt::Display;

use serde::{de, ser};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unsupported type: {0}")]
    UnsupportedType(&'static str),
    #[error("{0}")]
    Message(String),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}
