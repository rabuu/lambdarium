use crate::core::Term;

#[derive(Debug, Clone)]
pub struct Program {
    pub lets: Vec<Let>,
    pub terms: Vec<Term>,
}

#[derive(Debug, Clone)]
pub struct Let {
    pub metavar: MetaVariable,
    pub body: Term,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetaVariable {
    pub inner: String,
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
