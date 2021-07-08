pub mod error;
pub mod expr;

use crate::lexer::token::{Token, TokenExt};
pub use error::{Error, Result};
use expr::Expression;
use std::iter::Peekable;

/// A parsing of a stream of [`TokenExt`]s
pub struct Parsing<S: Iterator<Item = TokenExt>> {
    /// The source [`TokenExt`] iterator
    source: Peekable<S>,
}

macro_rules! la_binary {
    ($name:ident, $( $op:pat )|+, $sub:ident) => {
        fn $name(&mut self) -> Result<Expression> {
            let mut expr = self.$sub()?;

            while matches!(
                self.source.peek().map(|tke| tke.token.clone()),
                Some($( $op )|+)
            ) {
                expr = Expression::Binary(
                    self.source.next().unwrap(),
                    Box::new((expr, self.$sub()?)),
                );
            }

            Ok(expr)
        }
    }
}

impl<S: Iterator<Item = TokenExt>> Parsing<S> {
    pub fn new(source: S) -> Self {
        Self {
            source: source.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Expression> {
        self.expression()
    }

    // Recursive-descent parser

    fn expression(&mut self) -> Result<Expression> {
        self.equality()
    }

    la_binary!(equality, Token::EqualEqual | Token::BangEqual, comparison);
    la_binary!(
        comparison,
        Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual,
        term
    );
    la_binary!(term, Token::Plus | Token::Minus, factor);
    la_binary!(factor, Token::Star | Token::Slash, unary);

    fn unary(&mut self) -> Result<Expression> {
        if matches!(
            self.source.peek().map(|tke| tke.token.clone()),
            Some(Token::Bang | Token::Minus)
        ) {
            Ok(Expression::Unary(
                self.source.next().unwrap(),
                Box::new(self.unary()?),
            ))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expression> {
        macro_rules! literal {
            () => {
                Ok(Expression::Literal(self.source.next().unwrap()))
            };
        }

        match self
            .source
            .peek()
            .ok_or(Error::UnexpectedToken {
                expected: "literal or grouping".into(),
                found: "end of source".into(),
            })?
            .token
        {
            Token::Nil => literal!(),
            Token::Bool(_) => literal!(),
            Token::Str(_) => literal!(),
            Token::Integer(_) => literal!(),
            Token::Float(_) => literal!(),

            Token::ParenLeft => {
                self.source.next();
                let expr = self.expression()?;

                if self.source.peek().map(|tke| tke.token.clone()) == Some(Token::ParenRight) {
                    self.source.next();
                    Ok(Expression::Grouping(vec![expr]))
                } else {
                    Err(Error::UnexpectedToken {
                        expected: "closing delimiter ')'".into(),
                        found: match self.source.next() {
                            Some(tke) => format!("'{}'", tke.lexeme.content),
                            None => "end of source".into(),
                        },
                    })
                }
            }

            _ => Err(Error::UnexpectedToken {
                expected: "literal or grouping".into(),
                found: format!("lexeme '{}'", self.source.next().unwrap().lexeme.content),
            }),
        }
    }
}
