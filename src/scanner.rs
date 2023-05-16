use crate::{
    error::{error_line, report_err, Error},
    token::{Token, TokenType},
    value::Value,
};
use std::iter::Iterator;

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

    pub fn reset_with_source(&mut self, source: String) {
        self.source = source.chars().collect();
        self.start = 0;
        self.current = 0;
        self.line = 1;
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        let mut errors = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(Some(token)) => tokens.push(token),
                Ok(None) => {}
                Err(e) => errors.push(e),
            }
        }

        tokens
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<Option<Token>, Error> {
        let character = self.advance();
        match character {
            '(' => self.build_token(TokenType::LeftParen),
            ')' => self.build_token(TokenType::RightParen),
            '{' => self.build_token(TokenType::LeftBrace),
            '}' => self.build_token(TokenType::RightBrace),
            ',' => self.build_token(TokenType::Comma),
            '.' => self.build_token(TokenType::Dot),
            ';' => self.build_token(TokenType::Semicolon),
            '-' => self.build_token(TokenType::Minus),
            '+' => self.build_token(TokenType::Plus),
            '*' => self.build_token(TokenType::Star),
            '!' => {
                if self.match_('=') {
                    self.build_token(TokenType::BangEqual)
                } else {
                    self.build_token(TokenType::Bang)
                }
            }
            '=' => {
                println!("for equal, current: {}, peek: {}", character, self.peek());
                if self.match_('=') {
                    self.build_token(TokenType::EqualEqual)
                } else {
                    self.build_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_('=') {
                    self.build_token(TokenType::LessEqual)
                } else {
                    self.build_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_('=') {
                    self.build_token(TokenType::GreaterEqual)
                } else {
                    self.build_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.match_('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Result::Ok(None)
                } else if self.match_('*') {
                    self.multiline_comment()
                } else {
                    self.build_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => Result::Ok(None),
            '\n' => {
                self.line += 1;
                Result::Ok(None)
            }
            '"' => self.string(),
            _ => {
                if character.is_digit(10) {
                    self.number()
                } else if character.is_alphabetic() {
                    self.identifier_or_keyword()
                } else {
                    Result::Err(Error::Scan {
                        message: format!("Unexpected character: {}", character),
                        line: self.line,
                    })
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

    fn string(&mut self) -> Result<Option<Token>, Error> {
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
        self.build_token_value(TokenType::String, value)
    }

    fn number(&mut self) -> Result<Option<Token>, Error> {
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

    fn identifier_or_keyword(&mut self) -> Result<Option<Token>, Error> {
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
            println!(
                "nesting: {}, peek: {}, peek_next: {}",
                nesting,
                self.peek(),
                self.peek_next()
            );
            if self.peek() == '\0' {
                let error_message = "Unterminated Multiline Comment";
                error_line(self.line, error_message);
                return Result::Err(Error::Scan {
                    message: error_message.to_string(),
                    line: self.line,
                });
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
            self.advance();
        }
        Result::Ok(None)
    }

    fn is_identifier(character: char) -> bool {
        character.is_alphanumeric() || character == '_'
    }

    fn build_token(&mut self, token_type: TokenType) -> Result<Option<Token>, Error> {
        self.build_token_value(token_type, None)
    }

    fn build_token_value(
        &mut self,
        token_type: TokenType,
        literal: Option<Value>,
    ) -> Result<Option<Token>, Error> {
        let slice = &self.source[self.start..self.current];
        let lexeme = slice.into_iter().collect();
        let res = Result::Ok(Some(Token::new(token_type, lexeme, literal, self.line)));
        println!("Created token: {:?}", res);
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::token::TokenType;
    #[test]
    fn random_tokens() {
        //Shamelessly stolen from https://github.com/abesto/jlox-rs/blob/main/src/scanner.rs
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
