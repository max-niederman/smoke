use crate::lexer::token::TokenExt;
use std::fmt;

/// A Smoke expression
#[derive(Debug, Clone)]
pub enum Expression {
    /// A grouping of expressions
    Grouping(Vec<Self>),
    /// A literal value
    Literal(TokenExt),
    /// A unary expression
    Unary(TokenExt, Box<Self>),
    /// A binary expression
    Binary(TokenExt, Box<(Self, Self)>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Literal(tke) => write!(f, "{}", tke.lexeme.content),
            Self::Unary(opr, opd) => write!(f, "({} {})", opr.lexeme.content, opd),
            Self::Binary(opr, opd) => write!(f, "({} {} {})", opr.lexeme.content, opd.0, opd.1),
            Self::Grouping(children) => {
                write!(f, "({}", children.first().unwrap())?;
                for expr in children.iter().skip(1) {
                    write!(f, " {}", expr)?;
                }
                write!(f, ")")
            }
        }
    }
}
