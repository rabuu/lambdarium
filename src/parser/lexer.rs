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
pub enum Token {
    #[token("(")]
    ParenOpen,

    #[token(")")]
    ParenClose,

    #[token(r#"\"#)]
    #[token(r#"λ"#)]
    Lambda,

    #[token(".")]
    Dot,

    #[regex(r#""[a-zA-Z]+""#, |lex| strip_quotes(lex.slice()).to_owned())]
    #[regex(r#"[a-zA-Z]"#, |lex| lex.slice().to_owned())]
    Variable(String),

    #[token("def")]
    Def,

    #[regex(r#"<[a-zA-Z0-9_]+>"#, |lex| strip_quotes(lex.slice()).to_owned())]
    MetaVariable(String),

    #[token("=")]
    Equals,

    #[token(";")]
    Semicolon,

    #[token("[")]
    BracketOpen,

    #[token("]")]
    BracketClose,

    #[token("->")]
    Arrow,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::ParenOpen => write!(f, r#""(""#),
            Token::ParenClose => write!(f, r#"")""#),
            Token::Lambda => write!(f, r#""λ""#),
            Token::Dot => write!(f, r#"".""#),
            Token::Variable(var) => write!(f, "{var}"),
            Token::Def => write!(f, r#""def""#),
            Token::MetaVariable(var) => write!(f, r#"<{var}>"#),
            Token::Equals => write!(f, r#""=""#),
            Token::Semicolon => write!(f, r#"";""#),
            Token::BracketOpen => write!(f, r#""[""#),
            Token::BracketClose => write!(f, r#""]""#),
            Token::Arrow => write!(f, r#""->""#),
        }
    }
}

pub struct Lexer<'src> {
    tokens: SpannedIter<'src, Token>,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            tokens: Token::lexer(source).spanned(),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Spanned<Token, usize, LexerError>;

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
