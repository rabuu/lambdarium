use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod error;
mod eval;
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

fn main() -> miette::Result<()> {
    let args = CliArgs::parse();

    match args.cmd {
        Command::Repl => repl::run()?,
        Command::Eval { input_file } => eval::run(&input_file)?,
    }

    Ok(())
}
