use lalrpop_util::lalrpop_mod;

use crate::core;
use crate::meta::{Definition, Program, Term};

use lexer::Lexer;

mod lexer;

impl Term {
    pub fn parse<S: AsRef<str>>(source: S) -> Option<Self> {
        let mut lexer = Lexer::new(source.as_ref());
        let parser = grammar::TermParser::new();
        parser.parse(&mut lexer).ok()
    }
}

impl core::Term {
    pub fn parse<S: AsRef<str>>(source: S) -> Option<Self> {
        let meta_term = Term::parse(source)?;
        let term = meta_term.lower(&[])?;
        Some(term)
    }
}

impl Program {
    pub fn parse<S: AsRef<str>>(source: S) -> Option<Self> {
        let mut lexer = Lexer::new(source.as_ref());
        let parser = grammar::ProgramParser::new();
        parser.parse(&mut lexer).ok()
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
