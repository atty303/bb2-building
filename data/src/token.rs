#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Text(String),
    Var(String),
    NewLine,
    Empty,
    Error(String),
}

pub type Tokens = Vec<Token>;

pub trait TokensExt {
    fn map_var<F: Fn(&str) -> Vec<Token>>(&self, f: F) -> Vec<Token>;
}

impl TokensExt for Tokens {
    fn map_var<F: Fn(&str) -> Vec<Token>>(&self, f: F) -> Vec<Token> {
        self.iter().flat_map(|n| match n {
            Token::Var(s) => {
                let adds = f(s.as_str());
                if adds.is_empty() {
                    vec![n.clone()]
                } else {
                    let mut out = vec![];
                    // out.push(Node::Text(format!("<{}:", s)));
                    out.extend(adds);
                    // out.push(Node::Text(">".to_string()));
                    out
                }
            }
            n => vec![n.clone()],
        }).collect::<Vec<_>>()
    }
}
