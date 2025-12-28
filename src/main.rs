use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let Some(term) = lambda_calculator::parse_term(&line) else {
            eprintln!("Oops, try again!");
            continue;
        };

        if let Some(typ) = term.get_type() {
            println!("{}", typ);
        }

        // println!("TERM: {term}");
        // let mut term = term.clone();
        // while let Some(reduced) = term.beta_contract() {
        //     term = reduced;
        //     eprintln!("{term}");
        // }
    }
}
