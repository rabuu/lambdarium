use lalrpop_util::lalrpop_mod;

use crate::Term;

use lexer::Lexer;

mod lexer;

impl Term {
    pub fn parse<S: AsRef<str>>(source: S) -> Option<Self> {
        let mut lexer = Lexer::new(source.as_ref());
        let parser = grammar::TermParser::new();
        parser.parse(&mut lexer).ok()
    }
}

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    grammar,
    "/parser/grammar.rs"
);
