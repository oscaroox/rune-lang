mod lexer;
mod parser;

use lexer::scanner::Scanner;
use parser::list::ListParser;
use parser::parser::Parser;

fn main() {

    let input = String::from("[a, b, [a, b, c, d]]");

    let scanner = Scanner::new(input);

    let mut parser = Parser::new(scanner);

    let mut list_parser = ListParser::new(&mut parser);

    list_parser.parse();
}

