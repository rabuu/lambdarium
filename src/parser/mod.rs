use lalrpop_util::lalrpop_mod;

use crate::{core, lowering, meta};

pub use error::ParserError;
use lexer::Lexer;

mod error;
mod lexer;

impl meta::Term {
    pub fn parse<S: AsRef<str>>(source: S) -> Result<Self, ParserError> {
        let mut lexer = Lexer::new(source.as_ref());
        let parser = grammar::TermParser::new();
        parser.parse(&mut lexer).map_err(Into::into)
    }
}

impl meta::Definition {
    pub fn parse<S: AsRef<str>>(source: S) -> Result<Self, ParserError> {
        let mut lexer = Lexer::new(source.as_ref());
        let parser = grammar::DefinitionParser::new();
        parser.parse(&mut lexer).map_err(Into::into)
    }
}

impl meta::Item {
    pub fn parse<S: AsRef<str>>(source: S) -> Result<Self, ParserError> {
        let mut lexer = Lexer::new(source.as_ref());
        let parser = grammar::ItemParser::new();
        parser.parse(&mut lexer).map_err(Into::into)
    }
}

impl meta::Program {
    pub fn parse<S: AsRef<str>>(source: S) -> Result<Self, ParserError> {
        let mut lexer = Lexer::new(source.as_ref());
        let parser = grammar::ProgramParser::new();
        parser.parse(&mut lexer).map_err(Into::into)
    }
}

impl core::Term {
    pub fn parse<S: AsRef<str>>(source: S) -> Result<Self, ParserError> {
        let meta_term = meta::Term::parse(source)?;
        meta_term
            .lower(&lowering::Context::default())
            .map_err(Into::into)
    }
}

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    grammar,
    "/parser/grammar.rs"
);
