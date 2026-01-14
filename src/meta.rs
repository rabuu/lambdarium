#[derive(Debug, Clone)]
pub struct Program {
    pub defs: Vec<Definition>,
    pub terms: Vec<Term>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Def(Definition),
    Term(Term),
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
