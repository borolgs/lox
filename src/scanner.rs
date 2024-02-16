use crate::token::*;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".into(), None, self.line));

        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => match self.match_second('=') {
                true => self.add_token(TokenType::BangEqual, None),
                false => self.add_token(TokenType::Bang, None),
            },
            token if token == '=' && self.match_second('=') => {
                self.add_token(TokenType::EqualEqual, None)
            }
            '=' => self.add_token(TokenType::Equal, None),
            '<' => match self.match_second('=') {
                true => self.add_token(TokenType::LessEqual, None),
                false => self.add_token(TokenType::Less, None),
            },
            '>' => match self.match_second('=') {
                true => self.add_token(TokenType::GreaterEqual, None),
                false => self.add_token(TokenType::Greater, None),
            },
            '/' => match self.match_second('/') {
                false => self.add_token(TokenType::Slash, None),
                true => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
            },
            '"' => self.string(),
            token if token.is_digit(10) => self.number(),
            token if token.is_alphabetic() => self.identifier(),
            ' ' | '\t' | '\r' => (),
            '\n' => self.line += 1,
            _ => {
                // TODO: Lox.error(line, "Unexpected character.");
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current];

        let token_type: TokenType = text.try_into().unwrap_or(TokenType::Identifier);

        self.add_token(token_type, None);
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // consume the dot
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let value: f64 = self.source[self.start..self.current].parse().unwrap();

        self.add_token(TokenType::Number, Some(Literal::Number(value)));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            // TODO: Lox.error(line, "Unterminated string.");
            return;
        }

        // closing quote
        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, Some(Literal::String(value)));
    }

    fn advance(&mut self) -> char {
        let char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        char
    }

    fn match_second(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let char = self.source.chars().nth(self.current).unwrap();
        if char != expected {
            return false;
        }
        self.current += 1;

        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grouping_tokens() {
        let mut scanner = Scanner::new("({ })".into());
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(tokens[2].token_type, TokenType::RightBrace);
    }

    #[test]
    fn test_operator_tokens() {
        let mut scanner = Scanner::new("!= <= == =".into());
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token_type, TokenType::BangEqual);
        assert_eq!(tokens[1].token_type, TokenType::LessEqual);
        assert_eq!(tokens[2].token_type, TokenType::EqualEqual);
        assert_eq!(tokens[3].token_type, TokenType::Equal);
    }

    #[test]
    fn test_string_literal_tokens() {
        let mut scanner = Scanner::new("\"hello\"".into());
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].literal, Some(Literal::String("hello".into())));
    }

    #[test]
    fn test_string_multiline_literal_tokens() {
        let mut scanner = Scanner::new("\"hello\nworld\"".into());
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(
            tokens[0].literal,
            Some(Literal::String("hello\nworld".into()))
        );
    }

    #[test]
    fn test_number_literal_tokens() {
        let mut scanner = Scanner::new("123.456 42".into());
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].literal, Some(Literal::Number(123.456)));
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].literal, Some(Literal::Number(42.0)));
    }

    #[test]
    fn test_identifier_tokens() {
        let mut scanner = Scanner::new("foo bar".into());
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].lexeme, "foo");
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].lexeme, "bar");
    }

    #[test]
    fn test_keyword_tokens() {
        let mut scanner = Scanner::new("for return var".into());
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::For);
        assert_eq!(tokens[1].token_type, TokenType::Return);
        assert_eq!(tokens[2].token_type, TokenType::Var);
    }

    #[test]
    fn test_comments_skip() {
        let mut scanner = Scanner::new("// some comment\n () // comment after".into());
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 3);
    }
}
