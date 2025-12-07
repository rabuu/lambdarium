use std::fmt;

use crate::{Abstraction, Application, Term, Variable};

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(x) => x.fmt(f),
            Term::App(x) => x.fmt(f),
            Term::Abs(x) => x.fmt(f),
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.inner.len() == 1 {
            write!(f, "{}", self.inner)
        } else {
            write!(f, r#""{}""#, self.inner)
        }
    }
}

impl fmt::Display for Application {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.first, self.second)
    }
}

impl fmt::Display for Abstraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(λ{}.{})", self.binder, self.body)
    }
}
