use crate::core::{Abstraction, Application, Term, Variable};

impl Term {
    pub fn length(&self) -> usize {
        match self {
            Term::Var(x) => x.length(),
            Term::Abs(x) => x.length(),
            Term::App(x) => x.length(),
        }
    }
}

impl Variable {
    pub fn length(&self) -> usize {
        1
    }
}

impl Abstraction {
    pub fn length(&self) -> usize {
        self.binder.length() + self.body.length()
    }
}

impl Application {
    pub fn length(&self) -> usize {
        self.lhs.length() + self.rhs.length()
    }
}
