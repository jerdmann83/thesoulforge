use std::{env, process};

mod ast_printer;
mod environment;
mod error;
mod expr;
mod interpreter;
mod lox;
mod parser;
mod scanner;
mod stmt;
mod token;
mod token_type;
mod value;

use crate::lox::Lox;

fn main() {
    let mut l = Lox::new();

    match env::args().len() {
        1 => l.run_prompt(),
        2 => l.run_file(&env::args().collect::<Vec<String>>()[1]),
        _ => {
            eprintln!("usage: rlox [script]");
            process::exit(-1);
        }
    }
}
