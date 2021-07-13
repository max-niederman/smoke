mod error;
mod state;

use crate::parser::ast::Ast;
use crate::parser::ast::{Operation, Operator};
pub use error::{Error, Result};
pub use state::{Value, ValueWrap};
use std::collections::HashMap;

pub struct Interpreter {
    /// Tree of values
    value: ValueWrap,

    /// Stack of references to values in the value tree
    scopes: Vec<ValueWrap>,
}

impl Interpreter {
    pub fn new() -> Self {
        let value = Value::Scope(HashMap::new(), None).wrapped();
        Self {
            scopes: vec![value.clone()],
            value,
        }
    }

    pub fn interpret(&mut self, ast: &Ast) -> Result<ValueWrap> {
        match ast {
            Ast::Literal(lit) => Ok(Value::from(lit.clone()).wrapped()),

            Ast::Declaration { name, value } => {
                let value = self.interpret(value)?;
                self.scopes
                    .last()
                    .unwrap()
                    .borrow_mut()
                    .as_scope_mut()
                    .unwrap()
                    .0
                    .insert(name.clone(), value);
                Ok(Value::Nil.wrapped())
            }

            Ast::Reference(name) => self
                .scopes
                .iter()
                .rev()
                .find_map(|sc| sc.borrow().as_scope().unwrap().0.get(name).cloned())
                .ok_or(Error::ReferenceUndefinedError {
                    name: name.to_string(),
                }),

            Ast::Grouping(children) => {
                self.scopes
                    .push(Value::Scope(HashMap::new(), None).wrapped());
                let tail = children
                    .iter()
                    .map(|child| self.interpret(child))
                    .reduce(Result::and)
                    .expect("grouping had less than one child")?;

                Ok(Value::Scope(
                    self.scopes
                        .pop()
                        .and_then(|sc| sc.try_unwrap().ok())
                        .and_then(Value::into_scope)
                        .unwrap()
                        .0,
                    Some(tail),
                )
                .wrapped())
            }

            Ast::Operation(op) => match op {
                Operation::Unary { operator, operand } => {
                    let operand = self.interpret(operand)?.try_unwrap().unwrap();
                    match operator {
                        Operator::Not => match operand {
                            Value::Bool(opd) => Ok(Value::Bool(!opd)),
                            _ => Err(Error::TypeError {
                                expected: "boolean".into(),
                                found: format!("{:#?}", operand),
                            }),
                        },
                        Operator::Negate => match operand {
                            Value::Integer(num) => Ok(Value::Integer(-num)),
                            Value::Float(num) => Ok(Value::Float(-num)),
                            _ => Err(Error::TypeError {
                                expected: "number (float or integer)".into(),
                                found: format!("{:#?}", operand),
                            }),
                        },
                        _ => unreachable!(),
                    }
                }

                Operation::Binary { operator, operands } => {
                    let operands = (
                        self.interpret(&operands.0)?.try_unwrap().unwrap(),
                        self.interpret(&operands.1)?.try_unwrap().unwrap(),
                    );
                    match operator {
                        Operator::Equal => Ok(Value::Bool(operands.0 == operands.1)),
                        Operator::NotEqual => Ok(Value::Bool(operands.0 != operands.1)),

                        Operator::Greater => Ok(Value::Bool(
                            operands.0.into_number()? > operands.1.into_number()?,
                        )),
                        Operator::GreaterEqual => Ok(Value::Bool(
                            operands.0.into_number()? >= operands.1.into_number()?,
                        )),
                        Operator::Less => Ok(Value::Bool(
                            operands.0.into_number()? < operands.1.into_number()?,
                        )),
                        Operator::LessEqual => Ok(Value::Bool(
                            operands.0.into_number()? <= operands.1.into_number()?,
                        )),

                        Operator::Add => {
                            Ok((operands.0.into_number()? + operands.1.into_number()?).into())
                        }
                        Operator::Subtract => {
                            Ok((operands.0.into_number()? - operands.1.into_number()?).into())
                        }
                        Operator::Multiply => {
                            Ok((operands.0.into_number()? * operands.1.into_number()?).into())
                        }
                        Operator::Divide => {
                            Ok((operands.0.into_number()? / operands.1.into_number()?).into())
                        }

                        _ => unimplemented!(),
                    }
                }
            }
            .map(ValueWrap::wrapping),

            _ => unimplemented!(),
        }
    }
}
