use indexmap::IndexMap;
use nonempty::NonEmpty;

#[derive(Debug, Clone)]
pub struct Program {
    pub context: Context,
    pub terms: Vec<Term>,
}

#[derive(Debug, Clone, Default)]
pub struct Context {
    defs: IndexMap<MetaVariable, Term>,
}

impl Context {
    pub fn add_definition(&mut self, def: Definition) {
        self.defs.insert(def.metavar, def.body);
    }

    pub fn contains(&self, metavar: &MetaVariable) -> bool {
        self.defs.contains_key(metavar)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&MetaVariable, &Term)> {
        self.defs.iter()
    }
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
