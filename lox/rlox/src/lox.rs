use std::io::{self, Read, Write};

use crate::ast_printer::*;
use crate::parser::*;
use crate::scanner::*;
use crate::token::*;
use crate::token_type::*;
use std::fs::File;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn run(&mut self, s: &str) {
        let sc = Scanner::new(s);
        let toks = &sc.scan_tokens();
        // println!("tokens: {:?}", toks);

        let p = Parser::new(&toks);
        let expr = p.parse().unwrap();

        if self.had_error {
            todo!();
        }

        // println!("{:?}", expr);
        println!("ast: {}", serialize_ast(expr));
    }

    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            let _ = io::stdout().flush();

            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            self.run(&buf);
            self.had_error = false;
        }
    }

    pub fn run_file(&mut self, f: &str) {
        let mut buf = String::new();
        File::open(f).unwrap().read_to_string(&mut buf).unwrap();
        self.run(&buf);

        if self.had_error {
            todo!();
        }
    }

    pub fn error(line: usize, msg: &str) {
        Lox::report(line, "", msg);
    }

    pub fn report(line: usize, loc: &str, msg: &str) {
        eprintln!("[line {} Error{}: {}", line, loc, msg);
    }

    pub fn token_error(t: Token, msg: &str) {
        match t.ttype {
            TokenType::EOF => Self::report(t.line, " at end", msg),
            _ => Self::report(t.line, &format!(" at '{}'", t.lexeme), msg),
        }
    }
}
