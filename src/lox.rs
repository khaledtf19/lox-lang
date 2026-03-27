use std::{
    fs::File,
    io::{self, Read},
};
use text_io::read;

use crate::{interpreter::Interpreter, parser::parser::Parser, resolver::Resolver, scanner};

#[derive(Debug)]
pub struct Lox {
    pub interpretor: Interpreter,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            interpretor: Interpreter::new(),
        }
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

        let mut resolver = Resolver::new(&mut self.interpretor);

        if let Some(stmts) = statements {
            resolver.resolve_stmts(&stmts);
            if !resolver.had_error {
                self.interpretor.interpret(stmts);
            }
        }
    }
}
