use crate::{Abstraction, Application, Term, Variable};

impl Term<'_> {
    pub fn length(&self) -> usize {
        match self {
            Term::Var(x) => x.length(),
            Term::Abs(x) => x.length(),
            Term::App(x) => x.length(),
        }
    }
}

impl Variable<'_> {
    pub fn length(&self) -> usize {
        1
    }
}

impl Abstraction<'_> {
    pub fn length(&self) -> usize {
        self.binder.length() + self.body.length()
    }
}

impl Application<'_> {
    pub fn length(&self) -> usize {
        self.first.length() + self.second.length()
    }
}
