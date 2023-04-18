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
        println!("Created scanner with source: {}", source);
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
                println!("for equal, current: {}, peek: {}", character, self.peek());
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
                Ok(str_token) => self.push_token(str_token, result),
                Err(e) => report_err(&e),
            },

            _ => {
                if character.is_digit(10) {
                    let t = self.number();
                    self.push_token(t, result)
                } else if character.is_alphabetic() {
                    let t = self.identifier_or_keyword();
                    self.push_token(t, result);
                } else {
                    error_line(self.line, format!("Unexpected character: {}", character).as_str())
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
                message: "Unterminated String".to_string(),
                line: self.line,
            });
        }
        self.advance();
        let slice = &self.source[self.start + 1..self.current - 1];
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

    fn identifier_str(&mut self) -> String {
        while self::Scanner::is_identifier(self.peek()) {
            self.advance();
        }

        let slice = &self.source[self.start..self.current];
        slice.into_iter().collect()
    }

    fn identifier_or_keyword(&mut self) -> Token {
        let id = self.identifier_str();
        match id.as_str() {
            "and" => self.build_token(TokenType::And),
            "class" => self.build_token(TokenType::Class),
            "else" => self.build_token(TokenType::Else),
            "false" => self.build_token(TokenType::False),
            "for" => self.build_token(TokenType::For),
            "fun" => self.build_token(TokenType::Fun),
            "if" => self.build_token(TokenType::If),
            "nil" => self.build_token(TokenType::Nil),
            "or" => self.build_token(TokenType::Or),
            "print" => self.build_token(TokenType::Print),
            "return" => self.build_token(TokenType::Return),
            "super" => self.build_token(TokenType::Super),
            "this" => self.build_token(TokenType::This),
            "true" => self.build_token(TokenType::True),
            "var" => self.build_token(TokenType::Var),
            "while" => self.build_token(TokenType::While),
            _ => self.build_token(TokenType::Identifier),
        }
    }

    fn multiline_comment(&mut self) -> Result<Option<Token>, Error> {
        let mut nesting = 1;
        while nesting > 0 {
            // println!("nesting: {}, peek: {}, peek_next: {}", nesting, self.peek(), self.peek_next());
            if self.peek() == '\0' {
                let error_message = "Unterminated Multiline Comment";
                error_line(self.line, error_message);
                return Result::Err(Error::Scan { message: error_message.to_string(), line: self.line });
            }
            if self.peek() == '/' && self.peek_next() == '*' {
                self.advance();
                self.advance();
                nesting += 1;
                continue;
            }
            if self.peek() == '*' && self.peek_next() == '/' {
                self.advance();
                self.advance();
                nesting -= 1;
                continue;
            }
        }
        Result::Ok(None)
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
        self.push_token(Token::new(token_type, lexeme, literal, self.line), result);
    }

    fn build_token(&mut self, token_type: TokenType) -> Token {
        self.build_token_value(token_type, None)
    }

    fn build_token_value(&mut self, token_type: TokenType, literal: Option<Value>) -> Token {
        let slice = &self.source[self.start..self.current];
        let lexeme = slice.into_iter().collect();
        Token::new(token_type, lexeme, literal, self.line)
    }

    fn push_token(&self, token: Token, result: &mut Vec<Token>) {
        println!("Pushing token {:?}", token);
        result.push(token);
    }
}

#[cfg(test)]
mod tests {
    use crate::token::TokenType;
    #[test]
    fn random_tokens() {
        let source = "(){},.-+;*!23!=42.42/*.block.\n.comment.*/==<<==>/>=\"foo \nbar\"// this is a comment now";
        let mut scanner = super::Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        for (i, v) in [
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::Comma,
            TokenType::Dot,
            TokenType::Minus,
            TokenType::Plus,
            TokenType::Semicolon,
            TokenType::Star,
            TokenType::Bang,
            TokenType::Number,
            TokenType::BangEqual,
            TokenType::Number,
            TokenType::EqualEqual,
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Equal,
            TokenType::Greater,
            TokenType::Slash,
            TokenType::GreaterEqual,
            TokenType::String,
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&tokens[i].token_type, v);
        }

        assert_eq!(tokens[12].lexeme, "!=");
    }
}