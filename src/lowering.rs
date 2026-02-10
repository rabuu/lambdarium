use std::collections::{HashMap, VecDeque};

use miette::Diagnostic;
use thiserror::Error;

use crate::{core, meta};

type MetaBindings = HashMap<meta::MetaVariable, core::Term>;

#[derive(Error, Diagnostic, Debug)]
pub enum LoweringError {
    #[error("The term contains unresolvable meta-variables.")]
    UnresolvableMetaVariables,
}

impl meta::Program {
    pub fn lower(self) -> Result<Vec<core::Term>, LoweringError> {
        let context = self.context.lower()?;
        self.terms
            .into_iter()
            .map(|t| t.lower_inner(&context))
            .collect()
    }
}

impl meta::Term {
    pub fn lower(self, ctx: &meta::Context) -> Result<core::Term, LoweringError> {
        self.lower_inner(&ctx.lower()?)
    }

    fn lower_inner(self, meta_bindings: &MetaBindings) -> Result<core::Term, LoweringError> {
        match self {
            meta::Term::MetaVar(meta_variable) => meta_variable.lower_inner(meta_bindings),
            meta::Term::Var(variable) => variable.lower_inner(meta_bindings),
            meta::Term::App(application) => application.lower_inner(meta_bindings),
            meta::Term::Abs(abstraction) => abstraction.lower_inner(meta_bindings),
            meta::Term::Subst(substitution) => substitution.lower_inner(meta_bindings),
        }
    }
}

impl meta::MetaVariable {
    fn lower_inner(self, meta_bindings: &MetaBindings) -> Result<core::Term, LoweringError> {
        Ok(meta_bindings
            .get(&self)
            .ok_or(LoweringError::UnresolvableMetaVariables)?
            .clone())
    }
}

impl meta::Variable {
    fn lower_inner(self, _: &MetaBindings) -> Result<core::Term, LoweringError> {
        let var: core::Variable = self.into();
        Ok(var.into())
    }
}

impl meta::Application {
    fn lower_inner(self, meta_bindings: &MetaBindings) -> Result<core::Term, LoweringError> {
        let Self { lhs, rhs } = self;
        Ok(core::Application {
            lhs: Box::new(lhs.lower_inner(meta_bindings)?),
            rhs: Box::new(rhs.lower_inner(meta_bindings)?),
        }
        .into())
    }
}

impl meta::Abstraction {
    fn lower_inner(self, meta_bindings: &MetaBindings) -> Result<core::Term, LoweringError> {
        let Self { binders, body } = self;
        Ok(curry(
            VecDeque::from_iter(binders.into_iter()),
            body.lower_inner(meta_bindings)?,
        )
        .into())
    }
}

impl meta::Substitution {
    fn lower_inner(self, meta_bindings: &MetaBindings) -> Result<core::Term, LoweringError> {
        let Self {
            body,
            var,
            replacement,
        } = self;
        Ok(body
            .lower_inner(meta_bindings)?
            .subst(&var.into(), &replacement.lower_inner(meta_bindings)?))
    }
}

impl meta::Context {
    fn lower(&self) -> Result<MetaBindings, LoweringError> {
        let mut map = HashMap::new();

        for (metavar, body) in self.iter() {
            let body = body.clone().lower_inner(&map)?;
            map.insert(metavar.clone(), body);
        }

        Ok(map)
    }
}

/// \xyz.e -> \x.\y.\z.e
///
/// Panics when `binders` is empty.
fn curry(mut binders: VecDeque<meta::Variable>, body: core::Term) -> core::Abstraction {
    debug_assert!(!binders.is_empty());

    if binders.len() == 1 {
        return core::Abstraction {
            binder: binders.pop_front().unwrap().into(),
            body: Box::new(body),
        };
    }

    core::Abstraction {
        binder: binders.pop_front().unwrap().into(),
        body: Box::new(core::Term::Abs(curry(binders, body))),
    }
}
