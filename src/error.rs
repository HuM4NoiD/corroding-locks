pub fn error_line(line_num: i32, message: &str) {
    report(line_num, "", message)
}

pub fn report(line_num: i32, whr: &str, message: &str) {
    println!("[line {}] Error {}: {}", line_num, whr, message)
}
