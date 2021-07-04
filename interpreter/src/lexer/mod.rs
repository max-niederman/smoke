pub mod error;
pub mod parse;
pub mod token;

use crate::utils::{history_iter::HistoryIter, prelude::*};
use error::*;
use parse::Parse;
use std::sync::RwLock;
use token::{lexeme::{Lexeme, LexemeLocation}, Token, TokenExt};

/// A lexical analysis
#[derive(Debug)]
pub struct Analysis<S>
where
    S: Iterator<Item = char>,
{
    /// The source code to be analyzed
    source: RwLock<HistoryIter<S>>,

    /// Position in the source (line, column)
    position: (usize, usize),
}

impl<S> Analysis<S>
where
    S: Iterator<Item = char>,
{
    pub fn new(source: S) -> Self {
        Self {
            source: source.into_history(),
            position: (0, 0),
        }
    }
}

impl<S> Iterator for Analysis<S>
where
    S: Iterator<Item = char>,
{
    type Item = Result<TokenExt>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut parsed = Token::parse_from(self.source.view());
        parsed.sort_by_key(|(src, _)| src.len());

        let last = parsed.pop()?;
        Some(Ok(TokenExt {
            token: last.1,
            lexeme: Lexeme {
                content: last.0,
                location: LexemeLocation {
                    position: Some(self.position),
                    ..Default::default()
                }
            }
        }))
    }
}
