use crate::{
    scanner::Scanner, token::TokenType
};

pub fn compile(source: String) {
    let mut scanner = Scanner::new(source);
    let mut line: u32 = 0;
    let mut first = true;
    loop {
        let result = scanner.scan_token();
        if let Ok(Some(token)) = result {
            if first {
                first = false;
                print!("{:4} ", token.line);
                line = token.line;
            } else if line != token.line {
                print!("{:4} ", token.line);
                line = token.line;
            } else {
                print!("   | ");
            }
            println!("{:?} {}", token.token_type, token.lexeme);
            if token.token_type == TokenType::Eof {
                break;
            }
        }
    }
}
