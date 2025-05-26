use std::usize;

use crate::error;
use crate::token::{self, Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    curr: i32,
    line: i32,
    is_error: bool,
}
impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            is_error: false,
            tokens: vec![],
            start: 0,
            curr: 0,
            line: 1,
        }
    }
    fn is_at_end(&self) -> bool {
        self.curr >= self.source.len() as i32
    }
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while !self.is_at_end() {
            self.start = self.curr;
            self.scan_tokens();
        }

        tokens.push(Token {
            lexeme: "".to_string(),
            token_type: TokenType::EOF,
            literal: None,
            line: self.line,
        });
        tokens
    }
    fn get_char_from_source(&self, idx: usize) -> char {
        if self.is_at_end() {
            return '\0';
        };
        return self.source.chars().nth(idx).unwrap();
    }
    fn advance(&mut self) -> char {
        let c = self.get_char_from_source(self.curr as usize);

        self.curr += 1;

        c
    }
    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LEFTPAREN, None),
            ')' => self.add_token(TokenType::RIGHTPAREN, None),
            '{' => self.add_token(TokenType::LEFTBRACE, None),
            '}' => self.add_token(TokenType::RIGHTBRACE, None),
            ',' => self.add_token(TokenType::COMMA, None),
            '.' => self.add_token(TokenType::DOT, None),
            '-' => self.add_token(TokenType::MINUS, None),
            '+' => self.add_token(TokenType::PLUS, None),
            ';' => self.add_token(TokenType::SEMICOLON, None),
            '*' => self.add_token(TokenType::STAR, None),
            '!' => {
                let mut curr_type = TokenType::BANG;
                match self.match_char('=') {
                    true => curr_type = TokenType::BANGEQUAL,
                    _ => {}
                }
                self.add_token(curr_type, None);
            }
            '=' => {
                let mut curr_type = TokenType::EQUAL;
                match self.match_char('=') {
                    true => curr_type = TokenType::EQUALEQUAL,
                    _ => {}
                }
                self.add_token(curr_type, None);
            }

            '<' => {
                let mut curr_type = TokenType::LESS;
                match self.match_char('=') {
                    true => curr_type = TokenType::LESSEQUAL,
                    _ => {}
                }
                self.add_token(curr_type, None);
            }

            '>' => {
                let mut curr_type = TokenType::GREATER;
                match self.match_char('=') {
                    true => curr_type = TokenType::GREATEREQUAL,
                    _ => {}
                }
                self.add_token(curr_type, None);
            }
            '/' => match self.match_char('/') {
                true => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                false => self.add_token(TokenType::SLASH, None),
            },
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string_to_end(),
            _ => error::error(self.line, "Unexpected character.".to_string()),
        }
    }
    fn string_to_end(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            error::error(self.line, "Unterminated string.".to_string());
            return;
        }

        self.advance();
        let curr_str = self.sub_string(Some(self.start as usize + 1), Some(self.curr as usize - 1));
        self.add_token(TokenType::STRING, Some(curr_str));
    }
    fn sub_string(&self, start: Option<usize>, end: Option<usize>) -> String {
        if start.is_some() && end.is_some(){
            return self.source[start.unwrap()..end.unwrap()].to_string();
        }
        self.source[self.start as usize..self.curr as usize].to_string()
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        };
        self.get_char_from_source(self.curr as usize)
    }
    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.sub_string(None, None);
        self.tokens.push(Token {
            lexeme: text,
            token_type,
            literal,
            line: self.line,
        })
    }
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.get_char_from_source(self.curr as usize) != expected {
            return false;
        }
        self.curr += 1;
        return true;
    }
}
