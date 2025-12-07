use crate::{Abstraction, Application, Term, substitution::Subst};

impl Term {
    pub fn beta_contract(self) -> Option<Term> {
        match self {
            Term::Var(_) => None,
            Term::App(application) => application.beta_contract(),
            Term::Abs(abstraction) => abstraction.beta_contract(),
        }
    }
}

impl Application {
    fn beta_contract(mut self) -> Option<Term> {
        if let Term::Abs(abs) = *self.first {
            return Some(abs.body.subst(&abs.binder, &self.second));
        }

        if let Some(constracted) = self.first.clone().beta_contract() {
            self.first = Box::new(constracted);
            return Some(self.into());
        }

        if let Some(constracted) = self.second.beta_contract() {
            self.second = Box::new(constracted);
            return Some(self.into());
        }

        None
    }
}

impl Abstraction {
    fn beta_contract(mut self) -> Option<Term> {
        if let Some(contracted) = self.body.beta_contract() {
            self.body = Box::new(contracted);
            return Some(self.into());
        }

        None
    }
}
