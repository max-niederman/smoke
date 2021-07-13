pub mod literal;
pub mod operation;

use crate::{extract, extract_variant_method};
pub use literal::Literal;
pub use operation::{Operation, Operator};

/// A Smoke Abstract Syntax Tree
#[derive(Debug, Clone)]
pub enum Ast {
    /// A literal value
    Literal(Literal),

    /// A variable declaration
    Declaration { name: String, value: Box<Self> },

    /// A reference to a defined identifier
    Reference(String),

    /// A grouping of one or more syntax trees
    Grouping(Vec<Self>),

    /// The result of an operation
    Operation(Operation),

    /// A function taking arguments and returning a value
    Function {
        arguments: Vec<String>,
        body: Box<Self>,
    },
}

impl Ast {
    extract_variant_method!(as_literal(&self) { Self::Literal as (a): (&Literal) });
    extract_variant_method!(as_reference(&self) { Self::Reference as (a): (&str) });
    extract_variant_method!(as_operation(&self) { Self::Operation as (a): (&Operation) });
    extract_variant_method!(as_grouping(&self) { Self::Grouping as (a): (&[Self]) });

    extract_variant_method!(into_literal(self) { Self::Literal as (a): (Literal) });
    extract_variant_method!(into_reference(self) { Self::Reference as (a): (String) });
    extract_variant_method!(into_operation(self) { Self::Operation as (a): (Operation) });
    extract_variant_method!(into_grouping(self) { Self::Grouping as (a): (Vec<Self>) });
}
