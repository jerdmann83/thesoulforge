use crate::expr::*;
use crate::token::*;
use crate::token_type::*;
use std::rc::Rc;

pub fn serialize_ast(e: ExprRef) -> String {
    format!("{}", parenthesize(e))
}

fn parenthesize(e: ExprRef) -> String {
    let mut buf = String::new();
    match e.borrow().etype {
        ExprType::Binary => {
            buf.push_str(&format!("({}", e.borrow().token.lexeme));
            for c in &e.borrow().children {
                buf.push_str(&format!(" {}", &parenthesize(Rc::clone(&c))));
            }
            buf.push_str(")");
        }
        ExprType::Literal => {
            buf.push_str(&format!("{}", e.borrow().token.lexeme,));
        }
        ExprType::Unary => {
            buf.push_str(&format!("({}", e.borrow().token.lexeme));
            buf.push_str(&format!(
                " {})",
                parenthesize(Rc::clone(&e.borrow().children[0]))
            ));
        }
        ExprType::Grouping => {
            buf.push_str(&format!("(group"));
            for c in &e.borrow().children {
                buf.push_str(&format!(" {}", c.borrow().token.lexeme));
            }
            buf.push_str(&format!(")"));
        }
    }

    buf
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_prints() {
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
