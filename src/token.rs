use std::{fmt::{self,Display}, usize};

#[derive(Debug)]
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
impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Token {
    pub lexeme: String,
    pub token_type: TokenType,
    pub literal :Option<String>,
    pub line: usize
}

impl Token {
    fn new(token_type: TokenType, lexeme:String, literal:Option<String>, line:usize) ->Self{
        Token { lexeme, token_type, literal, line }
    }
    fn to_string(self) -> String{
        return self.token_type.to_string() + " " + &self.lexeme + " " + &self.literal.unwrap_or("".to_string())
    }
}


