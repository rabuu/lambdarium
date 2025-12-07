use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let Some(mut term) = lambda_calculator::parse_term(&line) else {
            eprintln!("Oops, try again!");
            continue;
        };

        println!("TERM: {term}");
        while let Some(reduced) = term.beta_contract() {
            term = reduced;
            eprintln!("{term}");
        }
    }
}
