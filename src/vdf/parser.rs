use std::iter::Peekable;
use std::str::Chars;

use thiserror::Error;

#[derive(Debug)]
pub enum Token {
    String(String),
    QuotedString(String),
    GroupStart,
    GroupEnd,
    Whitespace(String),
    Comment(String),
    Conditional(String),
    NewLine(String),
}

/// Notes:
///
/// The parser will detect line-end sequences and convert them to & from `EndOfLine` tokens.
/// The parser will also detect comments and ignore them but remember the position and contents.

#[derive(Debug)]
pub struct Tokens {
    data: Vec<Token>,
}

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("unexpected end of file")]
    EndOfFile,
    #[error("bad escape sequence: {0}")]
    BadEscape(char),
    #[error("unclosed quote string")]
    UnclosedQuote,
}

impl Tokens {
    pub fn new<S: Into<String>>(s: S) -> Result<Self, TokenError> {
        let bytes = s.into();
        let mut chars = bytes.chars().peekable();
        let mut tokens = vec![];

        while let Some(&chr) = chars.peek() {
            let token = match chr {
                '{' => {
                    chars.next();
                    Token::GroupStart
                }
                '}' => {
                    chars.next();
                    Token::GroupEnd
                }
                '\r' | '\n' => Self::use_new_line(&mut chars)?,
                '\t' | ' ' => Self::use_whitespace(&mut chars)?,
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => Self::use_string(&mut chars)?,
                '"' => Self::use_quoted_string(&mut chars)?,
                '/' => Self::use_comment(&mut chars)?,
                _ => panic!("test: {:?}", chr),
            };

            tokens.push(token);
        }

        println!("tokens: {:?}", tokens);

        Ok(Self { data: vec![] })
    }

    fn use_new_line(chars: &mut Peekable<Chars>) -> Result<Token, TokenError> {
        let mut s = String::new();

        for c in ['\r', '\n'] {
            if matches!(chars.peek(), Some(&chr) if chr == c) {
                s.push(c);
                chars.next();
            }
        }

        Ok(Token::NewLine(s))
    }

    fn use_whitespace(chars: &mut Peekable<Chars>) -> Result<Token, TokenError> {
        let mut s = String::new();

        while let Some(&chr) = chars.peek() {
            if chr == '\t' || chr == ' ' {
                s.push(chr);
                chars.next();
            } else {
                break;
            }
        }

        Ok(Token::Whitespace(s))
    }

    pub fn use_string(chars: &mut Peekable<Chars>) -> Result<Token, TokenError> {
        let mut s = String::new();

        while let Some(&chr) = chars.peek() {
            match chr {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => {
                    s.push(chr);
                    chars.next();
                }
                _ => break,
            }
        }

        Ok(Token::String(s))
    }

    pub fn use_comment(chars: &mut Peekable<Chars>) -> Result<Token, TokenError> {
        let mut s = String::new();

        while let Some(&chr) = chars.peek() {
            if chr == '\n' {
                if s.ends_with('\r') {
                    s.pop();
                }
                break;
            }

            s.push(chr);
            chars.next();
        }

        Ok(Token::Comment(s))
    }

    pub fn use_quoted_string(chars: &mut Peekable<Chars>) -> Result<Token, TokenError> {
        let mut s = String::new();

        // skip the opening quote
        chars.next();

        loop {
            if let Some(&chr) = chars.peek() {
                match chr {
                    '\\' => {
                        // skip escape prefix
                        chars.next();

                        // get escaped char
                        let escaped = chars.next().ok_or(TokenError::UnclosedQuote)?;
                        match escaped {
                            '\\' => s.push('\\'),
                            'n' => s.push('\n'),
                            't' => s.push('\t'),
                            '"' => s.push('"'),
                            chr => return Err(TokenError::BadEscape(chr)),
                        }
                    }
                    '"' => break,
                    _ => {
                        s.push(chr);
                        chars.next();
                    }
                }
            } else {
                return Err(TokenError::UnclosedQuote);
            }
        }

        // skip closing quote
        chars.next();

        Ok(Token::QuotedString(s))
    }
}

struct KeyValues {
    tokens: Tokens,
}

impl KeyValues {}
