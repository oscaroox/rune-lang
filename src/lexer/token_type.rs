use std::fmt;

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
            NAME => write!(f, "NAME"),

            LBRACK => write!(f, "LBRACK"),
            RBRACK => write!(f, "RBRACK"),
            COMMA => write!(f, "COMMA"),

            EOF => write!(f, "EOF")
        }
    }
}