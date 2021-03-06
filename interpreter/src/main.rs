#![feature(result_cloned)]
#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_uninit_array)]
#![feature(box_into_inner)]
#![feature(iter_advance_by)]

mod interpreter;
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

fn parse(tokens: &[lexer::token::TokenExt]) -> parser::Result<parser::ast::Ast> {
    use parser::Parser;

    Parser::new(tokens.iter().cloned()).parse()
}

fn interpret(ast: parser::ast::Ast) -> interpreter::Result<interpreter::ValueWrap> {
    use interpreter::Interpreter;

    Interpreter::new().interpret(&ast)
}

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        let tokens = get_tokens(&args)?;
        eprintln!("Tokens: {:#?}", tokens);

        let parsed = parse(&tokens).unwrap();
        eprintln!("Parsed: {:#?}", parsed);

        let res = interpret(parsed).unwrap();
        eprintln!("Result: {:#?}", res);
    } else {
        loop {
            eprint!("> ");
            let tokens = get_tokens(&args)?;
            eprintln!(
                "Tokens: {:#?}",
                tokens.iter().map(|tke| &tke.token).collect::<Vec<_>>()
            );

            let parsed = match parse(&tokens) {
                Ok(ast) => ast,
                Err(err) => {
                    eprintln!("Parser error:\n{:#?}", err);
                    continue;
                }
            };
            eprintln!("Parsed: {:#?}", parsed);

            let res = match interpret(parsed) {
                Ok(ast) => ast,
                Err(err) => {
                    eprintln!("Runtime error:\n{}", err);
                    continue;
                }
            };
            eprintln!("Result: {:#?}", res);
        }
    }

    Ok(())
}
