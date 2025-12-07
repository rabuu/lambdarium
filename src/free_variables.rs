use std::collections::HashSet;

use crate::{Abstraction, Application, Term, Variable};

impl<'src> Term<'src> {
    pub fn free_variables(&self) -> HashSet<Variable<'src>> {
        let mut env = HashSet::new();
        self.free_variables_with_env(&mut env)
    }

    fn free_variables_with_env(
        &self,
        env: &mut HashSet<Variable<'src>>,
    ) -> HashSet<Variable<'src>> {
        match self {
            Term::Var(x) => x.free_variables_with_env(env),
            Term::Abs(x) => x.free_variables_with_env(env),
            Term::App(x) => x.free_variables_with_env(env),
        }
    }

    pub fn contains_free_variable(&self, var: &Variable<'src>) -> bool {
        match self {
            Term::Var(x) => var == x,
            Term::Abs(x) => x.contains_free_variable(var),
            Term::App(x) => x.contains_free_variable(var),
        }
    }
}

impl<'src> Variable<'src> {
    fn free_variables_with_env(
        &self,
        env: &mut HashSet<Variable<'src>>,
    ) -> HashSet<Variable<'src>> {
        if env.contains(self) {
            HashSet::new()
        } else {
            HashSet::from([*self])
        }
    }
}

impl<'src> Abstraction<'src> {
    fn free_variables_with_env(
        &self,
        env: &mut HashSet<Variable<'src>>,
    ) -> HashSet<Variable<'src>> {
        env.insert(self.binder);
        self.body.free_variables_with_env(env)
    }

    fn contains_free_variable(&self, var: &Variable<'src>) -> bool {
        *var != self.binder && self.body.contains_free_variable(var)
    }
}

impl<'src> Application<'src> {
    fn free_variables_with_env(
        &self,
        env: &mut HashSet<Variable<'src>>,
    ) -> HashSet<Variable<'src>> {
        let first = self.first.free_variables_with_env(env);
        let second = self.second.free_variables_with_env(env);
        [first, second].into_iter().flatten().collect()
    }

    fn contains_free_variable(&self, var: &Variable<'src>) -> bool {
        self.first.contains_free_variable(var) || self.second.contains_free_variable(var)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_free_variables() {
        let term = crate::parse_term(r#"\xy.x(\z.yz)"#).unwrap();
        assert!(term.free_variables().is_empty());
        assert!(!term.contains_free_variable(&Variable::from("x")));
        assert!(!term.contains_free_variable(&Variable::from("y")));
        assert!(!term.contains_free_variable(&Variable::from("z")));
    }

    #[test]
    fn atom_is_free() {
        let term = crate::parse_term(r#"x"#).unwrap();
        assert_eq!(term.free_variables(), HashSet::from([Variable::from("x")]));
        assert!(term.contains_free_variable(&Variable::from("x")));
    }

    #[test]
    fn some_free_vars() {
        let term = crate::parse_term(r#"\x.xy(\y.zy)"#).unwrap();
        assert_eq!(
            term.free_variables(),
            HashSet::from([Variable::from("y"), Variable::from("z")])
        );
        assert!(!term.contains_free_variable(&Variable::from("x")));
        assert!(term.contains_free_variable(&Variable::from("y")));
        assert!(term.contains_free_variable(&Variable::from("z")));
    }
}
