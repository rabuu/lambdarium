use std::io;

use miette::Diagnostic;
use thiserror::Error;

use lambda_calculator::{LoweringError, ParserError, Print, core};

#[derive(Error, Diagnostic, Debug)]
pub enum AppError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Parser(#[from] ParserError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Lowering(#[from] LoweringError),

    #[error("Term is not normalizable: {}", term.print_pretty())]
    Normalization { term: core::Term },

    #[error(transparent)]
    IO(#[from] io::Error),
}
