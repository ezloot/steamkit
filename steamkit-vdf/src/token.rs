use std::iter::Peekable;
use std::str::Chars;

use crate::{Result, Error};

#[derive(Debug, Clone)]
pub enum Token {
    String(String),
    QuotedString(String),
    GroupStart,
    GroupEnd,
    Whitespace(String),
    Comment(String),
    Condition(String),
    NewLine(String),
}

#[derive(Debug, Clone, Default)]
pub struct Tokens {
    pub tokens: Vec<Token>,
}

impl Tokens {
    pub fn parse<S: Into<String>>(s: S) -> Result<Self> {
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
                '[' => Self::use_condition(&mut chars)?,
                '/' => Self::use_comment(&mut chars)?,
                _ => return Err(Error::Syntax(chr)),
            };

            tokens.push(token);
        }

        Ok(Self { tokens })
    }

    fn use_new_line(chars: &mut Peekable<Chars>) -> Result<Token> {
        let mut s = String::new();

        for c in ['\r', '\n'] {
            if matches!(chars.peek(), Some(&chr) if chr == c) {
                s.push(c);
                chars.next();
            }
        }

        Ok(Token::NewLine(s))
    }

    fn use_whitespace(chars: &mut Peekable<Chars>) -> Result<Token> {
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

    fn use_string(chars: &mut Peekable<Chars>) -> Result<Token> {
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

    fn use_comment(chars: &mut Peekable<Chars>) -> Result<Token> {
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

    fn use_quoted_string(chars: &mut Peekable<Chars>) -> Result<Token> {
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
                        let escaped = chars.next().ok_or(Error::UnclosedQuote)?;
                        match escaped {
                            '\\' => s.push('\\'),
                            'n' => s.push('\n'),
                            't' => s.push('\t'),
                            '"' => s.push('"'),
                            chr => return Err(Error::BadEscape(chr)),
                        }
                    }
                    '"' => break,
                    _ => {
                        s.push(chr);
                        chars.next();
                    }
                }
            } else {
                return Err(Error::UnclosedQuote);
            }
        }

        // skip closing quote
        chars.next();

        Ok(Token::QuotedString(s))
    }

    fn use_condition(chars: &mut Peekable<Chars>) -> Result<Token> {
        let mut s = String::new();

        // skip the opening quote
        chars.next();

        loop {
            if let Some(&chr) = chars.peek() {
                if chr == ']' {
                    break;
                }

                s.push(chr);
                chars.next();
            } else {
                return Err(Error::UnclosedCondition);
            }
        }

        // skip closing quote
        chars.next();

        Ok(Token::Condition(s))
    }
}
