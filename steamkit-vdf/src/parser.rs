#[cfg(all(feature = "regex", feature = "regex-lite"))]
compile_error!("Features 'regex' and 'regex-lite' cannot be enabled at the same time.");
#[cfg(feature = "regex")]
use regex::Regex;
#[cfg(feature = "regex-lite")]
use regex_lite::Regex;

use crate::{Group, Result};

thread_local! {
    static INT_REGEX: Regex = Regex::new(r"^\-?\d+$").unwrap();
    static FLOAT_REGEX: Regex = Regex::new(r"\-?\d+\.\d+$").unwrap();
    static BOOL_REGEX: Regex = Regex::new(r"(?i)^(true|false)$").unwrap();
}

#[derive(Debug, Default)]
pub struct Options {
    pub types: bool,
    pub conditionals: Option<Vec<String>>,
}

pub fn from_str(_input: &str, _options: &Options) -> Result<Group> {
    let entries = vec![];
    // let mut _reader = input.chars().peekable();

    Ok(Group { entries })
}
