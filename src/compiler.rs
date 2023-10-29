use std::mem;

use crate::{
    chunk::{Chunk, OpCode},
    scanner::Scanner,
    token::{Token, TokenType}, debug::disassemble_chunk, value::{Value, Obj},
};


#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    fn greater(self) -> Self {
        match self {
            Precedence::None => Self::Assignment,
            Precedence::Assignment => Self::Or,
            Precedence::Or => Self::And,
            Precedence::And => Self::Equality,
            Precedence::Equality => Self::Comparison,
            Precedence::Comparison => Self::Term,
            Precedence::Term => Self::Factor,
            Precedence::Factor => Self::Unary,
            Precedence::Unary => Self::Call,
            Precedence::Call => Self::Primary,
            Precedence::Primary => Self::Primary,
        }
    }
}

struct ParseRule {
    prefix: Option<fn(&mut Parser, &mut Chunk)>,
    infix: Option<fn(&mut Parser, &mut Chunk)>,
    precedence: Precedence,
}

impl ParseRule {
    fn get_rule(token_type: &TokenType) -> ParseRule {
        match token_type {
            TokenType::LeftParen => Self {
                prefix: Some(Parser::grouping),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::RightParen => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::LeftBrace => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::RightBrace => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Comma => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Dot => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Minus => Self {
                prefix: Some(Parser::unary),
                infix: Some(Parser::binary),
                precedence: Precedence::Term,
            },
            TokenType::Plus => Self {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Term,
            },
            TokenType::Semicolon => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Slash => Self {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Factor,
            },
            TokenType::Star => Self {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Factor,
            },
            TokenType::Bang => Self {
                prefix: Some(Parser::unary),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::BangEqual => Self {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Comparison,
            },
            TokenType::Equal => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::EqualEqual => Self {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Comparison,
            },
            TokenType::Greater => Self {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Comparison,
            },
            TokenType::GreaterEqual => Self {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Comparison,
            },
            TokenType::Less => Self {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Comparison,
            },
            TokenType::LessEqual => Self {
                prefix: None,
                infix: Some(Parser::binary),
                precedence: Precedence::Comparison,
            },
            TokenType::Identifier => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::String => Self {
                prefix: Some(Parser::string),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Number => Self {
                prefix: Some(Parser::number),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::And => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Class => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Else => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::False => Self {
                prefix: Some(Parser::literal),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Fun => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::For => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::If => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Nil => Self {
                prefix: Some(Parser::literal),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Or => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Print => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Return => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Super => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::This => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::True => Self {
                prefix: Some(Parser::literal),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Var => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::While => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Eof => Self {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        }
    }
}

pub struct Parser {
    current: Token,
    previous: Token,
    scanner: Scanner,
    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    pub fn new(source: String) -> Parser {
        Parser {
            current: Token::default(),
            previous: Token::default(),
            scanner: Scanner::new(source),
            had_error: false,
            panic_mode: false,
        }
    }

    fn advance(&mut self) {
        self.previous = mem::take(&mut self.current);
        loop {
            match self.scanner.scan_token() {
                Ok(token) => {
                    self.current = token;
                    break;
                }
                Err(e) => self.error_at_current(&e.message),
            }
        }
    }

    pub fn compile(&mut self, chunk: &mut Chunk) -> Result<(), ()> {
        self.advance();
        self.expression(chunk);
        self.consume(TokenType::Eof, "Expected end of expression");
        self.emit_return(chunk);
        if cfg!(feature = "DEBUG_PRINT_CODE") {
            if !self.had_error {
                disassemble_chunk(chunk, "code");
            }
        }
        if self.had_error {
            Err(())
        } else {
            Ok(())
        }
    }

    fn expression(&mut self, chunk: &mut Chunk) {
        self.parse_precedence(Precedence::Assignment, chunk);
    }

    fn number(&mut self, chunk: &mut Chunk) {
        let value = Value::Number(self.previous.lexeme.parse::<f64>().unwrap());
        self.emit_constant(value, chunk);
    }

    fn string(&mut self, chunk: &mut Chunk) {
        let str_len = self.previous.lexeme.len();
        let string = &self.previous.lexeme[1..str_len - 1];
        let value = Value::Str(Box::new(string.to_string()));
        self.emit_constant(value, chunk);
    }

    fn grouping(&mut self, chunk: &mut Chunk) {
        self.expression(chunk);
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self, chunk: &mut Chunk) {
        let operator_type = self.previous.token_type.clone();
        self.parse_precedence(Precedence::Unary, chunk);
        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::OpNegate.into(), chunk),
            TokenType::Bang => self.emit_byte(OpCode::OpNot.into(), chunk),
            _ => {}
        };
    }

    fn parse_precedence(&mut self, precedence: Precedence, chunk: &mut Chunk) {
        self.advance();
        let parse_rule = ParseRule::get_rule(&self.previous.token_type);
        let prefix_rule_option = parse_rule.prefix;
        if prefix_rule_option.is_none() {
            self.error("Expect expression.");
            return;
        }

        let prefix_rule = prefix_rule_option.unwrap();

        prefix_rule(self, chunk);

        while precedence <= ParseRule::get_rule(&self.current.token_type).precedence {
            self.advance();
            let infix_rule = ParseRule::get_rule(&self.previous.token_type).infix.unwrap();
            infix_rule(self, chunk);
        }
    }

    fn binary(&mut self, chunk: &mut Chunk) {
        let operator_type = self.previous.token_type.clone();
        let parse_rule = ParseRule::get_rule(&operator_type);
        self.parse_precedence(parse_rule.precedence.greater(), chunk);

        match operator_type {
            TokenType::BangEqual => self.emit_bytes(OpCode::OpEqual.into(), OpCode::OpNot.into(), chunk),
            TokenType::EqualEqual => self.emit_byte(OpCode::OpEqual.into(), chunk),
            TokenType::Greater => self.emit_byte(OpCode::OpGreater.into(), chunk),
            TokenType::GreaterEqual => self.emit_bytes(OpCode::OpLess.into(), OpCode::OpNot.into(), chunk),
            TokenType::Less => self.emit_byte(OpCode::OpLess.into(), chunk),
            TokenType::LessEqual => self.emit_bytes(OpCode::OpGreater.into(), OpCode::OpNot.into(), chunk),
            TokenType::Plus => self.emit_byte(OpCode::OpAdd.into(), chunk),
            TokenType::Minus => self.emit_byte(OpCode::OpSubtract.into(), chunk),
            TokenType::Star => self.emit_byte(OpCode::OpMultiply.into(), chunk),
            TokenType::Slash => self.emit_byte(OpCode::OpDivide.into(), chunk),
            _ => {}
        };
    }

    fn literal(&mut self, chunk: &mut Chunk) {
        match self.previous.token_type {
            TokenType::Nil => self.emit_byte(OpCode::OpNil.into(), chunk),
            TokenType::True => self.emit_byte(OpCode::OpTrue.into(), chunk),
            TokenType::False => self.emit_byte(OpCode::OpFalse.into(), chunk),
            _ => {}
        };
    }

    fn emit_byte(&mut self, byte: u8, chunk: &mut Chunk) {
        chunk.write(byte, self.current.line);
    }

    fn emit_constant(&mut self, constant: Value, chunk: &mut Chunk) {
        let constant_index = self.make_constant(constant, chunk);
        self.emit_bytes(
            OpCode::OpConstant.into(),
            constant_index,
            chunk,
        )
    }

    fn make_constant(&mut self, constant: Value, chunk: &mut Chunk) -> u8 {
        let index = chunk.add_constant(constant);
        if index as u8 > u8::MAX {
            self.error("Too many constants in one chunk.");
            return 0;
        }
        return index as u8;
    }

    fn emit_bytes(&self, byte1: u8, byte2: u8, chunk: &mut Chunk) {
        chunk.write(byte1, self.current.line);
        chunk.write(byte2, self.current.line);
    }

    fn emit_return(&mut self, chunk: &mut Chunk) {
        self.emit_byte(OpCode::OpReturn.into(), chunk);
    }

    fn error(&mut self, message: &str) {
        self.error_at(&self.previous.clone(), false, message);
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.current.clone(), true, message);
    }

    fn error_at(&mut self, token: &Token, is_error: bool, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        eprint!("[line {}] Error", token.line);
        if token.token_type == TokenType::Eof {
            eprint!(" at end");
        } else if is_error {
            // Don't do anything
        } else {
            eprint!(" at '{}'", token.lexeme);
        }
        eprintln!(" {}", message);
        self.had_error = true;
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }
}
