use std::collections::{HashMap, VecDeque};

use miette::Diagnostic;
use thiserror::Error;

use crate::{core, meta};

#[derive(Error, Diagnostic, Debug)]
pub enum LoweringError {
    #[error("The term contains unresolvable meta-variables.")]
    UnresolvableMetaVariables,
}

impl meta::Program {
    pub fn lower(self) -> Result<Vec<core::Term>, LoweringError> {
        let defs = lower_defs(&self.defs)?;

        self.terms
            .into_iter()
            .map(|t| t.lower_inner(&defs))
            .collect()
    }
}

impl meta::Term {
    pub fn lower(self, defs: &[meta::Definition]) -> Result<core::Term, LoweringError> {
        let defs = lower_defs(defs)?;
        self.lower_inner(&defs)
    }

    fn lower_inner(self, defs: &HashMap<String, core::Term>) -> Result<core::Term, LoweringError> {
        Ok(match self {
            meta::Term::Var(var) => core::Variable::from(var).into(),
            meta::Term::App { lhs, rhs } => core::Application {
                lhs: Box::new(lhs.lower_inner(defs)?),
                rhs: Box::new(rhs.lower_inner(defs)?),
            }
            .into(),
            meta::Term::Abs { binders, body } => {
                curry(VecDeque::from(binders), body.lower_inner(defs)?).into()
            }
            meta::Term::Meta(metavar) => defs
                .get(&metavar)
                .ok_or(LoweringError::UnresolvableMetaVariables)?
                .clone(),
            meta::Term::Subst {
                term,
                var,
                replacement,
            } => term
                .lower_inner(defs)?
                .subst(&core::Variable::from(var), &replacement.lower_inner(defs)?),
        })
    }
}

/// \xyz.e -> \x.\y.\z.e
///
/// Panics when `binders` is empty.
fn curry(mut binders: VecDeque<String>, body: core::Term) -> core::Abstraction {
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

fn lower_defs(defs: &[meta::Definition]) -> Result<HashMap<String, core::Term>, LoweringError> {
    let mut map = HashMap::new();

    for meta::Definition { metavar, body } in defs {
        let body = body.clone().lower_inner(&map)?;
        map.insert(metavar.clone(), body);
    }

    Ok(map)
}
