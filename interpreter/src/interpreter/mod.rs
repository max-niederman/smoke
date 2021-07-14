mod error;
mod state;

use crate::parser::ast::Ast;
use crate::parser::ast::{Operation, Operator};
pub use error::{Error, Result};
pub use state::{Value, ValueWrap};
use std::collections::HashMap;

pub struct Interpreter {
    /// Stack of references to values in the value tree
    scopes: Vec<ValueWrap>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            scopes: vec![Value::Scope(HashMap::new(), None).wrapped()],
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
                    .expect("grouping has a tail")?;

                Ok(Value::Scope(
                    self.scopes
                        .pop()
                        .expect("scopes on the stack")
                        .try_unwrap()
                        .expect("no strong references to evaluated scope")
                        .into_scope()
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

            Ast::Function { arguments, body } => {
                Ok(Value::Function(arguments.clone(), *body.clone()).wrapped())
            }

            Ast::FunctionApplication { function, arguments } => {
                let fn_val = self.interpret(function)?;
                let fn_data = fn_val.borrow().clone().into_func().ok_or(Error::TypeError {
                    expected: "function".into(),
                    found: format!("{:?}", function),
                })?;

                let mut interp_args = Vec::new();
                for raw in arguments {
                    interp_args.push(self.interpret(raw)?);
                }

                self.scopes.push(Value::Scope(
                    fn_data.0.clone().into_iter().zip(interp_args).collect(),
                    None
                ).wrapped());
                let returned = self.interpret(&fn_data.1)?;
                self.scopes.pop().expect("scopes on the stack");

                Ok(returned)
            }
        }
    }
}
