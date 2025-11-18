#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term<'src> {
    Var(Variable<'src>),
    App(Application<'src>),
    Abs(Abstraction<'src>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Variable<'src> {
    pub inner: &'src str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application<'src> {
    pub first: Box<Term<'src>>,
    pub second: Box<Term<'src>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abstraction<'src> {
    pub binder: Variable<'src>,
    pub body: Box<Term<'src>>,
}
