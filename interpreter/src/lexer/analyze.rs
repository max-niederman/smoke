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
            token_parsers::Integer,
            token_parsers::Float,
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

                // Literals
                "true" => Bool(true), "false" => Bool(false),
                "nil" => Nil,

                ";" => Semicolon,
            };

            ret
        };

        Identifier : |input| {
            let ident: String = input.take_while(|ch| *ch == '_' || ch.is_alphanumeric()).collect();

            match ident.chars().next() {
                Some(ch) if !ch.is_numeric() => vec![(ident.clone(), Token::Identifier(ident))],
                _ => vec![],
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

            if src.starts_with('-') { return vec![] }

            src
                .char_indices()
                .map(|(i, _)| &src[..=i])
                .filter_map(|sub| Some((sub.to_string(), Token::Float(sub.parse().ok()?))))
                .collect()
        };

        Integer : |input| {
            let src: String = input.take_while(|ch| !ch.is_whitespace()).collect();

            if src.starts_with('-') { return vec![] }

            src
                .char_indices()
                .map(|(i, _)| &src[..=i])
                .filter_map(|sub| Some((sub.to_string(), Token::Integer(sub.parse().ok()?))))
                .collect()
        };
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::Token::*;

    #[test]
    fn parses_tokens() {
        macro_rules! token_pairs {
            ($( $string:expr => [ $( $result:expr ),* ] ),*,) => {
                &[ $( ($string, &[$( $result, )*]) ),*, ]
            }
        }

        let tokens: &[(&str, &[(&str, Token)])] = token_pairs![
            // Braces
            "(" => [("(", ParenLeft)], ")" => [(")", ParenRight)],
            "{" => [("{", CurlyLeft)], "}" => [("}", CurlyRight)],
            "[" => [("[", SquareLeft)], "]" => [("]", SquareRight)],

            // Operators
            "," => [(",", Comma)],
            "." => [(".", Dot)],
            "-" => [("-", Minus)], "+" => [("+", Plus)],
            "/" => [("/", Slash)], "*" => [("*", Star)],
            "=" => [("=", Equal)], "==" => [("=", Equal), ("==", EqualEqual)],
            "!" => [("!", Bang)], "!=" => [("!", Bang), ("!=", BangEqual)],
            ">" => [(">", Greater)], ">=" => [(">", Greater), (">=", GreaterEqual)],
            "<" => [("<", Less)], "<=" => [("<", Less), ("<=", LessEqual)],

            // Keyword
            "fn" => [("fn", Function), ("fn", Identifier("fn".into()))],

            // Literals
            "nil" => [("nil", Nil), ("nil", Identifier("nil".into()))],
            "true" => [("true", Bool(true)), ("true", Identifier("true".into()))],
            "false" => [("false", Bool(false)), ("false", Identifier("false".into()))],
            "0" => [("0", Integer(0)), ("0", Float(0.0))],
            "0.0" => [("0", Integer(0)), ("0", Float(0.0)), ("0.", Float(0.0)), ("0.0", Float(0.0))],
            "\"string\"" => [("\"string\"", Str("string".into()))],

            ";" => [(";", Semicolon)],
        ];

        for (src, correct) in tokens {
            let mut parses = Token::parse_from(&mut src.chars());
            parses.sort_by_key(|(src, _)| src.len());

            assert!(
                parses.iter().cloned().eq(correct
                    .iter()
                    .cloned()
                    .map(|(src, tk)| (src.to_string(), tk))),
                "Parser results were incorrect\nParser Results: {:#?}\nCorrect Results: {:#?}",
                parses,
                correct,
            );
        }
    }
}
