use lalrpop_util::lalrpop_mod;

use crate::core::Term;
use crate::meta::{Definition, Program};

use lexer::Lexer;

mod lexer;

impl Term {
    pub fn parse<S: AsRef<str>>(source: S) -> Option<Self> {
        let mut lexer = Lexer::new(source.as_ref());
        let parser = grammar::TermParser::new();
        parser.parse(&mut lexer).ok()
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
