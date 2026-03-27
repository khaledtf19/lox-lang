// to remove the warnings
// #![allow(warnings)]

use std::env::{self};

use lox::Lox;

mod Environment;
mod ast;
mod error;
mod interpreter;
mod lox;
mod lox_callable;
mod lox_function;
mod parser;
mod resolver;
mod scanner;
mod stmt;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut lox = Lox::new();

    if args.len() <= 1 {
        println!("Usage: rlox [script]");
        return;
    } else if args.len() == 2 {
        lox.run_file(&args[1])
            .expect("Someting went wrong while reading");
        return;
    } else {
        let _ = lox.run_prompt();
    }
}
