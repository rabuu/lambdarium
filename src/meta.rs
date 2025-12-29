use std::collections::{HashMap, VecDeque};

use crate::core;

#[derive(Debug, Clone)]
pub struct Program {
    pub defs: Vec<Definition>,
    pub terms: Vec<Term>,
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub metavar: String,
    pub body: Term,
}

#[derive(Debug, Clone)]
pub enum Term {
    Var(String),
    App {
        lhs: Box<Term>,
        rhs: Box<Term>,
    },
    Abs {
        /// Must be non-empty
        binders: Vec<String>,
        body: Box<Term>,
    },
    Meta(String),
}

impl Term {
    pub fn lower(self, defs: &[Definition]) -> Option<core::Term> {
        let defs = lower_defs(defs)?;
        self.lower_inner(&defs)
    }

    fn lower_inner(self, defs: &HashMap<String, core::Term>) -> Option<core::Term> {
        Some(match self {
            Term::Var(var) => core::Variable::from(var).into(),
            Term::App { lhs, rhs } => core::Application {
                lhs: Box::new(lhs.lower_inner(defs)?),
                rhs: Box::new(rhs.lower_inner(defs)?),
            }
            .into(),
            Term::Abs { binders, body } => {
                curry(VecDeque::from(binders), body.lower_inner(defs)?).into()
            }
            Term::Meta(metavar) => match defs.get(&metavar) {
                Some(term) => term.clone(),
                None => return None,
            },
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

fn lower_defs(defs: &[Definition]) -> Option<HashMap<String, core::Term>> {
    let mut map = HashMap::new();

    for Definition { metavar, body } in defs {
        let body = body.clone().lower_inner(&map)?;
        map.insert(metavar.clone(), body);
    }

    Some(map)
}
