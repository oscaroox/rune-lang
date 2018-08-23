use std::fmt;
use lexer::token_type::TokenType;


pub struct Token {
    id: TokenType,
    text: String
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token(\"{}\", \"{}\")", self.id, self.text)
    }
}


impl Token {
    pub fn new(id: TokenType, text: String) -> Token {
        Token {
            id: id,
            text: text
        }
    }
}