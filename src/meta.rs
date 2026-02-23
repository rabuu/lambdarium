use crate::NonEmpty;

#[derive(Debug, Clone)]
pub struct Program {
    pub definitions: Vec<Definition>,
    pub terms: Vec<Term>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Def(Definition),
    Term(Term),
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub metavar: MetaVariable,
    pub body: Term,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    MetaVar(MetaVariable),
    Var(Variable),
    App(Application),
    Abs(Abstraction),
    Subst(Substitution),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetaVariable {
    pub inner: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variable {
    pub inner: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application {
    pub lhs: Box<Term>,
    pub rhs: Box<Term>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abstraction {
    pub binders: NonEmpty<Variable>,
    pub body: Box<Term>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Substitution {
    pub body: Box<Term>,
    pub var: Variable,
    pub replacement: Box<Term>,
}

impl<S> From<S> for Variable
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Self {
            inner: value.into(),
        }
    }
}

impl From<Variable> for crate::core::Variable {
    fn from(value: Variable) -> Self {
        Self::from(value.inner)
    }
}

impl<S> From<S> for MetaVariable
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Self {
            inner: value.into(),
        }
    }
}
