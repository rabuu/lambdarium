#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    ParenOpen,
    ParenClose,
    Lambda,
    Dot,
    Var(char),
}

pub fn lex(source: &str) -> Option<Vec<Token>> {
    let mut tokens = Vec::new();

    for c in source.chars() {
        let token = match c {
            '(' => Token::ParenOpen,
            ')' => Token::ParenClose,
            '\\' | 'λ' => Token::Lambda,
            '.' => Token::Dot,
            x if x.is_ascii_lowercase() => Token::Var(x),
            x if x.is_whitespace() => continue,
            _ => return None,
        };
        tokens.push(token);
    }

    Some(tokens)
}
