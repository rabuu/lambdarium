use crate::core::{Abstraction, Application, Term};

const NORMALIZATION_LIMIT: usize = 1000;

#[derive(Debug, Clone, Copy)]
pub enum EvaluationStrategy {
    NormalOrder,
    ApplicativeOrder,
    CallByValue,
    CallByName,
}

impl Term {
    pub fn beta_contract(self, strategy: EvaluationStrategy) -> Option<Term> {
        match self {
            Term::Var(_) => None,
            Term::App(application) => application.beta_contract(strategy),
            Term::Abs(abstraction) => abstraction.beta_contract(strategy),
        }
    }

    pub fn normalize(mut self) -> Option<Term> {
        let mut counter = 0;

        while let Some(contractum) = self.clone().beta_contract(EvaluationStrategy::NormalOrder) {
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
    fn beta_contract(mut self, strategy: EvaluationStrategy) -> Option<Term> {
        match strategy {
            EvaluationStrategy::NormalOrder | EvaluationStrategy::CallByValue => {
                if let Term::Abs(abs) = *self.lhs {
                    return Some(abs.body.subst(&abs.binder, &self.rhs));
                }

                if let Some(contracted) = self.lhs.clone().beta_contract(strategy) {
                    self.lhs = Box::new(contracted);
                    return Some(self.into());
                }

                if let Some(contracted) = self.rhs.beta_contract(strategy) {
                    self.rhs = Box::new(contracted);
                    return Some(self.into());
                }
            }
            EvaluationStrategy::ApplicativeOrder | EvaluationStrategy::CallByName => {
                if let Some(contracted) = self.lhs.clone().beta_contract(strategy) {
                    self.lhs = Box::new(contracted);
                    return Some(self.into());
                }

                if let Some(contracted) = self.rhs.clone().beta_contract(strategy) {
                    self.rhs = Box::new(contracted);
                    return Some(self.into());
                }

                if let Term::Abs(abs) = *self.lhs {
                    return Some(abs.body.subst(&abs.binder, &self.rhs));
                }
            }
        }

        None
    }
}

impl Abstraction {
    fn beta_contract(mut self, strategy: EvaluationStrategy) -> Option<Term> {
        match strategy {
            EvaluationStrategy::CallByValue | EvaluationStrategy::CallByName => None,
            EvaluationStrategy::NormalOrder | EvaluationStrategy::ApplicativeOrder => {
                let contracted = self.body.beta_contract(strategy)?;
                self.body = Box::new(contracted);
                Some(self.into())
            }
        }
    }
}
