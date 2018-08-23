use std::fmt;

#[derive(PartialEq)]
pub enum TokenType {
    NAME,
    LBRACK,
    RBRACK,
    COMMA,
    EOF
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::NAME => write!(f, "NAME"),

            TokenType::LBRACK => write!(f, "LBRACK"),
            TokenType::RBRACK => write!(f, "RBRACK"),
            TokenType::COMMA => write!(f, "COMMA"),

            TokenType::EOF => write!(f, "EOF")
        }
    }
}
