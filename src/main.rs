mod ast;
mod error;
mod parser;
mod scanner;
mod token;

use anyhow::Result;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    println!("Args: {:?}", args);
    match args.len() {
        1 => run_prompt(),
        2 => run_file(args[1].as_str()),
        _ => help(),
    }
}

fn run_prompt() -> Result<()> {
    let lines = io::stdin().lock().lines();
    for line in lines {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            run(&line)
        }
    }
    Ok(())
}

fn run_file(filename: &str) -> Result<()> {
    let source = std::fs::read_to_string(filename).expect("Could not read file");
    run(&source);
    Ok(())
}

fn help() -> Result<()> {
    println!("Usage: rlox [script]");
    Ok(())
}

fn run(source: &str) {
    let mut scanner = scanner::Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = parser::Parser::new(&tokens);
    let expr = parser.expression().unwrap();
    println!("{:?}", expr);
}
