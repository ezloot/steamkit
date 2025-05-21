/// Reference: https://github.com/ValveSoftware/source-sdk-2013/blob/master/src/tier1/KeyValues.cpp

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

impl<'a> Reader<'a> {
    fn skip_whitespace(&mut self) {
        eprintln!("skip_whitespace {}", self.position);
        while let Some(b' ' | b'\n' | b'\r' | b'\t' | 0x0c | 0x0b) = self.peek() {
            self.position += 1;
        }
    }

    fn skip_comment(&mut self) -> bool {
        eprintln!("skip_comment {}", self.position);
        if self.peek() == Some(b'/') && self.peek_n(1) == Some(b'/') {
            self.position += 2;

            while let Some(v) = self.peek()
                && v != b'\n'
            {
                self.position += 1;
            }

            true
        } else {
            false
        }
    }

    fn read_token(&mut self) -> Option<()> {
        eprintln!("read_token {}", self.position);
        loop {
            self.skip_whitespace();

            // return early since we are at the end of the file
            if self.is_eof() {
                return None;
            }

            // check if we are at a comment
            if !self.skip_comment() {
                break;
            }

            println!("{}", self.position);
        }

        Some(())
    }

    fn peek(&self) -> Option<u8> {
        if self.position < self.bytes.len() {
            Some(self.bytes[self.position])
        } else {
            None
        }
    }

    fn peek_n(&mut self, n: usize) -> Option<u8> {
        if self.position + n < self.bytes.len() {
            Some(self.bytes[self.position + n])
        } else {
            None
        }
    }

    fn is_eof(&self) -> bool {
        self.position >= self.bytes.len()
    }
}

pub fn from_str(input: &str, _options: &Options) -> Result<Group> {
    let entries = vec![];
    let mut reader = Reader::from(input);

    while let Some(token) = reader.read_token() {
        println!("{:?}", token);
    }

    Ok(Group { entries })
}

#[test]
fn test() {
    let input = r#"
        "key" "value"
        "key2" "value2"
        "key3" "value3"
    "#;

    let options = Options::default();
    let result = from_str(input, &options);

    assert!(result.is_ok());
}
