mod lexer;

use lexer::token::Token;
use lexer::token_type::TokenType;
use lexer::scanner::Scanner;

fn main() {


    let lbrack = Token::new(TokenType::LBRACK, String::from("["));
    let rbrack = Token::new(TokenType::RBRACK, String::from("]"));

    let scanner = Scanner::new(String::from("[a, b, c]"));
  

    println!("{}", lbrack);
    println!("{}", rbrack);
}

