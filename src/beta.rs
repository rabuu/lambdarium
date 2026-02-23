use crate::core::{Abstraction, Application, Term};

const NORMALIZATION_LIMIT: usize = 1000;

#[derive(Debug, Clone, Copy)]
pub enum EvaluationStrategy {
    NormalOrder,
    ApplicativeOrder,
    CallByValue,
    CallByName,
    LeftIndexed(usize),
}

impl EvaluationStrategy {
    fn weak(&self) -> bool {
        matches!(
            self,
            EvaluationStrategy::CallByValue | EvaluationStrategy::CallByName
        )
    }
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

    pub fn count_redexes(&self) -> usize {
        match self {
            Term::Var(_) => 0,
            Term::App(application) => application.count_redexes(),
            Term::Abs(abstraction) => abstraction.count_redexes(),
        }
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

                None
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

                None
            }
            EvaluationStrategy::LeftIndexed(mut n) => {
                if let Term::Abs(ref abs) = *self.lhs {
                    if n == 0 {
                        return Some(abs.clone().body.subst(&abs.binder, &self.rhs));
                    }
                    n -= 1;
                }

                let lhs_redexes = self.lhs.count_redexes();
                if n < lhs_redexes {
                    let contracted = self
                        .lhs
                        .clone()
                        .beta_contract(EvaluationStrategy::LeftIndexed(n))
                        .expect("contains at least n redexes");
                    self.lhs = Box::new(contracted);
                    return Some(self.into());
                }

                n -= lhs_redexes;

                if let Some(contracted) = self.rhs.beta_contract(EvaluationStrategy::LeftIndexed(n))
                {
                    self.rhs = Box::new(contracted);
                    return Some(self.into());
                }

                None
            }
        }
    }

    fn count_redexes(&self) -> usize {
        let lhs = self.lhs.count_redexes();
        let rhs = self.rhs.count_redexes();

        if let Term::Abs(_) = *self.lhs {
            lhs + rhs + 1
        } else {
            lhs + rhs
        }
    }
}

impl Abstraction {
    fn beta_contract(mut self, strategy: EvaluationStrategy) -> Option<Term> {
        if strategy.weak() {
            return None;
        }

        let contracted = self.body.beta_contract(strategy)?;
        self.body = Box::new(contracted);

        Some(self.into())
    }

    fn count_redexes(&self) -> usize {
        self.body.count_redexes()
    }
}
