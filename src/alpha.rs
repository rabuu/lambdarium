use crate::substitution::Subst;
use crate::{Abstraction, Application, Term, Variable};

impl Abstraction {
    pub fn alpha_rename(mut self, new: &Variable) -> Option<Abstraction> {
        if self.body.contains_free_variable(new) {
            return None;
        }

        self.body = Box::new(self.body.subst(&self.binder, &new.clone().into()));
        self.binder = new.clone();

        Some(self)
    }
}

impl Term {
    pub fn is_alpha_congruent(&self, other: &Term) -> bool {
        match self {
            Term::Var(var) => var.is_alpha_congruent(other),
            Term::App(app) => app.is_alpha_congruent(other),
            Term::Abs(abs) => abs.is_alpha_congruent(other),
        }
    }
}

impl Variable {
    fn is_alpha_congruent(&self, other: &Term) -> bool {
        match other {
            Term::Var(other) => self == other,
            _ => false,
        }
    }
}

impl Application {
    fn is_alpha_congruent(&self, other: &Term) -> bool {
        match other {
            Term::App(other) => {
                self.lhs.is_alpha_congruent(&other.lhs) && self.rhs.is_alpha_congruent(&other.rhs)
            }
            _ => false,
        }
    }
}

impl Abstraction {
    fn is_alpha_congruent(&self, other: &Term) -> bool {
        match other {
            Term::Abs(other) => {
                if self == other {
                    return true;
                }

                let Some(renamed) = self.clone().alpha_rename(&other.binder) else {
                    // If the renamed binder is a free variable in the body,
                    // the terms cannot be alpha congruent.
                    return false;
                };

                renamed.body.is_alpha_congruent(&other.body.clone())
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_alpha_rename() {
        let term = Abstraction {
            binder: Variable::from("x"),
            body: Box::new(
                Application {
                    lhs: Box::new(Variable::from("x").into()),
                    rhs: Box::new(Variable::from("y").into()),
                }
                .into(),
            ),
        };

        let renamed = Abstraction {
            binder: Variable::from("z"),
            body: Box::new(
                Application {
                    lhs: Box::new(Variable::from("z").into()),
                    rhs: Box::new(Variable::from("y").into()),
                }
                .into(),
            ),
        };

        assert_eq!(term.alpha_rename(&Variable::from("z")), Some(renamed));
    }

    #[test]
    fn identity_alpha_congruent() {
        let id1 = Term::parse(r#"\x.x"#).unwrap();
        let id2 = Term::parse(r#"\y.y"#).unwrap();
        assert!(id1.is_alpha_congruent(&id2));
    }

    #[test]
    fn constant_function_alpha_congruent() {
        let id1 = Term::parse(r#"\xy.x"#).unwrap();
        let id2 = Term::parse(r#"\ab.a"#).unwrap();
        assert!(id1.is_alpha_congruent(&id2));
    }
}
