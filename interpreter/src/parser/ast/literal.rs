use crate::lexer::Token;
use crate::parser::{Error, Result};
use crate::*;
use std::convert::TryFrom;

/// A literal Smoke value
#[derive(Debug, Clone)]
pub enum Literal {
    Nil,
    Bool(bool),
    Integer(isize),
    Float(f64),
    Str(String),
}


impl TryFrom<Token> for Literal {
    type Error = Error;
    fn try_from(token: Token) -> Result<Self> {
        match token {
            Token::Nil => Ok(Self::Nil),
            Token::Bool(val) => Ok(Self::Bool(val)),
            Token::Integer(val) => Ok(Self::Integer(val)),
            Token::Float(val) => Ok(Self::Float(val)),
            Token::Str(val) => Ok(Self::Str(val)),

            _ => Err(Error::Internal("token was not a literal")),
        }
    }
}
