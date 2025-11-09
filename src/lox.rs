use std::{
    fs::File,
    io::{self, Read},
};
use text_io::read;

use crate::{error::RunTimeError, interpreter::Interpreter, parser::parser::Parser, scanner};

#[derive(Debug)]
pub struct Lox {
    pub had_error: bool,
    pub had_runtime_error: bool,
    pub interpretor: Interpreter,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            had_error: false,
            had_runtime_error: false,
            interpretor: Interpreter::new(),
        }
    }

    pub fn run_time_error(err: RunTimeError) {
        println!("\n[line {} Error: {} ]", err.token.line, err.message);
        // self.hadError = true;
    }

    pub fn run_file(&mut self, file_name: &str) -> io::Result<()> {
        let mut file = File::open(file_name)?;
        let mut source = String::new();

        file.read_to_string(&mut source)?;

        self.run(source);
        return Ok(());
    }

    pub fn run_prompt(&mut self) -> io::Result<()> {
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
            self.run(line);
        }

        Ok(())
    }

    pub fn run(&mut self, source: String) {
        let mut scanner = scanner::Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        match statements {
            Some(stmts) => {
                self.interpretor.interpret(stmts);
            }
            None => {}
        }
    }
}
