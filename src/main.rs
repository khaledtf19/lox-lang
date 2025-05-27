use ast::expr::{BinaryExpr, Expr, LiteralValue};
use std::{
    env::{self},
    fs::File,
    io::{self, Read},
};
use text_io::read;
use token::{Token, TokenType};

mod ast;
mod error;
mod scanner;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    let expression = Expr::binary(
        Expr::unary(
            Token::new(TokenType::MINUS, "-".to_string(), None, 1),
            Expr::literal(LiteralValue::Number(123.0)),
        ),
        Token::new(TokenType::STAR, "*".to_string(), None, 1),
        Expr::grouping(Expr::literal(LiteralValue::Number(45.67))),
    );
    println!("{}", expression);

    if args.len() <= 1 {
        println!("Usage: rlox [script]");
        return;
    } else if args.len() == 2 {
        run_file(&args[1]).expect("Someting went wrong while reading");
        return;
    } else {
        run_prompt().expect("Someting went wrong while reading");
    }
    println!("Hello, world!");
}

fn run_file(file_name: &str) -> io::Result<()> {
    let mut bytes = File::bytes(File::open(file_name)?);
    println!("{}", file_name);
    return Ok(());
}

fn run_prompt() -> io::Result<()> {
    // let stop = false;

    loop {
        print!("> ");
        let line: String = read!("{}\n");
        if line.len() <= 0
            || line.is_empty()
            || line == "exit\r".to_string()
            || line == "\r".to_string()
        {
            break;
        }
        run(line);
    }

    Ok(())
}

fn run(source: String) {
    let mut scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens.iter() {
        println!("{:?}", token);
    }
}
