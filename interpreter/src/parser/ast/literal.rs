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

impl Literal {
    #[allow(dead_code)]
    pub const fn as_nil(&self) -> Option<()> {
        match self {
            Self::Nil => Some(()),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn into_nil(self) -> Option<()> {
        self.as_nil()
    }

    extract_variant_method!(as_bool(&self) { Self::Bool as (a): (&bool) });
    extract_variant_method!(as_int(&self) { Self::Integer as (a): (&isize) });
    extract_variant_method!(as_float(&self) { Self::Float as (a): (&f64) });
    extract_variant_method!(as_str(&self) { Self::Str as (a): (&str) });

    extract_variant_method!(into_bool(self) { Self::Bool as (a): (bool) });
    extract_variant_method!(into_int(self) { Self::Integer as (a): (isize) });
    extract_variant_method!(into_float(self) { Self::Float as (a): (f64) });
    extract_variant_method!(into_str(self) { Self::Str as (a): (String) });
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
