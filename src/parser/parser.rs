use lexer::scanner::Scanner;
use lexer::token::Token;
use lexer::token_type::TokenType;

pub struct Parser {
    lexer: Scanner,
    pub next_char: Token
}

impl Parser {

    pub fn new(mut lexer: Scanner) -> Parser {
        let token = lexer.next_token();
        Parser {
            lexer,
            next_char: token
        }
    }

    pub fn matches(&mut self, id: TokenType) {
 
        if self.next_char.id == id {
            self.consume();
        }  else {
            panic!("Expected {}; found {}", id, self.next_char.id);
        }
    }

    pub fn consume(&mut self) {
        self.next_char = self.lexer.next_token();
    }
}