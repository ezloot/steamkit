use std::{collections::HashMap, convert::TryFrom, iter::Peekable, slice::Iter};

use crate::{Error, Result, Token, Tokens};

#[derive(Debug, Clone)]
pub struct Node {
    pub key: String,
    pub value: NodeValue,
    pub condition: Option<String>,
}

#[derive(Debug, Clone)]
pub enum NodeValue {
    Map(Nodes),
    String(String),
}

impl NodeValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Map(_) => None,
            Self::String(s) => Some(&s),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Nodes {
    map: HashMap<String, Node>,
}

impl TryFrom<&Tokens> for Nodes {
    type Error = Error;

    fn try_from(tokens: &Tokens) -> std::result::Result<Self, Self::Error> {
        let mut tokens = tokens.tokens.iter().peekable();
        Self::read_root(&mut tokens)
    }
}

impl Nodes {
    pub fn lookup(&self, full_path: &str) -> Result<Option<&Node>> {
        let (path, rest) = full_path.split_once(".").unwrap_or((full_path, ""));
        let Some(node) =self.map.get(path) else {
            return Ok(None);
        };

        if rest != "" {
            match &node.value {
                NodeValue::Map(nodes) => nodes.lookup(rest),
                _ => Err(Error::Lookup),
            }
        } else {
            Ok(Some(node))
        }
    }

    fn read_root(tokens: &mut Peekable<Iter<Token>>) -> Result<Nodes> {
        let mut map = HashMap::new();

        loop {
            Self::read_comment(tokens);

            if tokens.peek().is_none() {
                break;
            }

            let node = Self::read_node(tokens)?;
            map.insert(node.key.clone(), node);
        }

        Ok(Nodes { map })
    }

    fn read_whitespace(tokens: &mut Peekable<Iter<Token>>, optional: bool) -> Result<bool> {
        let token = tokens.peek();
        let res = match token {
            None => Err(Error::EndOfFile),
            Some(Token::Whitespace(_)) => {
                tokens.next();
                Ok(true)
            }
            Some(&token) => Err(Error::UnexpectedToken(token.to_owned())),
        };

        if optional && res.is_err() {
            Ok(false)
        } else {
            res
        }
    }

    fn read_eol(tokens: &mut Peekable<Iter<Token>>) -> Result<()> {
        let mut token: Option<&&Token>;
        loop {
            token = tokens.peek();
            match token {
                Some(Token::Comment(_)) | Some(Token::Whitespace(_)) => {
                    tokens.next();
                    continue;
                }
                Some(Token::NewLine(_)) | None => {
                    tokens.next();
                    break Ok(());
                }
                Some(&token) => break Err(Error::UnexpectedToken(token.to_owned())),
            }
        }
    }

    fn read_string(tokens: &mut Peekable<Iter<Token>>) -> Result<String> {
        match tokens.peek() {
            Some(Token::QuotedString(s)) | Some(Token::String(s)) => {
                tokens.next();
                Ok(s.to_owned())
            }
            Some(&token) => Err(Error::UnexpectedToken(token.to_owned())),
            None => Err(Error::EndOfFile),
        }
    }

    fn read_token<'a>(tokens: &'a mut Peekable<Iter<Token>>) -> Result<&'a Token> {
        match tokens.next() {
            Some(token) => Ok(token),
            None => Err(Error::EndOfFile),
        }
    }

    fn read_group(tokens: &mut Peekable<Iter<Token>>) -> Result<Nodes> {
        Self::read_whitespace(tokens, true)?;

        // get start of group
        match Self::read_token(tokens)? {
            Token::GroupStart => {}
            token => return Err(Error::UnexpectedToken(token.to_owned())),
        }

        let mut map = HashMap::new();

        loop {
            Self::read_comment(tokens);

            let token = tokens.peek();
            if let Some(Token::GroupEnd) = token {
                tokens.next();
                Self::read_eol(tokens)?;
                break;
            }

            let node = Self::read_node(tokens)?;
            map.insert(node.key.clone(), node);
        }

        Ok(Nodes { map })
    }

    fn read_comment(tokens: &mut Peekable<Iter<Token>>) {
        loop {
            match tokens.peek() {
                Some(Token::Whitespace(_)) | Some(Token::Comment(_)) | Some(Token::NewLine(_)) => {
                    tokens.next();
                    continue;
                }
                Some(_) | None => break,
            }
        }
    }

    fn read_node(tokens: &mut Peekable<Iter<Token>>) -> Result<Node> {
        Self::read_comment(tokens);

        let key = Self::read_string(tokens)?;
        let mut value = None;
        let mut condition = None;

        if Self::read_whitespace(tokens, true)? {
            value = Self::read_string(tokens).map(|s| NodeValue::String(s)).ok();
        }

        // get final value
        let value = match value {
            Some(value) => {
                if Self::read_whitespace(tokens, true)? {
                    if let Some(Token::Condition(cond)) = tokens.peek() {
                        tokens.next();
                        condition = Some(cond.to_owned());
                    }
                }

                Self::read_eol(tokens)?;
                value
            }
            None => {
                Self::read_eol(tokens)?;
                NodeValue::Map(Self::read_group(tokens)?)
            }
        };

        Ok(Node {
            key,
            value,
            condition,
        })
    }
}
