use crate::{
    ast::{binary, grouping, literal, unary, Expr},
    token::{Literal, Token, TokenType},
};

#[derive(Debug)]
pub enum ParserError {
    ParseError(String),
}

pub struct Parser<'a> {
    pub tokens: &'a Vec<Token>,
    pub current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// `equality` → `equality`
    pub fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    /// `equality` → `comparison ( ( "!=" | "==" ) comparison )*`
    ///
    /// For each iteration, we create a new binary expression using the previous one as the left operand:  
    /// `a == b == c == d == e`  ->  `(== (== (== (== a b) c) d) e)`
    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.comparison()?;

        while let Some(operator) = self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let right = self.comparison()?;
            left = binary(left, operator, right);
        }

        Ok(left)
    }

    /// `comparison` → `term ( ( ">" | ">=" | "<" | "<=" ) term )*`
    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.term()?;

        while let Some(operator) = self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let right = self.term()?;
            left = binary(left, operator, right);
        }

        Ok(left)
    }

    /// term → factor ( ( "-" | "+" ) factor )*
    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.factor()?;

        while let Some(operator) = self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let right = self.factor()?;
            left = binary(left, operator, right);
        }

        Ok(left)
    }

    /// factor → unary ( ( "/" | "*" ) unary )*
    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.unary()?;

        while let Some(operator) = self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let right = self.unary()?;
            left = binary(left, operator, right);
        }

        Ok(left)
    }

    /// unary → ( "!" | "-" ) unary | primary
    fn unary(&mut self) -> Result<Expr, ParserError> {
        if let Some(operator) = self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let right = self.unary()?;
            return Ok(unary(operator, right));
        }
        self.primary()
    }

    /// primary → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"
    fn primary(&mut self) -> Result<Expr, ParserError> {
        if let Some(token) = self.match_token(TokenType::Number) {
            return Ok(literal(Literal::Number(token.lexeme.parse().unwrap())));
        }
        if let Some(token) = self.match_token(TokenType::String) {
            return Ok(literal(token.literal.unwrap()));
        }
        if let Some(_) = self.match_token(TokenType::True) {
            return Ok(literal(Literal::True));
        }
        if let Some(_) = self.match_token(TokenType::False) {
            return Ok(literal(Literal::False));
        }
        if let Some(_) = self.match_token(TokenType::Nil) {
            return Ok(literal(Literal::Nil));
        }
        if let Some(_) = self.match_token(TokenType::LeftParen) {
            let expr = self.expression()?;

            // TODO: Lox error https://craftinginterpreters.com/parsing-expressions.html#entering-panic-mode
            self.consume(TokenType::RightParen, "Expect ')' after expression.")
                .unwrap();

            return Ok(grouping(expr));
        }

        Err(ParserError::ParseError("Expect expression.".into()))
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParserError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(ParserError::ParseError(message.into()))
    }

    fn match_tokens(&mut self, token_types: &[TokenType]) -> Option<Token> {
        for token_type in token_types {
            if self.check(*token_type) {
                return Some(self.advance().clone());
            }
        }

        None
    }

    fn match_token(&mut self, token_type: TokenType) -> Option<Token> {
        if self.check(token_type) {
            return Some(self.advance().clone());
        }

        None
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        let token = &self.tokens[self.current];

        if !self.is_at_end() {
            self.current += 1;
        }

        token.clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    // TODO: https://craftinginterpreters.com/parsing-expressions.html#synchronizing-a-recursive-descent-parser
}

#[cfg(test)]
mod tests {
    use crate::scanner::Scanner;

    use super::*;

    #[test]
    fn test_exression() {
        let tests = [
            ("1", "1"),
            ("1 + 2", "(+ 1 2)"),
            ("(1 + 2)", "(group (+ 1 2))"),
            ("1 - 2", "(- 1 2)"),
            ("1 * 2", "(* 1 2)"),
            ("1 / 2", "(/ 1 2)"),
            ("1 + 2 * 3", "(+ 1 (* 2 3))"),
            ("1 + 2 * 3 - 4", "(- (+ 1 (* 2 3)) 4)"),
            ("1 + (2 * 3) - 4", "(- (+ 1 (group (* 2 3))) 4)"),
            ("1 + (2 * 3) - (4 * 5)", "(- (+ 1 (group (* 2 3))) (group (* 4 5)))"),
        ];

        for (input, expected) in tests {
            let mut scanner = Scanner::new(input.into());
            let tokens = scanner.scan_tokens();
            let mut parser = Parser::new(tokens);
            let expr = parser.expression().unwrap();
            assert_eq!(expr.to_string(), expected);
        }
    }
}
