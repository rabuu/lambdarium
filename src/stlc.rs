use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::core::{Term, Variable};
use crate::print::Print;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Var(TypeVariable),
    Fun { lhs: Box<Type>, rhs: Box<Type> },
}

impl Type {
    fn occurs_check(&self, var: &TypeVariable) -> bool {
        match self {
            Type::Var(type_variable) => type_variable == var,
            Type::Fun { lhs, rhs } => lhs.occurs_check(var) || rhs.occurs_check(var),
        }
    }

    fn substitute(&mut self, var: &TypeVariable, with: Type) {
        match self {
            Type::Var(type_variable) => {
                if type_variable == var {
                    *self = with
                }
            }
            Type::Fun { lhs, rhs } => {
                lhs.substitute(var, with.clone());
                rhs.substitute(var, with);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeVariable {
    id: usize,
}

impl fmt::Display for TypeVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "?{}", self.id)
    }
}

impl Term {
    pub fn get_type(&self) -> Option<Type> {
        let mut collector = ConstraintCollector::default();
        let mut env = HashMap::new();
        let (ununified_type, constraints) = collector.collect_constraints(&mut env, self)?;
        let constraints = constraints.into_iter().collect();
        let mgu = Unifier::most_general(constraints)?;
        let unified_type = mgu.apply(ununified_type);
        Some(unified_type)
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Constraint {
    lhs: Type,
    rhs: Type,
}

impl Constraint {
    fn substitute(&mut self, var: &TypeVariable, with: Type) {
        self.lhs.substitute(var, with.clone());
        self.rhs.substitute(var, with);
    }
}

impl fmt::Debug for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} = {}",
            self.lhs.print_verbose(),
            self.rhs.print_verbose()
        )
    }
}

#[derive(Debug, Default)]
struct ConstraintCollector {
    fresh_counter: usize,
}

impl ConstraintCollector {
    fn collect_constraints(
        &mut self,
        env: &mut HashMap<Variable, Type>,
        t: &Term,
    ) -> Option<(Type, HashSet<Constraint>)> {
        match t {
            Term::Var(variable) => {
                let typ = env.get(variable)?;
                Some((typ.clone(), HashSet::default()))
            }
            Term::App(application) => {
                let (lhs_typ, lhs_constrs) = self.collect_constraints(env, &application.lhs)?;
                let (rhs_typ, rhs_constrs) = self.collect_constraints(env, &application.rhs)?;

                let fresh = self.fresh();

                let new_constr = Constraint {
                    lhs: lhs_typ,
                    rhs: Type::Fun {
                        lhs: Box::new(rhs_typ),
                        rhs: Box::new(Type::Var(fresh)),
                    },
                };
                let mut constrs = HashSet::new();
                constrs.extend(lhs_constrs);
                constrs.extend(rhs_constrs);
                constrs.insert(new_constr);

                Some((Type::Var(fresh), constrs))
            }
            Term::Abs(abstraction) => {
                let fresh = self.fresh();

                let mut extended_env = env.clone();
                extended_env.insert(abstraction.binder.clone(), Type::Var(fresh));

                let (typ, constrs) =
                    self.collect_constraints(&mut extended_env, &abstraction.body)?;

                Some((
                    Type::Fun {
                        lhs: Box::new(Type::Var(fresh)),
                        rhs: Box::new(typ),
                    },
                    constrs,
                ))
            }
        }
    }

    fn fresh(&mut self) -> TypeVariable {
        let fresh = self.fresh_counter;
        self.fresh_counter += 1;
        TypeVariable { id: fresh }
    }
}

#[derive(Debug)]
struct Unifier {
    map: HashMap<TypeVariable, Type>,
}

impl Unifier {
    pub fn most_general(mut constrs: Vec<Constraint>) -> Option<Self> {
        let mut mgu = Self {
            map: HashMap::new(),
        };

        while let Some(Constraint { lhs, rhs }) = constrs.pop() {
            if lhs == rhs {
                continue;
            }

            match (lhs, rhs) {
                (Type::Var(var), typ) | (typ, Type::Var(var)) if !typ.occurs_check(&var) => {
                    for c in constrs.iter_mut() {
                        c.substitute(&var, typ.clone());
                    }
                    mgu.add(var, typ);
                }
                (
                    Type::Fun {
                        lhs: lhs1,
                        rhs: rhs1,
                    },
                    Type::Fun {
                        lhs: lhs2,
                        rhs: rhs2,
                    },
                ) => {
                    constrs.push(Constraint {
                        lhs: *lhs1,
                        rhs: *lhs2,
                    });
                    constrs.push(Constraint {
                        lhs: *rhs1,
                        rhs: *rhs2,
                    });
                }
                _ => return None,
            }
        }

        Some(mgu)
    }

    fn apply(&self, typ: Type) -> Type {
        match typ {
            Type::Var(type_variable) => {
                let Some(subst) = self.map.get(&type_variable) else {
                    return typ;
                };
                self.apply(subst.clone())
            }
            Type::Fun { lhs, rhs } => Type::Fun {
                lhs: Box::new(self.apply(*lhs)),
                rhs: Box::new(self.apply(*rhs)),
            },
        }
    }

    fn add(&mut self, var: TypeVariable, typ: Type) {
        self.map.insert(var, typ);
    }
}
