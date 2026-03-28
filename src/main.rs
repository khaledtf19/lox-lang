// to remove the warnings
// #![allow(warnings)]

use std::env::{self};

use lox::Lox;

mod Environment;
mod error;
mod expr;
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
        lox.run_prompt().expect("Someting went wrong");
    } else {
        lox.run_file(&args[args.len() - 1])
            .expect("Someting went wrong while reading");
    }
}
