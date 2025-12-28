use std::collections::HashSet;

use crate::core::{Abstraction, Application, Term, Variable};

impl Term {
    pub fn free_variables(&self) -> HashSet<Variable> {
        let mut env = HashSet::new();
        self.free_variables_with_env(&mut env)
    }

    fn free_variables_with_env(&self, env: &mut HashSet<Variable>) -> HashSet<Variable> {
        match self {
            Term::Var(x) => x.free_variables_with_env(env),
            Term::Abs(x) => x.free_variables_with_env(env),
            Term::App(x) => x.free_variables_with_env(env),
        }
    }

    pub fn contains_free_variable(&self, var: &Variable) -> bool {
        match self {
            Term::Var(x) => var == x,
            Term::Abs(x) => x.contains_free_variable(var),
            Term::App(x) => x.contains_free_variable(var),
        }
    }
}

impl Variable {
    fn free_variables_with_env(&self, env: &mut HashSet<Variable>) -> HashSet<Variable> {
        if env.contains(self) {
            HashSet::new()
        } else {
            HashSet::from([self.clone()])
        }
    }
}

impl Abstraction {
    fn free_variables_with_env(&self, env: &mut HashSet<Variable>) -> HashSet<Variable> {
        env.insert(self.binder.clone());
        self.body.free_variables_with_env(env)
    }

    fn contains_free_variable(&self, var: &Variable) -> bool {
        *var != self.binder && self.body.contains_free_variable(var)
    }
}

impl Application {
    fn free_variables_with_env(&self, env: &mut HashSet<Variable>) -> HashSet<Variable> {
        let lhs = self.lhs.free_variables_with_env(env);
        let rhs = self.rhs.free_variables_with_env(env);
        [lhs, rhs].into_iter().flatten().collect()
    }

    fn contains_free_variable(&self, var: &Variable) -> bool {
        self.lhs.contains_free_variable(var) || self.rhs.contains_free_variable(var)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_free_variables() {
        let term = Term::parse(r#"\xy.x(\z.yz)"#).unwrap();
        assert!(term.free_variables().is_empty());
        assert!(!term.contains_free_variable(&Variable::from("x")));
        assert!(!term.contains_free_variable(&Variable::from("y")));
        assert!(!term.contains_free_variable(&Variable::from("z")));
    }

    #[test]
    fn atom_is_free() {
        let term = Term::parse(r#"x"#).unwrap();
        assert_eq!(term.free_variables(), HashSet::from([Variable::from("x")]));
        assert!(term.contains_free_variable(&Variable::from("x")));
    }

    #[test]
    fn some_free_vars() {
        let term = Term::parse(r#"\x.xy(\y.zy)"#).unwrap();
        assert_eq!(
            term.free_variables(),
            HashSet::from([Variable::from("y"), Variable::from("z")])
        );
        assert!(!term.contains_free_variable(&Variable::from("x")));
        assert!(term.contains_free_variable(&Variable::from("y")));
        assert!(term.contains_free_variable(&Variable::from("z")));
    }
}
