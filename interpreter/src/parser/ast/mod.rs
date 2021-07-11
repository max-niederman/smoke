pub mod literal;
pub mod operation;

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

    /// A reference to a defined identifier
    Reference(String),

    /// The result of an operation
    Operation(Operation),

    /// A grouping of one or more syntax trees
    Grouping(Vec<Self>),

    /// A function taking arguments and returning a value
    Function {
        arguments: Vec<String>,
        body: Box<Self>,
    },
}
