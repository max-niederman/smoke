#![feature(iter_advance_by)]
#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_uninit_array)]

mod lexer;
mod parser;
mod utils;

use std::{
    env, fs,
    io::{self, BufRead},
};

fn get_tokens(args: &[String]) -> io::Result<Vec<lexer::token::TokenExt>> {
    use lexer::{Analysis, AnalysisMeta};

    if let Some(filename) = args.get(1) {
        let file = fs::read_to_string(filename)?;

        let meta = AnalysisMeta {
            file: Some(filename.into()),
            ..Default::default()
        };

        Ok(Analysis::new(file.chars(), meta)
            .map(Result::unwrap)
            .collect())
    } else {
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line)?;

        Ok(Analysis::new(line.chars(), AnalysisMeta::default())
            .map(Result::unwrap)
            .collect())
    }
}

fn parse(tokens: &[lexer::token::TokenExt]) -> parser::error::Result<parser::expr::Expression> {
    use parser::Parsing;

    Parsing::new(tokens.iter().cloned()).parse()
}

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        let tokens = get_tokens(&args)?;
        eprintln!("Tokens: {:#?}", tokens);

        let expr = parse(&tokens);
        eprintln!("Expression: {:#?}", expr);
    } else {
        loop {
            let tokens = get_tokens(&args)?;
            eprintln!("Tokens: {:#?}", tokens);

            let expr = parse(&tokens);
            eprintln!("Expression: {:#?}", expr);
        }
    }

    Ok(())
}
