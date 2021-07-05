pub mod error;
pub mod parse;
pub mod token;

use error::*;
use parse::Parse;
use std::iter::Peekable;
use token::{
    lexeme::{Lexeme, LexemeLocation},
    Token, TokenExt,
};

/// A lexical analysis
#[derive(Debug)]
pub struct Analysis<S>
where
    S: Iterator<Item = char>,
{
    /// The source code to be analyzed
    source: Peekable<S>,

    /// Position in the source (line, column)
    position: (usize, usize),
}

impl<S> Analysis<S>
where
    S: Iterator<Item = char> + Clone,
{
    pub fn new(source: S) -> Self {
        Self {
            source: source.peekable(),
            position: (0, 0),
        }
    }
}

impl<S> Iterator for Analysis<S>
where
    S: Iterator<Item = char> + Clone,
{
    type Item = Result<TokenExt>;

    fn next(&mut self) -> Option<Self::Item> {
        // Consume any leading whitespace before parsing
        while self.source.peek()?.is_whitespace() {
            self.source.next();
        }

        let mut parsed = Token::parse_from(&mut self.source.clone());
        parsed.sort_by_key(|(src, _)| src.len());

        let (src, token) = parsed.remove(0);

        // Consume the token's characters from the source
        self.source.advance_by(src.len()).unwrap();

        Some(Ok(TokenExt {
            token,
            lexeme: Lexeme {
                content: src,
                location: LexemeLocation {
                    position: Some(self.position),
                    ..Default::default()
                },
            },
        }))
    }
}
