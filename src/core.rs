#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable),
    App(Application),
    Abs(Abstraction),
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
    pub binder: Variable,
    pub body: Box<Term>,
}

impl From<Variable> for Term {
    fn from(value: Variable) -> Self {
        Self::Var(value)
    }
}

impl From<Application> for Term {
    fn from(value: Application) -> Self {
        Self::App(value)
    }
}

impl From<Abstraction> for Term {
    fn from(value: Abstraction) -> Self {
        Self::Abs(value)
    }
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
