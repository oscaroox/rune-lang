mod lexer;

use lexer::scanner::Scanner;

fn main() {


    let mut scanner = Scanner::new(String::from("[a, b, c]"));
    
    let mut token = scanner.next_token();

    while !token.id.is_eof() {
        println!("{}", token);
        token = scanner.next_token();
    }
}

