use std::fmt;

type BExpr = Box<Expr>;
type Name = String;

#[derive(Debug)]
pub enum Expr {
    Lam(Name, BExpr),
    App(BExpr, BExpr),
    Var(Name),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expr::*;
        match self {
            Lam(n, e) => write!(f, "({n}.{e})"),
            App(a, b) => write!(f, "({a} {b})"),
            Var(n) => write!(f, "{n}"),
        }
    }
}
