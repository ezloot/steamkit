use std::iter::Peekable;
use std::str::Chars;

use indexmap::IndexMap;

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
    pub tokens: Vec<Token>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unexpected end of file")]
    EndOfFile,
    #[error("bad escape sequence: {0}")]
    BadEscape(char),
    #[error("unclosed quote string")]
    UnclosedQuote,
    #[error("syntax error caused by: {0}")]
    Syntax(char),
}

impl Tokens {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn parse<S: Into<String>>(s: S) -> Result<Self, Error> {
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
                _ => return Err(Error::Syntax(chr)),
            };

            tokens.push(token);
        }

        Ok(Self { tokens })
    }

    fn use_new_line(chars: &mut Peekable<Chars>) -> Result<Token, Error> {
        let mut s = String::new();

        for c in ['\r', '\n'] {
            if matches!(chars.peek(), Some(&chr) if chr == c) {
                s.push(c);
                chars.next();
            }
        }

        Ok(Token::NewLine(s))
    }

    fn use_whitespace(chars: &mut Peekable<Chars>) -> Result<Token, Error> {
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

    fn use_string(chars: &mut Peekable<Chars>) -> Result<Token, Error> {
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

    fn use_comment(chars: &mut Peekable<Chars>) -> Result<Token, Error> {
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

    fn use_quoted_string(chars: &mut Peekable<Chars>) -> Result<Token, Error> {
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
}

#[derive(Debug)]
enum Node {
    Entry {
        key: Token,
        value: Token,
        conditional: Option<Token>,
    },
    Array(Vec<Node>),
    Map(IndexMap<String, Node>)
}

impl Node {
    pub fn clear(&mut self) {
        match self {
            Node::Array(nodes) => nodes.clear(),
            Node::Map(nodes) => nodes.clear(),
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct KeyValues {
    tokens: Tokens,
    root: Node,
}

impl KeyValues {
    pub fn new() -> Self {
        Self {
            tokens: Tokens::new(),
            root: Node::Map(IndexMap::new()),
        }
    }

    pub fn parse<S: Into<String>>(s: S) -> Result<Self, Error> {
        let tokens = Tokens::parse(s)?;
        let root = Node::Map(IndexMap::new());
        let mut kv = Self { tokens, root };
        kv.decode_tokens()?;
        Ok(kv)
    }

    pub fn decode_tokens(&mut self) -> Result<(), Error> {
        self.root.clear();

        Ok(())
    }
}
