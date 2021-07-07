use crate::lexer::token::TokenExt;

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
