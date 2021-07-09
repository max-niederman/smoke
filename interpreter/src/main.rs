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
    use lexer::{AnalysisMeta, Analyzer};

    if let Some(filename) = args.get(1) {
        let file = fs::read_to_string(filename)?;

        let meta = AnalysisMeta {
            file: Some(filename.into()),
            ..Default::default()
        };

        Ok(Analyzer::new(file.chars(), meta)
            .map(Result::unwrap)
            .collect())
    } else {
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line)?;

        Ok(Analyzer::new(line.chars(), AnalysisMeta::default())
            .map(Result::unwrap)
            .collect())
    }
}

fn parse(tokens: &[lexer::token::TokenExt]) -> parser::error::Result<parser::ast::Ast> {
    use parser::Parser;

    Parser::new(tokens.iter().cloned()).parse()
}

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        let tokens = get_tokens(&args)?;
        eprintln!("Tokens: {:#?}", tokens);

        let ast = parse(&tokens).unwrap();
        eprintln!("Expression: {:#?}", ast);
    } else {
        loop {
            eprint!("> ");
            let tokens = get_tokens(&args)?;
            eprintln!(
                "Tokens: {:#?}",
                tokens.iter().map(|tke| &tke.token).collect::<Vec<_>>()
            );

            match parse(&tokens) {
                Ok(ast) => eprintln!("Expression: {:#?}", ast),
                Err(err) => eprintln!("Parser error: {:#?}", err),
            }
        }
    }

    Ok(())
}
