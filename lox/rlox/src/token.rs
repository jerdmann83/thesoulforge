use crate::token_type::TokenType;

#[derive(Clone, Debug)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    // object literal?
    line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            ttype: ttype,
            lexeme: lexeme,
            line: line,
        }
    }
}
