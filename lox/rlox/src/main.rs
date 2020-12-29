use std::io::{self, Read};
use std::{env, process};

mod lox;
mod scanner;
mod token;
mod token_type;

use crate::lox::Lox;
use crate::scanner::Scanner;

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
