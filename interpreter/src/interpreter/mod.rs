mod error;
mod state;

pub use error::{Error, Result};
pub use state::Scope;
use crate::parser::ast::{Ast, Literal};

pub struct Interpreter {
    /// Stack of scopes
    scopes: Vec<Scope>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::empty()],
        }
    }

    pub fn interpret(&mut self, ast: Ast) -> Result<Ast> {
        match ast {
            Ast::Literal(_) => Ok(ast),
            _ => unimplemented!(),
        }
    }
}
