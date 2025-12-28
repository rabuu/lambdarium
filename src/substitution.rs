use crate::core::{Abstraction, Application, Term, Variable};

pub trait Subst: Sized {
    fn subst(self, x: &Variable, to: &Term) -> Term {
        let mut fresh = Fresh::new();
        self.subst_impl(x, to, &mut fresh)
    }

    fn subst_impl(self, x: &Variable, to: &Term, fresh: &mut Fresh) -> Term;
}

impl Subst for Term {
    fn subst_impl(self, x: &Variable, to: &Term, fresh: &mut Fresh) -> Term {
        match self {
            Term::Var(variable) => variable.subst_impl(x, to, fresh),
            Term::App(application) => application.subst_impl(x, to, fresh),
            Term::Abs(abstraction) => abstraction.subst_impl(x, to, fresh),
        }
    }
}

impl Subst for Variable {
    fn subst_impl(self, x: &Variable, to: &Term, _: &mut Fresh) -> Term {
        if self == *x { to.clone() } else { self.into() }
    }
}

impl Subst for Application {
    fn subst_impl(mut self, x: &Variable, to: &Term, fresh: &mut Fresh) -> Term {
        self.lhs = Box::new(self.lhs.subst_impl(x, to, fresh));
        self.rhs = Box::new(self.rhs.subst_impl(x, to, fresh));
        self.into()
    }
}

impl Subst for Abstraction {
    fn subst_impl(mut self, x: &Variable, to: &Term, fresh: &mut Fresh) -> Term {
        if self.binder == *x {
            return self.into();
        }

        if to.contains_free_variable(&self.binder) && self.body.contains_free_variable(x) {
            let fresh_var = fresh.fresh();
            return Abstraction {
                binder: fresh_var.clone(),
                body: Box::new(
                    self.body
                        .subst_impl(&self.binder, &fresh_var.into(), fresh)
                        .subst_impl(x, to, fresh),
                ),
            }
            .into();
        }

        self.body = Box::new(self.body.subst_impl(x, to, fresh));
        self.into()
    }
}

#[derive(Debug, Default)]
pub struct Fresh {
    state: usize,
}

impl Fresh {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fresh(&mut self) -> Variable {
        let var = format!("_{}", self.state).into();
        self.state += 1;
        var
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_happens() {
        let term = Term::parse("\\x.zy").unwrap();
        let replace = Term::parse("(uv)").unwrap();
        assert_eq!(term.clone().subst(&Variable::from("x"), &replace), term);
    }

    #[test]
    fn simple_subst() {
        let term = Term::parse("\\y.x").unwrap();
        let replace = Term::parse("z").unwrap();
        let substed = Term::parse("\\y.z").unwrap();
        assert_eq!(term.subst(&Variable::from("x"), &replace), substed);
    }

    #[test]
    fn nested_subst() {
        let term = Term::parse(r#"\y.x(\x.x)"#).unwrap();
        let replace = Term::parse(r#"\y.xy"#).unwrap();
        let substed = Term::parse(r#"\y.(\y.xy)(\x.x)"#).unwrap();
        assert_eq!(term.subst(&Variable::from("x"), &replace), substed);
    }

    #[test]
    fn subst_with_fresh() {
        let term = Term::parse(r#"\y.xy"#).unwrap();
        let y: Term = Variable::from("y").into();
        let fresh = Variable::from("_0");

        let substed = Abstraction {
            binder: fresh.clone(),
            body: Box::new(
                Application {
                    lhs: Box::new(y.clone()),
                    rhs: Box::new(fresh.into()),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(term.subst(&Variable::from("x"), &y), substed);
    }
}
