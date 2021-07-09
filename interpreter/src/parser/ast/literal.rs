use std::convert::TryFrom;
use crate::parser::{Error, Result};
use crate::lexer::Token;

/// A literal Smoke value
#[derive(Debug, Clone)]
pub enum Literal {
    Nil,
    Bool(bool),
    Integer(isize),
    Float(f64),
    Str(String),
}

macro_rules! as_variant_method {
    ($method:ident : $variant:ident => $ret:ty) => {
        pub const fn $method(&self) -> Option<$ret> {
            match *self {
                Self::$variant(val) => Some(val),
                _ => None,
            }
        }
    };
}

macro_rules! variant_method {
    ($method:ident : $variant:pat => $ret:expr) => {

    }
}

impl Literal {
    pub const fn as_nil(&self) -> Option<()> {
        match self {
            Self::Nil => Some(()),
            _ => None,
        }
    }

    as_variant_method!(as_bool: Bool => bool);
    as_variant_method!(as_int: Integer => isize);
    as_variant_method!(as_float: Float => f64);

    variant_method!(to_nil: Self::Nil => ());
    variant_method!(as_float: Self::Float(val) => va);
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
