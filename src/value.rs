#[derive(Debug)]
pub enum Value {
    Boolean(bool),
    Number(f64),
    String(String),
    Nil(),
}
