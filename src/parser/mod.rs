use lalrpop_util::lalrpop_mod;

use crate::ast;

mod token;

pub fn parse(source: &str) -> Option<ast::Term> {
    todo!()
}

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    grammar,
    "/parser/grammar.rs"
);
