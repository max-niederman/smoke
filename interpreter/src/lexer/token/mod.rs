pub mod lexeme;

use lexeme::Lexeme;

/// A token with metadata
#[derive(Debug, Clone, PartialEq)]
pub struct TokenExt {
    /// The [`Token`]'s type and literal value, if any
    pub token: Token,

    /// The lexeme from which the [`Token`] was parsed
    pub lexeme: Lexeme,
}

/// A raw token with no metadata
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Braces
    ParenLeft,
    ParenRight,
    CurlyLeft,
    CurlyRight,
    SquareLeft,
    SquareRight,

    // Operators
    Comma,
    Dot,
    Minus,
    Plus,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Keywords
    Function,
    Return,
    Let,
    If,
    Else,
    For,
    While,

    // Literals
    Identifier(String),
    Nil,
    Bool(bool),
    Integer(isize),
    Float(f64),
    Str(String),

    Semicolon,
}
