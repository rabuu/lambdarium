use std::collections::VecDeque;

use chumsky::input::{Stream, ValueInput};
use chumsky::prelude::*;

use crate::ast;
use lexer::Token;

mod lexer;

pub fn parse(source: &str) -> Option<ast::Term> {
    let tokens = lexer::lex(source)?;
    parse_tokens(tokens)
}

fn parse_tokens(tokens: Vec<Token>) -> Option<ast::Term> {
    let token_stream = Stream::from_iter(tokens);
    term().parse(token_stream).into_output()
}

fn term<'tok, I>() -> impl Parser<'tok, I, ast::Term>
where
    I: ValueInput<'tok, Token = Token>,
{
    let open = just(Token::ParenOpen);
    let close = just(Token::ParenClose);

    recursive(|term| {
        let var = select! {
            Token::Var(c) => ast::Variable(c),
        };

        let app = term
            .clone()
            .then(term.clone())
            .delimited_by(open.clone(), close.clone())
            .map(|(first, second)| ast::Application {
                first: Box::new(first),
                second: Box::new(second),
            });

        let abs = just(Token::Lambda)
            .ignore_then(var.repeated().at_least(1).collect::<VecDeque<_>>())
            .then_ignore(just(Token::Dot))
            .then(term.clone())
            .delimited_by(open.clone(), close.clone())
            .map(|(mut binders, body)| {
                let binder = binders.pop_front().expect("at least 1");
                ast::Abstraction {
                    binder,
                    body: Box::new(desugar_abstraction_body(binders, body)),
                }
            });

        choice((
            var.map(ast::Term::Var),
            app.map(ast::Term::App),
            abs.map(ast::Term::Abs),
        ))
    })
}

fn desugar_abstraction_body(mut binders: VecDeque<ast::Variable>, body: ast::Term) -> ast::Term {
    let Some(binder) = binders.pop_front() else {
        return body;
    };

    ast::Term::Abs(ast::Abstraction {
        binder,
        body: Box::new(desugar_abstraction_body(binders, body)),
    })
}
