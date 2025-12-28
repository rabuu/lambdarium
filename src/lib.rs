use std::collections::VecDeque;

mod alpha;
mod beta;
mod display;
mod free_variables;
mod length;
mod parser;
mod stlc;
mod substitution;

pub const LAMBDA: char = 'λ';
pub const BETA: char = 'β';

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable),
    App(Application),
    Abs(Abstraction),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variable {
    pub inner: String,
}

impl<S> From<S> for Variable
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Self {
            inner: value.into(),
        }
    }
}

impl From<Variable> for Term {
    fn from(value: Variable) -> Self {
        Self::Var(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application {
    pub lhs: Box<Term>,
    pub rhs: Box<Term>,
}

impl From<Application> for Term {
    fn from(value: Application) -> Self {
        Self::App(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abstraction {
    pub binder: Variable,
    pub body: Box<Term>,
}

impl Abstraction {
    /// \xyz.e -> \x.\y.\z.e
    ///
    /// # Preconditions
    /// Panics when `binders` is empty.
    pub fn from_uncurried(mut binders: VecDeque<Variable>, body: Term) -> Abstraction {
        assert!(!binders.is_empty());

        if binders.len() == 1 {
            Self {
                binder: binders[0].clone(),
                body: Box::new(body),
            }
        } else {
            Self {
                binder: binders.pop_front().unwrap(),
                body: Box::new(Term::Abs(Abstraction::from_uncurried(binders, body))),
            }
        }
    }
}

impl From<Abstraction> for Term {
    fn from(value: Abstraction) -> Self {
        Self::Abs(value)
    }
}
