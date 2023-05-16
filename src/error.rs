use crate::token::Token;

pub fn error_line(line_num: u32, message: &str) {
    report(line_num, "", message)
}

pub fn report(line_num: u32, whr: &str, message: &str) {
    println!("[line {}] Error {}: {}", line_num, whr, message)
}

#[derive(Debug)]
pub enum Error {
    Scan { message: String, line: u32 },
    Parse { message: String, token: Token },
}

pub fn report_err(error: &Error) {
    match error {
        Error::Scan { message, line } => report_scan_err(message, line.to_owned()),
        Error::Parse { message, token } => report_parse_err(message, token),
    }
}

pub fn report_scan_err(message: &str, line: u32) {
    println!("[line {}] Error {}: {}", line, "", message)
}

pub fn report_parse_err(message: &str, token: &Token) {
    println!("Parse error for token: {}, {}", token, message);
}
