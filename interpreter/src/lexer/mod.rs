pub mod error;
pub mod token;

use crate::utils::{
    prelude::*,
    history_iter::HistoryIter,
};
use error::*;
use std::sync::RwLock;
pub use token::Token;

/// A lexical analysis
#[derive(Debug)]
pub struct Analysis<S>
where
    S: Iterator<Item = char>,
{
    /// The source code to be analyzed
    source: RwLock<HistoryIter<S>>,
}

impl<S> Analysis<S>
where
    S: Iterator<Item = char>,
{
    pub fn new(source: S) -> Self {
        Self {
            source: source.into_history(),
        }
    }
}

impl<S> Iterator for Analysis<S>
where
    S: Iterator<Item = char>,
{
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
