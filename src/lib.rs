use std::collections::VecDeque;

pub use parser::parse_term;

mod display;
mod free_variables;
mod length;
mod parser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term<'src> {
    Var(Variable<'src>),
    App(Application<'src>),
    Abs(Abstraction<'src>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Variable<'src> {
    pub inner: &'src str,
}

impl<'src> From<&'src str> for Variable<'src> {
    fn from(value: &'src str) -> Self {
        Self { inner: value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application<'src> {
    pub first: Box<Term<'src>>,
    pub second: Box<Term<'src>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abstraction<'src> {
    pub binder: Variable<'src>,
    pub body: Box<Term<'src>>,
}

impl<'src> Abstraction<'src> {
    pub fn from_uncurried(
        mut binders: VecDeque<Variable<'src>>,
        body: Term<'src>,
    ) -> Abstraction<'src> {
        assert!(!binders.is_empty());

        if binders.len() == 1 {
            return Self {
                binder: binders[0],
                body: Box::new(body),
            };
        }

        Self {
            binder: binders.pop_front().unwrap(),
            body: Box::new(Term::Abs(Abstraction::from_uncurried(binders, body))),
        }
    }
}
