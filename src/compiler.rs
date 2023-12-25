use std::mem;

use crate::{
    chunk::{Chunk, OpCode},
    debug::disassemble_chunk,
    obj::Obj,
    scanner::Scanner,
    token::{Token, TokenType},
    value::Value,
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
    prefix: Option<fn(&mut Parser, bool, &mut Chunk)>,
    infix: Option<fn(&mut Parser, bool, &mut Chunk)>,
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
                prefix: Some(Parser::variable),
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

        while !self.match_(TokenType::Eof) {
            self.declaration(chunk);
        }

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

    fn var_declaration(&mut self, chunk: &mut Chunk) {
        let global = self.parse_variable(chunk, "Expect variable name.");
        
        if self.match_(TokenType::Equal) {
            self.expression(chunk);
        } else {
            self.emit_byte(OpCode::OpNil.into(), chunk);
        }

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.");

        self.define_variable(global, chunk);
    }

    fn expression_statement(&mut self, chunk: &mut Chunk) {
        self.expression(chunk);
        self.consume(TokenType::Semicolon, "Expect ';' after expression");
        self.emit_byte(OpCode::OpPop.into(), chunk);
    }

    fn print_statement(&mut self, chunk: &mut Chunk) {
        self.expression(chunk);
        self.consume(TokenType::Semicolon, "Expect ';' after value");
        self.emit_byte(OpCode::OpPrint.into(), chunk)
    }

    fn synchronize(&mut self) {
        self.panic_mode = false;

        while self.current.token_type != TokenType::Eof {
            if self.previous.token_type == TokenType::Semicolon {
                return;
            }
            match self.current.token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }
            self.advance();
        }
    }

    fn declaration(&mut self, chunk: &mut Chunk) {
        if self.match_(TokenType::Var) {
            self.var_declaration(chunk);
        } else {
            self.statement(chunk);
        }

        if self.panic_mode {
            self.synchronize();
        }
    }

    fn statement(&mut self, chunk: &mut Chunk) {
        if self.match_(TokenType::Print) {
            self.print_statement(chunk);
        } else {
            self.expression_statement(chunk);
        }
    }

    fn number(&mut self, can_assign: bool, chunk: &mut Chunk) {
        let value = Value::from(self.previous.lexeme.parse::<f64>().unwrap());
        self.emit_constant(value, chunk);
    }

    fn string(&mut self, can_assign: bool, chunk: &mut Chunk) {
        let str_len = self.previous.lexeme.len();
        let string = &self.previous.lexeme[1..str_len - 1];
        let value = Value::from(Obj::from(string.to_string()));
        self.emit_constant(value, chunk);
    }

    fn named_variable(&mut self, name: String, can_assign: bool, chunk: &mut Chunk) {
        let arg = self.identifier_constant(name, chunk);

        if can_assign && self.match_(TokenType::Equal) {
            self.expression(chunk);
            self.emit_bytes(OpCode::OpSetGlobal.into(), arg, chunk);
        } else {
            self.emit_bytes(OpCode::OpGetGlobal.into(), arg, chunk);
        }
    }

    fn variable(&mut self, can_assign: bool, chunk: &mut Chunk) {
        let name = self.previous.lexeme.clone();
        self.named_variable(name, can_assign, chunk);
    }

    fn grouping(&mut self, can_assign: bool, chunk: &mut Chunk) {
        self.expression(chunk);
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self, can_assign: bool, chunk: &mut Chunk) {
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

        let can_assign = precedence <= Precedence::Assignment;
        prefix_rule(self, can_assign, chunk);

        while precedence <= ParseRule::get_rule(&self.current.token_type).precedence {
            self.advance();
            let infix_rule = ParseRule::get_rule(&self.previous.token_type)
                .infix
                .unwrap();
            infix_rule(self, can_assign, chunk);
        }

        if can_assign && self.match_(TokenType::Equal) {
            self.error("Invalid assignment target.");
        }
    }

    fn parse_variable(&mut self, chunk: &mut Chunk, error_message: &str) -> u8 {
        self.consume(TokenType::Identifier, error_message);
        let token_name = (&self.previous.lexeme).to_string();
        let constant_index = self.identifier_constant(token_name, chunk);
        return constant_index;
    }

    fn define_variable(&mut self, global: u8, chunk: &mut Chunk) {
        self.emit_bytes(OpCode::OpDefineGlobal.into(), global, chunk);
    }

    fn identifier_constant(&mut self, name: String, chunk: &mut Chunk) -> u8 {
        return self.make_constant(Value::from(Obj::from(name)), chunk)
    }

    fn binary(&mut self, can_assign: bool, chunk: &mut Chunk) {
        let operator_type = self.previous.token_type.clone();
        let parse_rule = ParseRule::get_rule(&operator_type);
        self.parse_precedence(parse_rule.precedence.greater(), chunk);

        match operator_type {
            TokenType::BangEqual => {
                self.emit_bytes(OpCode::OpEqual.into(), OpCode::OpNot.into(), chunk)
            }
            TokenType::EqualEqual => self.emit_byte(OpCode::OpEqual.into(), chunk),
            TokenType::Greater => self.emit_byte(OpCode::OpGreater.into(), chunk),
            TokenType::GreaterEqual => {
                self.emit_bytes(OpCode::OpLess.into(), OpCode::OpNot.into(), chunk)
            }
            TokenType::Less => self.emit_byte(OpCode::OpLess.into(), chunk),
            TokenType::LessEqual => {
                self.emit_bytes(OpCode::OpGreater.into(), OpCode::OpNot.into(), chunk)
            }
            TokenType::Plus => self.emit_byte(OpCode::OpAdd.into(), chunk),
            TokenType::Minus => self.emit_byte(OpCode::OpSubtract.into(), chunk),
            TokenType::Star => self.emit_byte(OpCode::OpMultiply.into(), chunk),
            TokenType::Slash => self.emit_byte(OpCode::OpDivide.into(), chunk),
            _ => {}
        };
    }

    fn literal(&mut self, can_assign: bool, chunk: &mut Chunk) {
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
        self.emit_bytes(OpCode::OpConstant.into(), constant_index, chunk)
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

    fn match_(&mut self, token_type: TokenType) -> bool {
        if !self.check(token_type) {
            return false;
        }
        self.advance();
        return true;
    }

    fn check(&self, token_type: TokenType) -> bool {
        self.current.token_type == token_type
    }
}
