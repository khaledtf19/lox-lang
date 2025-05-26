use std::{fmt::{self,Display}, usize};

use phf::phf_map;




#[derive(Debug, Clone, Copy)]
pub enum TokenType {
  // Single-character tokens.
  LEFTPAREN, RIGHTPAREN, LEFTBRACE, RIGHTBRACE,
  COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

  // One or two character tokens.
  BANG, BANGEQUAL,
  EQUAL, EQUALEQUAL,
  GREATER, GREATEREQUAL,
  LESS, LESSEQUAL,

  // Literals.
  IDENTIFIER, STRING, NUMBER,

  // Keywords.
  AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
  PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

  EOF
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
};
pub fn parse_keyword(keyword: &str) -> Option<TokenType> {
    KEYWORDS.get(keyword).cloned()
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum TokenLiteral {
    Float(f64),
    Text(String)
}

impl ToString for TokenLiteral {
    fn to_string(&self) -> String {
        match self {
            TokenLiteral::Float(num) => num.to_string(),
            TokenLiteral::Text(s) => s.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub lexeme: String,
    pub token_type: TokenType,
    pub literal :Option<TokenLiteral>,
    pub line: usize
}

impl Token {
    fn new(token_type: TokenType, lexeme:String, literal:Option<TokenLiteral>, line:usize) ->Self{
        Token { lexeme, token_type, literal, line }
    }
    fn to_string(self) -> String{
        let l = match self.literal {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };
        return self.token_type.to_string() + " " + &self.lexeme + " " + &l;
    }
}


