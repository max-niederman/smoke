#![feature(linked_list_cursors)]
#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_uninit_array)]

mod lexer;
mod utils;

use std::io;
use utils::prelude::*;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    loop {
        let tokens: Vec<_> = lexer::Analysis::new(stdin.lock().chars()).collect();
        eprintln!("Tokens: {:#?}", tokens);
    }
}
