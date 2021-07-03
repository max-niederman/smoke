pub mod lexeme;

use lexeme::Lexeme;

/// A token without metadata
#[derive(Debug, Clone)]
pub struct Token {
    /// The lexeme from which the [`Token`] was parsed
    pub lexeme: Lexeme,

    /// The [`Token`]'s type and literal value, if any
    pub data: TokenData,
}

/// A [`Token`]'s type and literal value, if any
#[derive(Debug, Clone)]
pub enum TokenData {
    // Brackets
    ParenLeft, ParenRight,
    CurlyLeft, CurlyRight,
    SquareLeft, SquareRight,

    // Operators
    Comma,
    Dot,
    Minus, Plus,
    Slash, Star,
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,


    // Keywords
    Fn, Return,
    Let,
    If, Else,
    For, While, Loop,

    // Literals
    Identifier(String),
    String(String),
    Float(f64),
    Integer(usize),

    Semicolon,
    EOF,
}
