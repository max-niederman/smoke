#![feature(iter_advance_by)]
#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_uninit_array)]

mod lexer;
mod utils;

use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    loop {
        let tokens: Vec<_> =
            lexer::Analysis::new(stdin.lock().lines().next().unwrap()?.chars()).collect();
        eprintln!("Tokens: {:#?}", tokens);
    }
}
