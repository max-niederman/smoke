/// A lexeme with optional metadata
#[derive(Debug, Clone)]
pub struct Lexeme {
    /// The raw content
    pub content: String,

    /// The location from which the lexeme was parsed
    pub location: LexemeLocation,
}

/// The location from which a [`Lexeme`] was parsed
#[derive(Debug, Clone)]
pub struct LexemeLocation {
    /// The file wherein the [`Lexeme`] was encountered
    pub file: Option<std::path::PathBuf>,

    /// A tuple containing the line and column numbers of the start of the [`Lexeme`]
    pub position: Option<(usize, usize)>,
}
