use std;

#[derive(Debug)]
pub struct Scanner {
    pub input: Vec<char>,
    c: Option<char>
}

impl Scanner {
    pub fn new(input: String) -> Scanner {
        Scanner {
            input: input.chars().collect(),
            c: None
        }
    }
}