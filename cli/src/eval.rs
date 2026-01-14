use std::fs;
use std::path::Path;

use lambda_calculator::meta::Program;
use lambda_calculator::print::Print;

use crate::error::AppError;

pub fn run(input_file: &Path) -> Result<(), AppError> {
    let file = fs::read_to_string(input_file)?;
    let program = Program::parse(&file)?;
    let terms = program.lower()?;

    for term in terms {
        let nf = term
            .clone()
            .normalize()
            .ok_or(AppError::Normalization { term })?;

        println!("{}", nf.print_pretty());
    }

    Ok(())
}
