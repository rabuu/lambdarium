use std::collections::VecDeque;

use indexmap::IndexMap;
use miette::Diagnostic;
use thiserror::Error;

use crate::{core, meta};

#[derive(Error, Diagnostic, Debug)]
pub enum LoweringError {
    #[error("The term contains unresolvable meta-variables.")]
    UnresolvableMetaVariables,
}

#[derive(Debug, Clone, Default)]
pub struct Context {
    defs: IndexMap<meta::MetaVariable, core::Term>,
}

impl Context {
    pub fn add_definition(&mut self, def: meta::Definition) -> Result<(), LoweringError> {
        let body = def.body.lower(self)?;
        self.defs.insert(def.metavar, body);
        Ok(())
    }

    pub fn get(&self, var: &meta::MetaVariable) -> Option<&core::Term> {
        self.defs.get(var)
    }

    pub fn contains(&self, metavar: &meta::MetaVariable) -> bool {
        self.defs.contains_key(metavar)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&meta::MetaVariable, &core::Term)> {
        self.defs.iter()
    }
}

impl meta::Program {
    pub fn lower(self) -> Result<Vec<core::Term>, LoweringError> {
        let mut ctx = Context::default();
        for def in self.definitions {
            ctx.add_definition(def)?;
        }

        self.terms.into_iter().map(|t| t.lower(&ctx)).collect()
    }
}

impl meta::Term {
    pub fn lower(self, ctx: &Context) -> Result<core::Term, LoweringError> {
        match self {
            meta::Term::MetaVar(meta_variable) => meta_variable.lower(ctx),
            meta::Term::Var(variable) => Ok(variable.lower()),
            meta::Term::App(application) => application.lower(ctx),
            meta::Term::Abs(abstraction) => abstraction.lower(ctx),
            meta::Term::Subst(substitution) => substitution.lower(ctx),
        }
    }
}

impl meta::MetaVariable {
    pub fn lower(self, ctx: &Context) -> Result<core::Term, LoweringError> {
        Ok(ctx
            .get(&self)
            .ok_or(LoweringError::UnresolvableMetaVariables)?
            .clone())
    }
}

impl meta::Variable {
    pub fn lower(self) -> core::Term {
        core::Term::Var(self.into())
    }
}

impl meta::Application {
    pub fn lower(self, ctx: &Context) -> Result<core::Term, LoweringError> {
        Ok(core::Application {
            lhs: Box::new(self.lhs.lower(ctx)?),
            rhs: Box::new(self.rhs.lower(ctx)?),
        }
        .into())
    }
}

impl meta::Abstraction {
    pub fn lower(self, ctx: &Context) -> Result<core::Term, LoweringError> {
        let binders = VecDeque::from_iter(self.binders);
        let body = self.body.lower(ctx)?;
        Ok(curry(binders, body).into())
    }
}

impl meta::Substitution {
    pub fn lower(self, ctx: &Context) -> Result<core::Term, LoweringError> {
        let var: core::Variable = self.var.into();
        let replacement = self.replacement.lower(ctx)?;
        Ok(self.body.lower(ctx)?.subst(&var, &replacement))
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
