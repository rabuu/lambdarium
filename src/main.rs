use std::io::{self, BufRead};

mod ast;
mod parser;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let Some(term) = parser::parse(&line) else {
            eprintln!("Oops, try again!");
            continue;
        };

        println!("{term:?}");
    }
}
