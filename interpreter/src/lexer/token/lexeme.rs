/// A lexeme with optional metadata
#[derive(Debug, Clone, PartialEq)]
pub struct Lexeme {
    /// The raw content
    pub content: String,

    /// The location from which the lexeme was parsed
    pub location: LexemeLocation,
}

/// The location from which a [`Lexeme`] was parsed
#[derive(Debug, Clone, PartialEq)]
pub enum LexemeLocation {
    File {
        /// The file wherein the [`Lexeme`] was encountered
        path: Option<std::path::PathBuf>,

        /// A tuple containing the line and column numbers of the start of the [`Lexeme`]
        position: (usize, usize),
    },
    Repl,
    Internal,
}
