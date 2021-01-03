use crate::token_type::TokenType;

#[derive(Clone, Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    // object literal?
    pub line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: &str, line: usize) -> Self {
        Token {
            ttype: ttype,
            lexeme: lexeme.to_string(),
            line: line,
        }
    }

    pub fn new_default(ttype: TokenType) -> Self {
        Token {
            ttype: ttype,
            lexeme: "".to_string(),
            line: 0,
        }
    }
}
