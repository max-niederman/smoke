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
#[derive(Debug, Clone)]
pub struct Analysis<S>
where
    S: Iterator<Item = char>,
{
    /// The source code to be analyzed
    source: Peekable<S>,

    /// Position in the source (line, column)
    position: (usize, usize),

    /// Metadata on the [`Analysis`], including information about the source
    meta: AnalysisMeta,
}

impl<S> Analysis<S>
where
    S: Iterator<Item = char> + Clone,
{
    pub fn new(source: S, meta: AnalysisMeta) -> Self {
        Self {
            source: source.peekable(),
            position: (0, 0),
            meta,
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
            if self.source.next() == Some('\n') {
                self.position.0 += 1;
                self.position.1 = 0;
            } else {
                self.position.1 += 1;
            };
        }

        let mut parsed = Token::parse_from(&mut self.source.clone());
        parsed.sort_by_key(|(src, _)| src.len());

        parsed.truncate(1);
        let (src, token) = parsed.remove(0);

        // Consume the token's characters from the source
        self.source.advance_by(src.len()).unwrap();

        let position = self.position;
        self.position.1 += src.len();

        Some(Ok(TokenExt {
            token,
            lexeme: Lexeme {
                content: src,
                location: LexemeLocation {
                    file: self.meta.file.clone(),
                    position: Some(position),
                },
            },
        }))
    }
}

#[derive(Debug, Clone)]
pub struct AnalysisMeta {
    pub file: Option<std::path::PathBuf>,
}

impl Default for AnalysisMeta {
    fn default() -> Self {
        Self {
            file: None,
        }
    }
}
