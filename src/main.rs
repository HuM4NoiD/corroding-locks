use std::{fs, io::{self, BufRead, Write}, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("all args: {:?}", args);
    if args.len() > 2 {
        println!("Usage: lox [script]");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(file_name: &str) {
    let contents = fs::read_to_string(file_name).expect("Could not read the source file");
    run(contents)
}

fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    let _ = io::stdout().flush();
    let mut lines = stdin.lock().lines();
    while let Some(line) = lines.next() {
        match line {
            Ok(line_source) => {
                run(line_source)
            }
            Err(e) => {
                println!("Error reading line, {}", e);
            }
        }
        print!("> ");
        let _ = io::stdout().flush();
    }
}

fn run(source: String) {
    println!("Got Source:\n{}", source)
}
