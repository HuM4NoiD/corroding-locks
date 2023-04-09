use std::env;
pub mod error;
pub mod lox;
pub mod scanner;
pub mod token;
pub mod value;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("all args: {:?}", args);
    let instance = lox::Lox::new(args);
    instance.run_main();
}
