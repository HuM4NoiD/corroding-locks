pub fn error_line(line_num: u32, message: &str) {
    report(line_num, "", message)
}

pub fn report(line_num: u32, whr: &str, message: &str) {
    println!("[line {}] Error {}: {}", line_num, whr, message)
}

#[derive(Debug)]
pub enum Error {
    Scan { message: String, line: u32 },
}

pub fn report_err(error: &Error) {
    match error {
        Error::Scan { message, line } => report_scan_err(message, line.to_owned()),
    }
}

pub fn report_scan_err(message: &str, line: u32) {
    println!("[line {}] Error {}: {}", line, "", message)
}
