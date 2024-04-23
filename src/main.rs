// TODO: Create lexer errors
// TODO: Create parser errors
// TODO: Create scanner errors
// TODO: Create interpreter errors
// BUG: If statement not working

use std::env::args;
use std::io::{self, stdout, BufRead, Write};
mod errors {
    pub(crate) mod error;
}
use parser::parser::Parser;
use scanner::scanner::Scanner;
mod tokens {
    pub(crate) mod token;
    pub(crate) mod token_type;
}
mod ast {
    pub(crate) mod expr;
    pub mod stmt;
}

mod parser {
    pub(crate) mod helpers;
    pub(crate) mod lookups;
    pub(crate) mod parser;
}
mod object {
    pub(crate) mod object;
}
mod scanner {
    pub(crate) mod scanner;
}

struct Cedar {}

impl Cedar {
    pub fn new() -> Cedar {
        Cedar {}
    }

    fn run_file(&mut self, path: &str) -> io::Result<()> {
        let buf = std::fs::read_to_string(path)?;
        if self.run(buf, path.to_string()).is_err() {
            // Ignore: error was already reported
            std::process::exit(65);
        }

        Ok(())
    }

    fn run_prompt(&mut self) {
        let stdin = io::stdin();
        print!("> ");
        let _ = stdout().flush();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    break;
                }
                let _ = self.run(line, "Prompt".to_string());
            } else {
                break;
            }
            print!("> ");
            let _ = stdout().flush();
        }
    }

    fn run(&mut self, source: String, file_name: String) -> Result<(), ()> {
        let mut scanner = Scanner::new(source.chars().collect(), file_name.to_string());
        let tokens = scanner.scan_tokens();

        // Ok(for token in tokens.unwrap().clone() {
        //     println!("{:?}", token);
        // })

        // let expr = parse(tokens.unwrap().clone());

        let mut parser = Parser::new(tokens.unwrap().clone());
        let result = parser.parse();

        for res in result {
            println!("{}", res);
        }

        Ok(())

        // match parser.parse() {
        //     Ok(statements) => {
        //         self.interpreter.interpret(&statements);
        //     }
        //     Err(error) => error.report(),
        // }
        // Ok(())
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let mut cedar = Cedar::new();
    //println!("{}", args[1]);
    match args.len() {
        1 => cedar.run_prompt(),
        2 => cedar.run_file(&args[1]).expect("Could not run file"),
        _ => {
            println!("Usage: lox-ast [script]");
            std::process::exit(64);
        }
    }
}

// struct KayLan {
//     had_error: bool,
// }
