mod error;
mod node;
mod token;

use std::convert::TryFrom;

pub use error::*;
pub use node::*;
pub use token::*;

#[derive(Debug)]
pub struct Vdf {
    tokens: Tokens,
    root: Nodes,
}

impl Vdf {
    pub fn new() -> Self {
        Self {
            tokens: Tokens::default(),
            root: Nodes::default(),
        }
    }

    pub fn root(&self) -> &Nodes {
        &self.root
    }

    pub fn tokens(&self) -> &Tokens {
        &self.tokens
    }

    pub fn parse<S: Into<String>>(s: S) -> Result<Self> {
        let tokens = Tokens::parse(s)?;
        let root = Nodes::try_from(&tokens)?;
        Ok(Self { tokens, root })
    }
}
