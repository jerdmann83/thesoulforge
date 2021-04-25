use std::io::{self, Read, Write};

use crate::interpreter::*;
use crate::parser::*;
use crate::scanner::*;
use crate::token::*;
use crate::token_type::*;
use std::fs::File;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct Lox {
    errs: u32,
    runtime_errs: u32,
}

static mut ERRS: [AtomicU32; 2] = [AtomicU32::new(0), AtomicU32::new(0)];
const ERR_INTERPRET: usize = 0;
const ERR_RUNTIME: usize = 1;

impl Lox {
    pub fn new() -> Self {
        Lox {
            errs: 0,
            runtime_errs: 0,
        }
    }

    pub fn run(&mut self, s: &str) {
        let sc = Scanner::new(s);
        let toks = &sc.scan_tokens();

        let p = Parser::new(&toks);
        let expr = p.parse();
        if expr.is_err() {
            return;
        }
        //println!("{:?}", expr);
        let expr = expr.unwrap();

        let mut ir = Interpreter::new();
        let val = ir.interpret(&expr);

        // println!("ast: {}", AstPrinter::serialize_stmts(&expr));
        match val {
            Ok(_) => {}
            Err(_e) => {} // println!("error: {:?}", e),
        }

        // if self.errs > 0 {
        //     todo!();
        // }
        // if self.runtime_errs > 0 {
        //     todo!();
        // }
    }

    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            let _ = io::stdout().flush();

            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            self.run(&buf);

            self.errs = 0;
            self.runtime_errs = 0;
        }
    }

    pub fn run_file(&mut self, f: &str) {
        let mut buf = String::new();
        File::open(f).unwrap().read_to_string(&mut buf).unwrap();
        self.run(&buf);

        if self.errs > 0 {
            todo!();
        }
    }

    /// safety:  atomic add
    pub fn error(line: usize, msg: &str) {
        unsafe {
            ERRS[ERR_INTERPRET].fetch_add(1, Ordering::SeqCst);
        }
        Lox::report(line, "", msg);
    }

    /// safety:  atomic add
    pub fn runtime_error(msg: &str) {
        unsafe {
            ERRS[ERR_RUNTIME].fetch_add(1, Ordering::SeqCst);
        }
        eprintln!("runtime error: {}", msg);
    }

    pub fn report(line: usize, loc: &str, msg: &str) {
        eprintln!("[line {}] error{}: {}", line, loc, msg);
    }

    #[allow(dead_code)]
    pub fn token_error(t: Token, msg: &str) {
        match t.ttype {
            TokenType::EOF => Self::report(t.line, " at end", msg),
            _ => Self::report(t.line, &format!(" at '{}'", t.lexeme), msg),
        }
    }
}

impl Drop for Lox {
    /// safety:  atomic store
    fn drop(&mut self) {
        unsafe {
            ERRS[ERR_INTERPRET].store(0, Ordering::Relaxed);
            ERRS[ERR_RUNTIME].store(0, Ordering::Relaxed);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn lox_test(buf: &str) {
        let mut l = Lox::new();
        l.run(&buf);
        assert_no_errs();
    }

    fn assert_no_errs() {
        // rust is rightfully complaining
        // refactor these counters into the Lox instance itself trade-off is
        // downstream components like scanner/parser/etc now need references to
        // the top-level lox instance.
        // either that or inject in callback functors
        unsafe {
            for e in &ERRS {
                assert!(e.load(Ordering::Relaxed) == 0);
            }
        }
    }

    #[test]
    pub fn lox_hello() {
        let buf = "
    var x=1;
    x = x + 1;
    print x;";
        lox_test(buf);
    }

    #[test]
    pub fn lox_scopes() {
        let buf = r#"
var a = "global a";
var b = "global b";
var c = "global c";
{
  var a = "outer a";
  var b = "outer b";
  {
    var a = "inner a";
    print a;
    print b;
    print c;
  }
  print a;
  print b;
  print c;
}
print a;
print b;
print c;"#;
        lox_test(buf);
    }

    #[test]
    pub fn lox_for() {
        let buf = "
    for (var x=0; x<10; x=x+1) {
        print x;
    }
    print 1;";
        lox_test(buf);
    }
}
