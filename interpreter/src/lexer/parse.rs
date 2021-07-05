use super::token::Token;

pub trait Parse: Sized {
    fn parse_from<I>(input: &mut I) -> Vec<(String, Self)>
    where
        I: Iterator<Item = char> + Clone;
}

impl Parse for Token {
    fn parse_from<I>(input: &mut I) -> Vec<(String, Self)>
    where
        I: Iterator<Item = char> + Clone,
    {
        let mut ret = Vec::new();

        use token_parsers::TokenParser;
        macro_rules! use_parser {
            ($parser:ty) => {
                ret.append(&mut <$parser>::parse(&mut input.clone()))
            };

            ($( $parser:ty ),*,) => {
                $( use_parser!($parser); )*
            }
        }

        // NOTE: This order is important, since parser results are appended in this order
        use_parser![
            token_parsers::Static,
            token_parsers::Identifier,
            token_parsers::Str,
            token_parsers::Float,
            token_parsers::Integer,
        ];

        ret
    }
}

mod token_parsers {
    use crate::lexer::token::Token::{self, *};

    pub trait TokenParser {
        fn parse<I: Iterator<Item = char> + Clone>(input: &mut I) -> Vec<(String, Token)>;
    }

    macro_rules! decl_parser {
        ($type:ty, $name:ident : |$input:ident| $body:expr) => {
            pub struct $name;
            impl TokenParser for $name {
                fn parse<I: Iterator<Item = char> + Clone>($input: &mut I) -> Vec<(String, Token)> { $body }
            }
        };

        ($type:ty, { $( $name:ident : |$input:ident| $body:expr );*; }) => {
            $( decl_parser!($type, $name : |$input| $body); )*
        };
    }

    decl_parser!(Token, {
        Static : |input| {
            let mut ret = Vec::new();

            macro_rules! decl_token {
                ($string:expr => $token:expr) => {
                    if input.clone().take($string.len()).eq($string.chars()) {
                        ret.push(($string.into(), $token))
                    }
                };

                ($( $string:expr => $token:expr ),*,) => {
                    $( decl_token!($string => $token); )*
                }
            }

            decl_token! {
                // Braces
                "(" => ParenLeft, ")" => ParenRight,
                "{" => CurlyLeft, "}" => CurlyRight,
                "[" => SquareLeft, "]" => SquareRight,

                // Operators
                "," => Comma,
                "." => Dot,
                "-" => Minus, "+" => Plus,
                "/" => Slash, "*" => Star,
                "=" => Equal, "==" => EqualEqual,
                "!" => Bang, "!=" => BangEqual,
                ">" => Greater, ">=" => GreaterEqual,
                "<" => Less, "<=" => LessEqual,

                // Keywords
                "fn" => Function, "return" => Return,
                "let" => Let,
                "if" => If, "else" => Else,
                "for" => For, "while" => While,

                ";" => Semicolon,
            };

            ret
        };

        Identifier : |input| {
            let mut input = input.peekable();

            let mut ident = String::new();
            if let Some(first) = input.next_if(|ch| *ch == '_' || ch.is_alphabetic()) {
                ident.push(first);

                loop {
                    match input.next() {
                        Some(ch) if ch == '_' || ch.is_alphanumeric() => ident.push(ch),
                        _ => break,
                    }
                }

                // NOTE: This should really return all possible identifiers, but just the longest one is fine
                // since it would be used anyway
                vec![(
                    ident.clone(),
                    Token::Identifier(ident)
                )]
            } else {
                vec![]
            }
        };

        Str : |input| {
            let mut input = input.peekable();

            if input.next() == Some('"') {
                let literal: String = input.take_while(|ch| *ch != '"').collect();

                vec![(
                    format!("\"{}\"", literal),
                    Token::Str(literal),
                )]
            } else {
                vec![]
            }
        };

        Float : |input| {
            let src: String = input.take_while(|ch| !ch.is_whitespace()).collect();

            if let Ok(literal) = src.parse::<f64>() {
                vec![(
                    src,
                    Token::Float(literal),
                )]
            } else {
                vec![]
            }
        };

        Integer : |input| {
            let src: String = input.take_while(|ch| !ch.is_whitespace()).collect();

            if let Ok(literal) = src.parse::<isize>() {
                vec![(
                    src,
                    Token::Integer(literal),
                )]
            } else {
                vec![]
            }
        };
    });
}

#[cfg(test)]
mod tests {
    use super::token_parsers::TokenParser;
    use super::*;
    use crate::lexer::token::Token::*;

    #[test]
    fn parses_static_tokens() {
        macro_rules! token_pairs {
            ($( $string:expr => [ $( $tokens:expr ),* ] ),*,) => {
                &[ $( ($string, &[$( $tokens, )*]) ),*, ]
            }
        }

        const TOKENS: &[(&str, &[Token])] = token_pairs![
            // Braces
            "(" => [ParenLeft], ")" => [ParenRight],
            "{" => [CurlyLeft], "}" => [CurlyRight],
            "[" => [SquareLeft], "]" => [SquareRight],

            // Operators
            "," => [Comma],
            "." => [Dot],
            "-" => [Minus], "+" => [Plus],
            "/" => [Slash], "*" => [Star],
            "=" => [Equal], "==" => [Equal, EqualEqual],
            "!" => [Bang], "!=" => [Bang, BangEqual],
            ">" => [Greater], ">=" => [Greater, GreaterEqual],
            "<" => [Less], "<=" => [Less, LessEqual],

            // Keywords
            "fn" => [Function], "return" => [Return],
            "let" => [Let],
            "if" => [If], "else" => [Else],
            "for" => [For], "while" => [While],

            ";" => [Semicolon],
        ];

        for (src, correct) in TOKENS {
            let parses = token_parsers::Static::parse(&mut src.chars());

            assert!(
                parses.iter().map(|(_, tk)| tk).eq(correct.iter()),
                "Parser results were incorrect\nParser Results: {:#?}\nCorrect Results: {:#?}",
                parses,
                correct,
            );
        }
    }
}
