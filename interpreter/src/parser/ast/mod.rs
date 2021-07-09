pub mod literal;
pub mod operation;

use crate::lexer::token::TokenExt;
pub use literal::Literal;
pub use operation::{Operation, Operator};
use std::fmt;

/// A Smoke Abstract Syntax Tree
#[derive(Debug, Clone)]
pub enum Ast {
    /// A variable declaration
    Declaration { name: String, value: Box<Self> },

    /// A literal value
    Literal(Literal),

    /// The result of an operation
    Operation(Operation),

    /// A grouping of one or more syntax trees
    Grouping(Vec<Self>),
}
