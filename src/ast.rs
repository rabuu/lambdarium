#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable),
    App(Application),
    Abs(Abstraction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Variable(pub char);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application {
    pub first: Box<Term>,
    pub second: Box<Term>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abstraction {
    pub binder: Variable,
    pub body: Box<Term>,
}
