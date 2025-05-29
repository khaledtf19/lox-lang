use std::usize;

use crate::error::LoxError;
use crate::token::{Token, TokenLiteral, TokenType, parse_keyword};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    curr: usize,
    line: usize,

    pub is_error: bool,
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
        self.curr >= self.source.len()
    }
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.curr;
            self.scan_token();
        }

        self.tokens.push(Token {
            lexeme: "".to_string(),
            token_type: TokenType::EOF,
            literal: None,
            line: self.line,
        });
        self.tokens.clone()
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
            '/' => {
                let next = self.peek();
                if next == '/' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if next == '*' {
                    while self.peek() != '/' && !self.is_at_end() {
                        if self.peek() == '\n' {
                            self.line += 1;
                        }
                        self.advance();
                    }
                    self.advance();
                } else {
                    self.add_token(TokenType::SLASH, None);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string_to_end(),
            _ => {
                if c.is_numeric() {
                    self.number_to_end();
                } else if self.is_alph(c) {
                    self.identifier_to_end();
                } else {
                    self.is_error = true;
                    LoxError::error(self.line, "Unexpected character.".to_string());
                }
            }
        }
    }
    fn identifier_to_end(&mut self) {
        while self.is_alph_numeric(self.peek()) {
            self.advance();
        }
        let text = self.sub_string(Some(self.start), Some(self.curr));
        let mut curr_type = parse_keyword(&text);
        if curr_type.is_none() {
            curr_type = Some(TokenType::IDENTIFIER)
        }

        self.add_token(curr_type.unwrap(), None);
    }
    fn string_to_end(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() && self.get_char_from_source(self.curr as usize) != '"' {
            self.is_error = true;
            LoxError::error(self.line, "Unterminated string.".to_string());
            return;
        }

        self.advance();
        let curr_str = self.sub_string(Some(self.start as usize + 1), Some(self.curr as usize - 1));

        self.add_token(TokenType::STRING, Some(TokenLiteral::Text(curr_str)));
    }

    fn number_to_end(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        let num = self.sub_string(Some(self.start), Some(self.curr));
        self.add_token(
            TokenType::NUMBER,
            Some(TokenLiteral::Float(num.parse().unwrap())),
        );
    }

    fn peek_next(&self) -> char {
        if self.curr + 1 >= self.source.len() {
            return '\0';
        }
        self.get_char_from_source(self.curr + 1)
    }

    fn is_alph_numeric(&self, c: char) -> bool {
        self.is_digit(c) || self.is_alph(c)
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }
    fn is_alph(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn sub_string(&self, start: Option<usize>, end: Option<usize>) -> String {
        if start.is_some() && end.is_some() {
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
    fn add_token(&mut self, token_type: TokenType, literal: Option<TokenLiteral>) {
        let text = self.sub_string(Some(self.start), Some(self.curr));

        println!("{}", text);
        self.tokens.push(Token {
            lexeme: text,
            token_type,
            literal,
            line: self.line,
        });
    }
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.get_char_from_source(self.curr) != expected {
            return false;
        }
        self.curr += 1;
        return true;
    }
}
