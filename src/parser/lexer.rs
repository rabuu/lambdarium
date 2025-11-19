use std::fmt;

use logos::Logos;

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r#"\s*"#)]
pub enum Token<'src> {
    #[token("(")]
    ParenOpen,

    #[token(")")]
    ParenClose,

    #[token(r#"\"#)]
    #[token(r#"λ"#)]
    Lambda,

    #[token(".")]
    Dot,

    #[regex(r#""[a-zA-Z]+""#)]
    #[regex(r#"[a-zA-Z]"#)]
    Variable(&'src str),
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::ParenOpen => write!(f, r#""(""#),
            Token::ParenClose => write!(f, r#"")""#),
            Token::Lambda => write!(f, r#""λ""#),
            Token::Dot => write!(f, r#"".""#),
            Token::Variable(var) => write!(f, "{var}"),
        }
    }
}
