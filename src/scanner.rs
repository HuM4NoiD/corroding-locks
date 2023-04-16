use crate::{
    error::error_line,
    token::{Token, TokenType},
    value::Value,
};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        &self.tokens
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let character = self.advance();
        match character {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            ';' => self.add_token(TokenType::Semicolon),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.match_('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_('*') {
                    self.multiline_comment();
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),

            _ => {
                if character.is_digit(10) {
                    self.number();
                } else if character.is_alphabetic() {
                    self.identifier();
                } else {
                    error_line(self.line, "Unexpected Character ")
                }
            }
        }
    }

    fn match_(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            error_line(self.line, "Unterminated String!");
            return;
        }
        self.advance();
        let slice = &self.source[self.start + 1..self.current + 1];
        let value = Some(Value::String(slice.into_iter().collect()));
        self.add_token_value(TokenType::String, value)
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let slice = &self.source[self.start..self.current];
        let num_str: String = slice.into_iter().collect();
        let num_val = Some(Value::Number(num_str.parse::<f64>().unwrap()));
        self.add_token_value(TokenType::Number, num_val)
    }

    fn identifier(&mut self) {
        while self::Scanner::is_identifier(self.peek()) {
            self.advance();
        }

        self.add_token(TokenType::Identifier);
    }

    fn multiline_comment(&mut self) {
        let mut nesting = 1;
        while nesting > 0 {
            if self.peek() == '\0' {
                error_line(self.line, "Unterminated Multiline Comment");
                return;
            }
            if self.peek() == '/' && self.peek_next() == '*' {
                self.advance();
                self.advance();
                nesting += 1
            }
            if self.peek() == '*' && self.peek_next() == '/' {
                self.advance();
                self.advance();
                nesting -= 1;
            }
        }
    }

    fn is_identifier(character: char) -> bool {
        character.is_alphanumeric() || character == '_'
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_value(token_type, None);
    }

    fn add_token_value(&mut self, token_type: TokenType, literal: Option<Value>) {
        let slice = &self.source[self.start..self.current];
        let lexeme = slice.into_iter().collect();
        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.line));
    }
}
