#[macro_use]
extern crate paste;

#[macro_use]
pub mod ast_macros;
pub mod ast;
pub mod compiler;
pub mod error;
pub mod scanner;
pub mod token;
pub mod value;
pub mod chunk;
pub mod debug;
#[macro_use]
pub mod vm;

use std::{result, env, process::ExitCode, io::{self, Write}, fs};

use crate::{
    chunk::{ Chunk, OpCode::* }, 
    debug::disassemble_chunk,
    value::{ VmValue },
    vm::{ InterpretError, VM }
};


fn main() -> ExitCode {
    let mut vm = VM::new();
    
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        repl(&mut vm)
    } else if args.len() == 2 {
        run_file(&mut vm, &args[1])
    } else {
        println!("Usage: lox-rust [path]");
        ExitCode::from(42)
    }
}

fn repl(vm: &mut VM) -> ExitCode {
    let stdin = io::stdin();
    loop {
        let mut buffer = String::new();
        print!("> ");
        let _ = io::stdout().flush();
        let _ = stdin.read_line(&mut buffer);
        println!("{}", &buffer);
        let _ = vm.interpret(buffer);
    }
}

fn run_file(vm: &mut VM, file_path: &str) -> ExitCode {
    let result = fs::read_to_string(file_path);
    match result {
        Ok(source) => {
            let result = vm.interpret(source);
            use InterpretError as IE;
            match result {
                Ok(_) => ExitCode::from(0),
                Err(e) => match e {
                    IE::CompileError => ExitCode::from(65),
                    IE::RuntimeError => ExitCode::from(70),
                }
            }
        },
        Err(e) => {
            println!("Could not open file {}. error: {:?}", file_path, e);
            ExitCode::from(74)
        },
    }
}

