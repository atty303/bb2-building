use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    Text(String),
    Var(String),
    NewLine,
    /// New line and indent
    Indent,
    Empty,
    Error(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tokens(pub Vec<Token>);

impl Tokens {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push(&mut self, token: Token) {
        self.0.push(token);
    }

    pub fn extend(&mut self, other: Tokens) {
        self.0.extend(other.0);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn has_var(&self) -> bool {
        for token in &self.0 {
            match token {
                Token::Var(_) => return true,
                _ => (),
            }
        }
        false
    }

    pub fn map_var<F: Fn(&mut Tokens, &str) -> ()>(&self, f: F) -> Tokens {
        let mut out = Tokens(vec![]);
        for token in &self.0 {
            match token {
                Token::Var(s) => {
                    let mut subs = Tokens(vec![]);
                    f(&mut subs, &s);
                    if subs.is_empty() {
                        out.push(token.clone());
                    } else {
                        out.extend(subs);
                    }
                }
                _ => out.push(token.clone()),
            }
        }
        out
    }

    pub fn format<F: Fn(&mut Tokens, &str) -> ()>(&self, formatter: F) -> Tokens {
        let mut out: Tokens;
        let mut tokens = self.clone();
        loop {
            if !tokens.has_var() {
                break;
            }

            let mut replaced = false;
            out = Tokens(vec![]);
            for token in &tokens.0 {
                match token {
                    Token::Var(name) => {
                        let mut subs = Tokens(vec![]);
                        formatter(&mut subs, &name);
                        if subs.is_empty() {
                            out.push(token.clone());
                        } else {
                            replaced = true;
                            out.extend(subs);
                        }
                    }
                    _ => out.push(token.clone()),
                }
            }
            if !replaced {
                break;
            }
            tokens = out;
        }
        tokens.clone()
    }
}

impl Display for Tokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for token in &self.0 {
            match token {
                Token::Text(s) => write!(f, "{}", s)?,
                Token::Var(s) => write!(f, "<{}>", s)?,
                Token::NewLine => write!(f, "\n")?,
                Token::Empty => (),
                Token::Error(s) => write!(f, "!{}!", s)?,
                Token::Indent => write!(f, "\n  ")?,
            }
        }
        Ok(())
    }
}
