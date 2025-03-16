use crate::{Group, Result};

#[derive(Debug, Default)]
pub enum DuplicateMode {
    First,
    Last,
    #[default]
    Keep,
}

#[derive(Debug, Default)]
pub struct Options {
    pub types: bool,
    pub escape_sequences: bool,
    pub duplicate_mode: DuplicateMode,
    pub conditionals: Option<Vec<String>>,
}

struct Reader<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> From<&'a str> for Reader<'a> {
    fn from(input: &'a str) -> Self {
        Self {
            bytes: input.as_bytes(),
            position: 0,
        }
    }
}

enum Token<'a> {
    String(&'a str),
    GroupStart(&'a str),
    GroupEnd(&'a str),
    Comment(&'a str),
    Whitespace(&'a str),
}

pub fn from_str(input: &str, _options: &Options) -> Result<Group> {
    let entries = vec![];
    let reader = Reader::from(input);

    Ok(Group { entries })
}
