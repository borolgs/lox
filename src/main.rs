mod ast;
mod error;
mod interpreter;
mod parser;
mod scanner;
mod token;

use error::LoxError;
use interpreter::{Interpreter, IntrError, IntrResult};
use parser::Parser;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    println!("Args: {:?}", args);
    match args.len() {
        1 => run_prompt(),
        2 => run_file(args[1].as_str()),
        _ => help(),
    }
}

fn run_prompt() -> anyhow::Result<()> {
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

fn run_file(filename: &str) -> anyhow::Result<()> {
    let source = std::fs::read_to_string(filename).expect("Could not read file");
    run(&source);
    Ok(())
}

fn help() -> anyhow::Result<()> {
    println!("Usage: rlox [script]");
    Ok(())
}

fn run(source: &str) {
    let res = interpret(source);
    match res {
        Ok(res) => println!("{:?}", res),
        Err(err) => match err {
            LoxError::ParseError(_) => todo!(),
            LoxError::RuntimeError(IntrError::Unsupported(token)) => {
                println!("Unsupported operation\n[line {}]", token.line)
            }
            LoxError::RuntimeError(IntrError::Runtime(token, message)) => {
                println!("{}\n[line {}]", message, token.line)
            }
            _ => todo!(),
        },
    }
}

fn interpret(input: &str) -> Result<IntrResult, LoxError> {
    let mut scanner = scanner::Scanner::new(input.into());
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens);
    let mut interpreter = Interpreter;

    let res = match parser.expression() {
        Ok(expr) => interpreter.evaluate(&expr)?,
        Err(err) => {
            return Err(err.into());
        }
    };

    Ok(res)
}
