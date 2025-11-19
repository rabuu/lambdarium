use lalrpop_util::lalrpop_mod;

use crate::ast;

use lexer::Lexer;

mod lexer;

pub fn parse<'src>(source: &'src str) -> Option<ast::Term<'src>> {
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
