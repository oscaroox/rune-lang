use lexer::token_type::TokenType;
use parser::parser::Parser;

pub struct ListParser<'a> {
    parser: &'a mut Parser
}

impl <'a>ListParser<'a> {

    pub fn new(parser: &'a mut Parser) -> ListParser {
        ListParser {
            parser: parser
        }
    }

    pub fn parse(&mut self) {
        self.list();
    }

    fn list(&mut self) {
        self.parser.matches(TokenType::LBRACK);
        self.elements();
        self.parser.matches(TokenType::RBRACK);
    }

    fn elements(&mut self) {
        self.element();

        while self.parser.next_char.id == TokenType::COMMA {
            self.parser.matches(TokenType::COMMA);
            self.element();
        }
    }

    fn element(&mut self) {

        if self.parser.next_char.id == TokenType::NAME {
            self.parser.matches(TokenType::NAME);
        } else if self.parser.next_char.id == TokenType::LBRACK {
            self.list();
        } else {
            panic!("expecting name or list; found {}", self.parser.next_char);
        }
    }
}