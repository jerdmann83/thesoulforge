use crate::expr::*;

pub fn serialize_ast(e: Expr) -> String {
    format!("{}", parenthesize(&e))
}

fn parenthesize(e: &Expr) -> String {
    let mut buf = String::new();
    match e.etype {
        ExprType::Binary => {
            buf.push_str(&format!("({}", e.token.lexeme));
            for c in &e.children {
                buf.push_str(&format!(" {}", &parenthesize(&c)));
            }
            buf.push_str(")");
        }
        ExprType::Literal => {
            buf.push_str(&format!("{}", e.token.lexeme));
        }
        ExprType::Unary => {
            buf.push_str(&format!("({}", e.token.lexeme));
            buf.push_str(&format!(" {})", parenthesize(&e.children[0])));
        }
        ExprType::Grouping => {
            buf.push_str(&format!("(group"));
            for c in &e.children {
                buf.push_str(&format!(" {}", c.token.lexeme));
            }
            buf.push_str(&format!(")"));
        }
    }

    buf
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::*;
    use crate::token_type::*;

    #[test]
    fn it_prints() {
        let e = Expr::new_binary(
            Token::new(TokenType::Star, "*", 1),
            Expr::new_literal(Token::new(TokenType::Number(1.0), "1", 1)),
            Expr::new_literal(Token::new(TokenType::Number(2.0), "2", 1)),
        );
        assert_eq!(serialize_ast(e), "(* 1 2)");

        let e = Expr::new_binary(
            Token::new(TokenType::Star, "*", 1),
            Expr::new_unary(
                Token::new(TokenType::Minus, "-", 1),
                Expr::new_literal(Token::new(TokenType::Number(123.0), "123", 1)),
            ),
            Expr::new_grouping(&vec![Expr::new_literal(Token::new(
                TokenType::Number(45.67),
                "45.67",
                1,
            ))]),
        );
        assert_eq!(serialize_ast(e), "(* (- 123) (group 45.67))");
    }
}
