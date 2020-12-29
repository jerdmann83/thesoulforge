use std::io::{self, Read};

use crate::scanner::*;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn run(&mut self, s: &str) {
        let mut sc = Scanner::new(s);
    }

    pub fn run_prompt(&mut self) {
        let mut buf = String::new();
        loop {
            print!("> ");
            io::stdin().read_to_string(&mut buf).unwrap();
            self.run(&buf);
            self.had_error = false;
        }
    }

    pub fn run_file(&mut self, f: &str) {
        self.run(f);

        if self.had_error {
            //
        }
    }

    pub fn error(line: usize, msg: &str) {
        Lox::report(line, "", msg);
    }

    pub fn report(line: usize, loc: &str, msg: &str) {
        eprintln!("[line {} Error{}: {}", line, loc, msg);
    }
}
