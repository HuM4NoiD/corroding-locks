use std::{
    fs,
    io::{self, BufRead, Write},
};

pub struct Lox {
    args: Vec<String>,
}

impl Lox {
    pub fn new(args: Vec<String>) -> Self {
        Lox { args }
    }

    pub fn run_main(&self) {
        if self.args.len() > 2 {
            println!("Usage: lox [script]");
        } else if self.args.len() == 2 {
            self.run_file(&self.args[1]);
        } else {
            self.run_prompt();
        }
    }

    fn run_file(&self, file_name: &str) {
        let contents = fs::read_to_string(file_name).expect("Could not read the source file");
        self.run(contents)
    }

    fn run_prompt(&self) {
        let stdin = io::stdin();
        print!("> ");
        let _ = io::stdout().flush();
        let mut lines = stdin.lock().lines();
        while let Some(line) = lines.next() {
            match line {
                Ok(line_source) => self.run(line_source),
                Err(e) => {
                    println!("Error reading line, {}", e);
                }
            }
            print!("> ");
            let _ = io::stdout().flush();
        }
    }

    fn run(&self, source: String) {
        println!("Got Source:\n{}", source)
    }
}
