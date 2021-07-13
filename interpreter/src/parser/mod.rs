pub mod ast;
pub mod error;

use crate::lexer::token::{Token, TokenExt};
use ast::{Ast, Literal, Operation, Operator};
pub use error::{Error, Result};
use std::convert::TryInto;
use std::iter::Peekable;

/// A parsing of a stream of [`TokenExt`]s
pub struct Parser<S: Iterator<Item = TokenExt>> {
    /// The source [`TokenExt`] iterator
    source: Peekable<S>,
}

macro_rules! la_binary {
    ($name:ident, $( $op:pat )|+, $sub:ident) => {
        fn $name(&mut self) -> Result<Ast> {
            let mut expr = self.$sub()?;

            while matches!(
                self.source.peek().map(|tke| tke.token.clone()),
                Some($( $op )|+)
            ) {
                expr = Ast::Operation(
                    Operation::binary(
                        Operator::try_from_token_binary(&self.source.next().unwrap().token)?,
                        (expr, self.$sub()?)
                    )
                );
            }

            Ok(expr)
        }
    }
}

impl<S: Iterator<Item = TokenExt>> Parser<S> {
    pub fn new(source: S) -> Self {
        Self {
            source: source.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Ast> {
        self.expression()
    }

    // Recursive-descent parser

    fn expression(&mut self) -> Result<Ast> {
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

    fn unary(&mut self) -> Result<Ast> {
        if matches!(
            self.source.peek().map(|tke| tke.token.clone()),
            Some(Token::Bang | Token::Minus)
        ) {
            Ok(Ast::Operation(Operation::unary(
                Operator::try_from_token_unary(&self.source.next().unwrap().token)?,
                self.unary()?,
            )))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Ast> {
        macro_rules! literal {
            () => {
                Ok(Ast::Literal(self.source.next().unwrap().token.try_into()?))
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

            Token::Identifier(_) => Ok(Ast::Reference(self.expect_ident()?)),

            Token::ParenLeft => {
                self.source.next();
                let expr = self.expression()?;
                self.expect(|tke| tke.token == Token::ParenRight, "closing delimiter ')")?;

                Ok(Ast::Grouping(vec![expr]))
            }
            Token::CurlyLeft => {
                self.source.next();
                let mut exprs = vec![self.expression().unwrap_or(Ast::Literal(Literal::Nil))];

                while self
                    .expect(|tke| tke.token == Token::Semicolon, "semicolon")
                    .is_ok()
                {
                    exprs.push(self.expression()?);
                }

                self.expect(|tke| tke.token == Token::CurlyRight, "closing delimiter '}")?;

                Ok(Ast::Grouping(exprs))
            }

            Token::Let => {
                self.source.next();

                Ok(Ast::Declaration {
                    name: match self
                        .expect(
                            |tke| matches!(tke.token, Token::Identifier(_)),
                            "identifier",
                        )?
                        .token
                    {
                        Token::Identifier(ident) => ident,
                        _ => panic!(),
                    },
                    value: {
                        self.expect(|tke| tke.token == Token::Equal, "assignment operator")?;
                        Box::new(self.expression()?)
                    },
                })
            }
            Token::Function => {
                self.source.next();

                let name = match self
                    .expect(
                        |tke| matches!(tke.token, Token::Identifier(_)),
                        "function name",
                    )?
                    .token
                {
                    Token::Identifier(val) => val,
                    _ => unreachable!(),
                };

                self.expect(|tke| tke.token == Token::ParenLeft, "opening delimiter '('")?;
                let mut arguments = Vec::new();
                while let Ok(ident) = self.expect_ident() {
                    arguments.push(ident);
                    if self
                        .expect(|tke| tke.token == Token::Comma, "argument seperator ','")
                        .is_err()
                    {
                        break;
                    }
                }
                self.expect(
                    |tke| tke.token == Token::ParenRight,
                    "closing delimiter ')'",
                )?;

                Ok(Ast::Declaration {
                    name,
                    value: Box::new(Ast::Function {
                        arguments,
                        body: Box::new(self.expression()?),
                    }),
                })
            }

            _ => Err(Error::UnexpectedToken {
                expected: "expression".into(),
                found: format!("'{}'", self.source.next().unwrap().lexeme.content),
            }),
        }
    }

    // Helpers

    fn expect<P: FnMut(&TokenExt) -> bool>(&mut self, pred: P, expected: &str) -> Result<TokenExt> {
        self.source.next_if(pred).ok_or(Error::UnexpectedToken {
            expected: expected.into(),
            found: match self.source.peek() {
                Some(tke) => format!("'{}'", tke.lexeme.content),
                None => "end of source".into(),
            },
        })
    }

    fn expect_ident(&mut self) -> Result<String> {
        match self
            .expect(
                |tke| matches!(tke.token, Token::Identifier(_)),
                "identifier",
            )?
            .token
        {
            Token::Identifier(val) => Ok(val),
            _ => unreachable!(),
        }
    }
}
