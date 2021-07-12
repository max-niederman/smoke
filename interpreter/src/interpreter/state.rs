use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use crate::parser::ast::Literal;
use crate::*;

/// A program state in a scope
#[derive(Debug, Clone)]
pub enum Scope {
    Branch(HashMap<String, Rc<RefCell<Self>>>),
    Leaf(Rc<RefCell<Value>>),
}

impl Scope {
    extract_variant_method!(as_branch(&self) { Self::Branch as (a): (&HashMap<String, Rc<RefCell<Self>>>) });
    extract_variant_method!(as_leaf(&self) { Self::Leaf as (a): (&Rc<RefCell<Value>>) });
    extract_variant_method!(into_branch(self) { Self::Branch as (a): (HashMap<String, Rc<RefCell<Self>>>) });
    extract_variant_method!(into_leaf(self) { Self::Leaf as (a): (Rc<RefCell<Value>>) });

    pub fn empty() -> Self {
        Self::Branch(HashMap::new())
    }
}

/// A runtime Smoke value
#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Integer(isize),
    Float(f64),
    Str(String),
}

impl Value {
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

impl From<Literal> for Value {
    fn from(lit: Literal) -> Self {
        match lit {
            Literal::Nil => Self::Nil,
            Literal::Bool(val) => Self::Bool(val),
            Literal::Integer(val) => Self::Integer(val),
            Literal::Float(val) => Self::Float(val),
            Literal::Str(val) => Self::Str(val),
        }
    }
}
