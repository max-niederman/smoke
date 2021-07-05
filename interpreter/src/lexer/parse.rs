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

        use_parser![token_parsers::Whitespace, token_parsers::Static,];

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
