#[macro_use]
extern crate paste;

#[macro_use]
pub mod ast_macros;
pub mod ast;
pub mod error;
pub mod lox;
pub mod scanner;
pub mod token;
pub mod value;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("all args: {:?}", args);
    let instance = lox::Lox::new(args);
    instance.run_main();
}
