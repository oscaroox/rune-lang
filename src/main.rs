mod lexer;
mod parser;

use lexer::scanner::Scanner;
use parser::parser::Parser;
use parser::list::ListParser;

fn main() {


    let scanner = Scanner::new(String::from("[a, b, [a, b, c, d]]"));

    let mut parser = Parser::new(scanner);

    let mut list_parser = ListParser::new(&mut parser);

    list_parser.parse();
}

