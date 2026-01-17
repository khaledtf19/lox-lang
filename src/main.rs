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
mod scanner;
mod stmt;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let expression = Expr::binary(
    //     Expr::unary(
    //         Token::new(TokenType::MINUS, "-".to_string(), None, 1),
    //         Expr::literal(LiteralValue::Number(123.0)),
    //     ),
    //     Token::new(TokenType::STAR, "*".to_string(), None, 1),
    //     Expr::grouping(Expr::literal(LiteralValue::Number(45.67))),
    // );
    let mut lox = Lox::new();

    if args.len() <= 1 {
        println!("Usage: rlox [script]");
        return;
    } else if args.len() == 2 {
        lox.run_file(&args[1])
            .expect("Someting went wrong while reading");
        return;
    } else {
        lox.run_prompt();
    }
}
