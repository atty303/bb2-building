#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Text(String),
    Var(String),
    NewLine,
    Empty,
    Error(String),
}
