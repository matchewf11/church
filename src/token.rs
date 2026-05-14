type Symbol = String;

#[derive(Debug, PartialEq)]
pub enum Token {
    Var(Symbol),
    Dot,
    LParen,
    RParen,
}
