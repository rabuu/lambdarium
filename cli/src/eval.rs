use std::fs;
use std::path::Path;

use lambda_calculator::meta::Program;

pub fn run(input_file: &Path) -> Result<(), ()> {
    let file = fs::read_to_string(input_file).map_err(|_| ())?;
    let program = Program::parse(&file).ok_or(())?;
    let terms = program.lower().ok_or(())?;

    for term in terms {
        let nf = term.normalize().ok_or(())?;
        println!("{nf}");
    }

    Ok(())
}
