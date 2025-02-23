use std::{
    io::{BufRead, BufReader, Cursor, Read},
    iter::Peekable,
    str::Chars,
};

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

pub fn from_reader<R: Read>(mut input: R, options: &Options) -> Result<Group> {
    let mut s = String::new();
    input.read_to_string(&mut s).unwrap();
    from_str(&s, options)
}

pub fn from_str(input: &str, options: &Options) -> Result<Group> {
    let mut entries = vec![];
    let mut reader = input.chars().peekable();
    


    Ok(Group { entries })
}

struct Reader<R: Read> {
    reader: R,
    buffer: Vec<u8>,
}

// fn consume_whitespace(reader: &mut Peekable<Chars>) {
//     while let Some(&c) = reader.peek() {
//         if c != ' ' && c != '\t' {
//             break;
//         }
//         reader.next();
//     }
// }

// #[test]
// fn test() {
//     let input = r#"    test  "#;

//     from_str(input, &Options::default()).unwrap();
// }