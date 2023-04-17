use crate::{
    error::{error_line, report_err, Error},
    token::{Token, TokenType},
    value::Value,
};
use std::{iter::Iterator};

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(&mut tokens)
        }

        tokens
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self, result: &mut Vec<Token>) {
        let character = self.advance();
        match character {
            '(' => self.add_token(TokenType::LeftParen, result),
            ')' => self.add_token(TokenType::RightParen, result),
            '{' => self.add_token(TokenType::LeftBrace, result),
            '}' => self.add_token(TokenType::RightBrace, result),
            ',' => self.add_token(TokenType::Comma, result),
            '.' => self.add_token(TokenType::Dot, result),
            ';' => self.add_token(TokenType::Semicolon, result),
            '-' => self.add_token(TokenType::Minus, result),
            '+' => self.add_token(TokenType::Plus, result),
            '*' => self.add_token(TokenType::Star, result),
            '!' => {
                if self.match_('=') {
                    self.add_token(TokenType::BangEqual, result)
                } else {
                    self.add_token(TokenType::Bang, result)
                }
            }
            '=' => {
                if self.match_('=') {
                    self.add_token(TokenType::EqualEqual, result)
                } else {
                    self.add_token(TokenType::Equal, result)
                }
            }
            '<' => {
                if self.match_('=') {
                    self.add_token(TokenType::LessEqual, result)
                } else {
                    self.add_token(TokenType::Less, result)
                }
            }
            '>' => {
                if self.match_('=') {
                    self.add_token(TokenType::GreaterEqual, result)
                } else {
                    self.add_token(TokenType::Greater, result)
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
                    self.add_token(TokenType::Slash, result)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => match self.string() {
                Ok(str_token) => result.push(str_token),
                Err(e) => report_err(&e),
            },

            _ => {
                if character.is_digit(10) {
                    result.push(self.number());
                } else if character.is_alphabetic() {
                    result.push(self.identifier());
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

    fn string(&mut self) -> Result<Token, Error> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return Result::Err(Error::Scan {
                message: "Unterminated String".to_owned(),
                line: self.line,
            });
        }
        self.advance();
        let slice = &self.source[self.start + 1..self.current + 1];
        let value = Some(Value::String(slice.into_iter().collect()));
        Result::Ok(self.build_token_value(TokenType::String, value))
    }

    fn number(&mut self) -> Token {
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
        self.build_token_value(TokenType::Number, num_val)
    }

    fn identifier(&mut self) -> Token {
        while self::Scanner::is_identifier(self.peek()) {
            self.advance();
        }

        self.build_token(TokenType::Identifier)
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

    fn add_token(&mut self, token_type: TokenType, result: &mut Vec<Token>) {
        self.add_token_value(token_type, None, result);
    }

    fn add_token_value(
        &mut self,
        token_type: TokenType,
        literal: Option<Value>,
        result: &mut Vec<Token>,
    ) {
        let slice = &self.source[self.start..self.current];
        let lexeme = slice.into_iter().collect();
        result.push(Token::new(token_type, lexeme, literal, self.line));
    }

    fn build_token(&mut self, token_type: TokenType) -> Token {
        self.build_token_value(token_type, None)
    }

    fn build_token_value(&mut self, token_type: TokenType, literal: Option<Value>) -> Token {
        let slice = &self.source[self.start..self.current];
        let lexeme = slice.into_iter().collect();
        Token::new(token_type, lexeme, literal, self.line)
    }
}
