use std::{
    fmt::{self, Display},
    usize,
};

use phf::phf_map;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    //Ternary
    QUESTION,
    COLON,

    // One or two character tokens.
    BANG,
    BANGEQUAL,
    EQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESS,
    LESSEQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    BREAK,

    EOF,
}

// static map
static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and"    => TokenType::AND,
    "class"  => TokenType::CLASS,
    "else"   => TokenType::ELSE,
    "false"  => TokenType::FALSE,
    "for"    => TokenType::FOR,
    "fun"    => TokenType::FUN,
    "if"     => TokenType::IF,
    "nil"    => TokenType::NIL,
    "or"     => TokenType::OR,
    "print"  => TokenType::PRINT,
    "return" => TokenType::RETURN,
    "super"  => TokenType::SUPER,
    "this"   => TokenType::THIS,
    "true"   => TokenType::TRUE,
    "var"    => TokenType::VAR,
    "while"  => TokenType::WHILE,
    "break" => TokenType::BREAK
};
pub fn parse_keyword(keyword: &str) -> Option<TokenType> {
    KEYWORDS.get(keyword).cloned()
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum TokenLiteral {
    Float(f64),
    Text(String),
}
impl Display for TokenLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenLiteral::Float(v) => write!(f, "{}", v),
            TokenLiteral::Text(v) => write!(f, "{}", v),
        }
    }
}

// impl ToString for TokenLiteral {
//     fn to_string(&self) -> String {
//         match self {
//             TokenLiteral::Float(num) => num.to_string(),
//             TokenLiteral::Text(s) => s.to_string(),
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct Token {
    pub lexeme: String,
    pub literal: Option<TokenLiteral>,
    pub line: usize,
    pub token_type: TokenType,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<TokenLiteral>,
        line: usize,
    ) -> Self {
        Token {
            lexeme,
            token_type,
            literal,
            line,
        }
    }
    pub fn to_string(self) -> String {
        let l = match self.literal {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };
        return self.token_type.to_string() + " " + &self.lexeme + " " + &l;
    }
}
