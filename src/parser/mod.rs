use lalrpop_util::lalrpop_mod;

use crate::core;
use crate::meta::{Definition, Program, Term};

pub use error::ParserError;
use lexer::Lexer;

mod error;
mod lexer;

impl Term {
    pub fn parse<S: AsRef<str>>(source: S) -> Result<Self, ParserError> {
        let mut lexer = Lexer::new(source.as_ref());
        let parser = grammar::TermParser::new();
        parser.parse(&mut lexer).map_err(Into::into)
    }
}

impl core::Term {
    pub fn parse<S: AsRef<str>>(source: S) -> Result<Self, ParserError> {
        let meta_term = Term::parse(source)?;
        meta_term.lower(&[]).map_err(Into::into)
    }
}

impl Program {
    pub fn parse<S: AsRef<str>>(source: S) -> Result<Self, ParserError> {
        let mut lexer = Lexer::new(source.as_ref());
        let parser = grammar::ProgramParser::new();
        parser.parse(&mut lexer).map_err(Into::into)
    }
}

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    grammar,
    "/parser/grammar.rs"
);

enum ProgramElement {
    Def(Definition),
    Term(Term),
}
