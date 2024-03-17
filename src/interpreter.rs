use crate::{
    ast::Expr,
    token::{Token, TokenType},
};

pub struct Interpreter;

#[derive(Debug, PartialEq)]
pub enum IntrResult {
    Number(f64),
    String(String),
    Bool(bool),
    None,
}

#[derive(Debug, PartialEq)]
pub enum IntrError {
    Runtime(Token, String),
    Unsupported(Token),
    NotImplemented(Token),
}

impl Interpreter {
    pub fn evaluate(&mut self, expr: &Expr) -> Result<IntrResult, IntrError> {
        match expr {
            Expr::Binary(left, operator, right) => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                match (operator.token_type, left, right) {
                    (
                        TokenType::Minus, // -
                        IntrResult::Number(left),
                        IntrResult::Number(right),
                    ) => Ok(IntrResult::Number(left - right)),
                    (
                        TokenType::Slash, // /
                        IntrResult::Number(left),
                        IntrResult::Number(right),
                    ) => Ok(IntrResult::Number(left / right)),
                    (
                        TokenType::Star, // *
                        IntrResult::Number(left),
                        IntrResult::Number(right),
                    ) => Ok(IntrResult::Number(left * right)),
                    (
                        TokenType::Plus, // +
                        IntrResult::Number(left),
                        IntrResult::Number(right),
                    ) => Ok(IntrResult::Number(left + right)),
                    (
                        TokenType::Plus, // + string string
                        IntrResult::String(left),
                        IntrResult::String(right),
                    ) => Ok(IntrResult::String(left + right.as_ref())),
                    (
                        TokenType::Greater, // >
                        IntrResult::Number(left),
                        IntrResult::Number(right),
                    ) => Ok(IntrResult::Bool(left > right)),
                    (
                        TokenType::GreaterEqual, // >=
                        IntrResult::Number(left),
                        IntrResult::Number(right),
                    ) => Ok(IntrResult::Bool(left >= right)),
                    (
                        TokenType::Less, // <
                        IntrResult::Number(left),
                        IntrResult::Number(right),
                    ) => Ok(IntrResult::Bool(left < right)),
                    (
                        TokenType::LessEqual, // <=
                        IntrResult::Number(left),
                        IntrResult::Number(right),
                    ) => Ok(IntrResult::Bool(left <= right)),
                    (
                        TokenType::EqualEqual, // ==
                        IntrResult::Number(left),
                        IntrResult::Number(right),
                    ) => Ok(IntrResult::Bool(left == right)),
                    (
                        TokenType::EqualEqual, // == string string
                        IntrResult::String(left),
                        IntrResult::String(right),
                    ) => Ok(IntrResult::Bool(left == right)),
                    (
                        TokenType::EqualEqual, // == nil nil
                        IntrResult::None,
                        IntrResult::None,
                    ) => Ok(IntrResult::Bool(true)),
                    (
                        TokenType::BangEqual, // !=
                        IntrResult::Number(left),
                        IntrResult::Number(right),
                    ) => Ok(IntrResult::Bool(left == right)),
                    (
                        TokenType::BangEqual, // != string string
                        IntrResult::String(left),
                        IntrResult::String(right),
                    ) => Ok(IntrResult::Bool(left == right)),
                    _ => Err(IntrError::Unsupported(operator.clone())),
                }
            }
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Unary(operator, expr) => {
                let right = self.evaluate(expr)?;

                match (operator.token_type, right) {
                    (TokenType::Bang, IntrResult::Number(number)) => Ok(IntrResult::Bool(number > 0.0)),
                    (TokenType::Bang, IntrResult::Bool(value)) => Ok(IntrResult::Bool(!value)),
                    (TokenType::Bang, IntrResult::None) => Ok(IntrResult::Bool(false)),
                    (TokenType::Bang, _) => Ok(IntrResult::Bool(true)),
                    (TokenType::Minus, IntrResult::Number(number)) => Ok(IntrResult::Number(-number)),
                    _ => Err(IntrError::Unsupported(operator.clone())),
                }
            }
            Expr::Literal(literal) => match literal {
                crate::token::Literal::String(value) => Ok(IntrResult::String(value.clone())),
                crate::token::Literal::Number(number) => Ok(IntrResult::Number(*number)),
                crate::token::Literal::True => Ok(IntrResult::Bool(true)),
                crate::token::Literal::False => Ok(IntrResult::Bool(false)),
                crate::token::Literal::Nil => Ok(IntrResult::None),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::Parser, scanner};

    use super::*;

    fn run(input: &str) -> Result<IntrResult, IntrError> {
        let mut scanner = scanner::Scanner::new(input.into());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let expr = parser.expression().unwrap();
        let mut interpreter = Interpreter;
        interpreter.evaluate(&expr)
    }

    #[test]
    fn test_evaluate_unary_expression() {
        let result = run("-456").unwrap();
        assert_eq!(result, IntrResult::Number(-456.0));
    }

    #[test]
    fn test_evaluate_binary_expression() {
        let tests = [
            ("2 > 1", IntrResult::Bool(true)),
            ("2 > 1", IntrResult::Bool(true)),
            ("1 > 2", IntrResult::Bool(false)),
            ("4 + 2", IntrResult::Number(6.0)),
            ("1 + 1 * 3", IntrResult::Number(4.0)),
            ("(1 + 1) * 3", IntrResult::Number(6.0)),
            ("400 - 402", IntrResult::Number(-2.0)),
            ("\"one\"", IntrResult::String("one".to_string())),
            ("\"one\" == \"one\"", IntrResult::Bool(true)),
            ("\"one\" != \"two\"", IntrResult::Bool(false)),
            ("\"hello \" + \"world\"", IntrResult::String("hello world".to_string())),
        ];

        for (input, expected) in tests.iter() {
            let result = run(input).unwrap();
            assert_eq!(result, *expected);
        }
    }

    #[test]
    fn test_evaluate_error() {
        let result = run("5 + true");
        assert!(result.is_err());
    }
}
