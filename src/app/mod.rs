use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use lambda_calculator::meta::Program;

mod repl;

#[derive(Debug, Parser)]
#[command(version)]
pub struct CliArgs {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Execute the interactive REPL
    Repl,

    /// Evaluate (β-normalize) a file with λ-terms
    Eval {
        /// The input source file
        input_file: PathBuf,
    },
}

pub fn run(cli: CliArgs) -> Result<(), ()> {
    match cli.cmd {
        Command::Repl => repl::run().map_err(|_| ())?,
        Command::Eval { input_file } => {
            let file = fs::read_to_string(&input_file).map_err(|_| ())?;
            let program = Program::parse(&file).ok_or(())?;
            let terms = program.lower().ok_or(())?;

            for term in terms {
                let nf = term.normalize().ok_or(())?;
                println!("{nf}");
            }
        }
    }

    Ok(())
}
