use std::fmt::Display;

use crate::token::{Literal, Token};

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    // Assign(Token, Box<Expr>),
    // Call(Box<Expr>, Token, Vec<Expr>),
    // Get(Box<Expr>, Token),
    Grouping(Box<Expr>),
    Literal(Literal),
    // Logical(Box<Expr>, Token, Box<Expr>),
    // Set(Box<Expr>, Token, Box<Expr>),
    // Super(Token, Token),
    // This(Token),
    Unary(Token, Box<Expr>),
    // Variable(Token),
}

pub fn binary(left: Expr, operator: Token, right: Expr) -> Expr {
    Expr::Binary(Box::new(left), operator, Box::new(right))
}

pub fn grouping(expr: Expr) -> Expr {
    Expr::Grouping(Box::new(expr))
}

pub fn literal(literal: Literal) -> Expr {
    Expr::Literal(literal)
}

pub fn unary(operator: Token, right: Expr) -> Expr {
    Expr::Unary(operator, Box::new(right))
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(left, operator, right) => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Grouping(expr) => {
                write!(f, "(group {})", expr)
            }
            Expr::Literal(literal) => match literal {
                Literal::String(v) => write!(f, "{}", v),
                Literal::Number(v) => write!(f, "{:.1}", v),
            },

            Expr::Unary(operator, right) => write!(f, "({} {})", operator.lexeme, right),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::TokenType;

    use super::*;

    #[test]
    fn test_binary_expr() {
        let expr = binary(
            literal(Literal::Number(1.0)),
            Token::new(TokenType::Minus, "-".into(), None, 1),
            literal(Literal::Number(2.0)),
        );
        assert_eq!(expr.to_string(), "(- 1.0 2.0)");
    }

    #[test]
    fn test_nested_expr() {
        let expr = binary(
            literal(Literal::Number(1.0)),
            Token::new(TokenType::Minus, "-".into(), None, 1),
            grouping(literal(Literal::Number(2.0))),
        );
        assert_eq!(expr.to_string(), "(- 1.0 (group 2.0))");
    }
}
