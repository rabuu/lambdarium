use crate::core::{Abstraction, Application, Term};

const NORMALIZATION_LIMIT: usize = 1000;

impl Term {
    pub fn beta_contract(self) -> Option<Term> {
        match self {
            Term::Var(_) => None,
            Term::App(application) => application.beta_contract(),
            Term::Abs(abstraction) => abstraction.beta_contract(),
        }
    }

    pub fn normalize(mut self) -> Option<Term> {
        let mut counter = 0;

        while let Some(contractum) = self.clone().beta_contract() {
            self = contractum;
            counter += 1;

            if counter > NORMALIZATION_LIMIT {
                return None;
            }
        }

        Some(self)
    }
}

impl Application {
    fn beta_contract(mut self) -> Option<Term> {
        if let Term::Abs(abs) = *self.lhs {
            return Some(abs.body.subst(&abs.binder, &self.rhs));
        }

        if let Some(constracted) = self.lhs.clone().beta_contract() {
            self.lhs = Box::new(constracted);
            return Some(self.into());
        }

        if let Some(constracted) = self.rhs.beta_contract() {
            self.rhs = Box::new(constracted);
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
