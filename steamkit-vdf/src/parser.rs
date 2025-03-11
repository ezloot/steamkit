use crate::{Group, Result};

#[derive(Debug, Default)]
pub struct Options {
    pub types: bool,
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
    fn peek(&self) -> Option<u8> {
        if self.position < self.bytes.len() {
            Some(self.bytes[self.position])
        } else {
            None
        }
    }

    fn next(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.position += 1;
        Some(byte)
    }

    fn skip_whitespace(&mut self, newlines: bool) {
        while let Some(byte) = self.peek() {
            match byte {
                b' ' | b'\t' => {
                    self.next();
                }
                b'\r' | b'\n' => {
                    if newlines {
                        self.next();

                        if byte == b'\r' && self.peek() == Some(b'\n') {
                            self.next();
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }
}

pub fn from_str(input: &str, _options: &Options) -> Result<Group> {
    let entries = vec![];
    let reader = Reader::from(input);

    Ok(Group { entries })
}
