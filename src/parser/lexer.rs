use std::fmt;

use logos::{Logos, SpannedIter};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum LexerError {
    #[default]
    InvalidToken,
}

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r#"\s*"#)]
#[logos(error = LexerError)]
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

    #[regex(r#""[a-zA-Z]+""#, |lex| strip_quotes(lex.slice()))]
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

pub struct Lexer<'src> {
    tokens: SpannedIter<'src, Token<'src>>,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            tokens: Token::lexer(source).spanned(),
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Spanned<Token<'src>, usize, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens
            .next()
            .map(|(token, span)| Ok((span.start, token?, span.end)))
    }
}

fn strip_quotes(s: &str) -> &str {
    let len = s.len();
    &s[1..len - 1]
}
