use super::Ast;
use crate::lexer::Token;
use crate::parser::{Error, Result};
use std::convert::TryFrom;

/// An operation augmenting one or more expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Unary {
        operator: Operator,
        operand: Box<Ast>,
    },
    Binary {
        operator: Operator,
        operands: Box<(Ast, Ast)>,
    },
}

impl Operation {
    pub fn unary(operator: Operator, operand: Ast) -> Self {
        Self::Unary {
            operand: Box::new(operand),
            operator,
        }
    }

    pub fn binary(operator: Operator, operands: (Ast, Ast)) -> Self {
        Self::Binary {
            operands: Box::new(operands),
            operator,
        }
    }
}

/// Enumeration of builtin operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    // Unaries
    Not,
    Negate,

    // Binaries
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl TryFrom<&Token> for Operator {
    type Error = crate::parser::Error;
    fn try_from(token: &Token) -> Result<Self> {
        match token {
            Token::Bang => Ok(Self::Not),

            Token::EqualEqual => Ok(Self::Equal),
            Token::BangEqual => Ok(Self::NotEqual),
            Token::Greater => Ok(Self::Greater),
            Token::GreaterEqual => Ok(Self::GreaterEqual),
            Token::Less => Ok(Self::Less),
            Token::LessEqual => Ok(Self::LessEqual),
            Token::Plus => Ok(Self::Add),
            Token::Star => Ok(Self::Multiply),
            Token::Slash => Ok(Self::Divide),

            // Ambiguous tokens
            Token::Minus => Err(Error::Internal("token was ambiguous")),

            _ => Err(Error::Internal("token did not correspond to any operator")),
        }
    }
}

impl Operator {
    pub fn try_from_token_unary(token: &Token) -> Result<Self> {
        match token {
            Token::Minus => Ok(Self::Negate),
            _ => TryFrom::try_from(token),
        }
    }

    pub fn try_from_token_binary(token: &Token) -> Result<Self> {
        match token {
            Token::Minus => Ok(Self::Subtract),
            _ => TryFrom::try_from(token),
        }
    }
}
