use lalrpop_util::lalrpop_mod;

use crate::Term;

use lexer::Lexer;

mod lexer;

pub fn parse_term<'src>(source: &'src str) -> Option<Term<'src>> {
    let mut lexer = Lexer::new(source);
    let parser = grammar::TermParser::new();
    parser.parse(&mut lexer).ok()
}

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    grammar,
    "/parser/grammar.rs"
);
