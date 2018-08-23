use lexer::token::Token;
use lexer::token_type::TokenType;

#[derive(Debug)]
pub struct Scanner {
    pub input: Vec<char>,
    pos: usize,
    c: char
}

impl Scanner {
    pub fn new(input: String) -> Scanner {
        let chars: Vec<char> = input.chars().collect();
        Scanner {
            input: chars.clone(),
            pos: 0,
            c: chars[0]
        }
    }

    fn consume(&mut self) {
        self.pos += 1;

        if self.pos < self.input.len() {
            self.c = self.input[self.pos];
        }
    }

    fn consume_alphabetic(&mut self) -> Token {
        let mut id = String::new();

        while self.is_alphabetic() {
            id.push(self.c);
            self.consume();
        }   

        Token::new(TokenType::NAME, id)
    }

    fn is_whitespace(&self) -> bool {
        self.c.is_whitespace()
    }

    fn is_alphabetic(&self) -> bool {
        self.c.is_alphabetic()
    }

    fn skip_whitespace(&mut self) {
        while self.is_whitespace() { 
            self.consume(); 
        }
    }

    pub fn next_token(&mut self) -> Token {

        while self.pos < self.input.len() {
            if self.is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if self.c == ',' {
                self.consume();
                return Token::new(TokenType::COMMA, ",".to_owned());
            }

            if self.c == '[' {
                self.consume();
                return Token::new(TokenType::LBRACK, "[".to_owned());
            }

            if self.c == ']' {
               self.consume();
               return Token::new(TokenType::RBRACK, "]".to_owned()); 
            }

            if self.is_alphabetic() {
                return self.consume_alphabetic();
            }  

            panic!("Invalid character: {}", self.c);

        }
        Token::new(TokenType::EOF, "".to_owned())
    }
}