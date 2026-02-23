use std::path::PathBuf;

use clap::{Parser, Subcommand};
use miette::IntoDiagnostic;

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
    /// Execute the interactive TUI
    Tui,

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
        Command::Tui => lambdarium_tui::run_app().into_diagnostic()?,
        Command::Repl => repl::run()?,
        Command::Eval { input_file } => eval::run(&input_file)?,
    }

    Ok(())
}
