use crate::{ast::Expr, lexer::Lexer, token::Token};
use std::iter::Peekable;

#[derive(Debug)]
enum Error {
    Empty,
    Infix,
    Prefix,
    Unclosed,
    Param(Expr),
}

struct Parser<I: Iterator<Item = Token>>(Peekable<I>);

#[derive(PartialOrd, PartialEq)]
enum Prec {
    Lowest,
    Lam,
    Call,
}

impl Prec {
    fn infix_prec(token: &Token) -> Option<Prec> {
        use Prec::*;
        use Token::*;
        Some(match token {
            Var(_) => Call,
            Dot => Lam,
            LParen | RParen => Lowest,
        })
    }
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(input: I) -> Self {
        Self(input.peekable())
    }

    pub fn parse(mut self) -> Result<Expr, Error> {
        self.parse_expr(Prec::Lowest)
    }

    fn parse_expr(&mut self, prec: Prec) -> Result<Expr, Error> {
        let mut lhs = self.parse_prefix()?;

        while let Some(curr_peeked_tok) = self.0.peek()
            && let Some(cur_peek_tok_prec) = Prec::infix_prec(curr_peeked_tok)
            && prec < cur_peek_tok_prec
        {
            lhs = self.parse_infix(lhs)?;
        }

        Ok(lhs)
    }

    fn parse_prefix(&mut self) -> Result<Expr, Error> {
        use Token::*;
        Ok(match self.0.next() {
            None => return Err(Error::Empty),
            Some(Var(s)) => Expr::Var(s),
            Some(LParen) => {
                let exp = self.parse_expr(Prec::Lowest)?;
                if self.0.next() != Some(RParen) {
                    return Err(Error::Unclosed);
                }
                exp
            }
            Some(Dot) | Some(RParen) => return Err(Error::Prefix),
        })
    }

    fn parse_infix(&mut self, lhs: Expr) -> Result<Expr, Error> {
        use Expr::*;
        Ok(match self.0.peek() {
            None => return Err(Error::Empty),
            Some(Token::RParen) => return Err(Error::Infix),
            Some(Token::Var(_)) => match self.0.next().unwrap() {
                Token::Var(s) => App(Box::new(lhs), Box::new(Var(s))),
                _ => unreachable!(),
            },
            Some(Token::LParen) => {
                let exp = self.parse_expr(Prec::Lowest)?;
                App(Box::new(lhs), Box::new(exp))
            }
            Some(Token::Dot) => {
                self.0.next();
                let expr = self.parse_expr(Prec::Lowest)?;
                match lhs {
                    Expr::Var(s) => Lam(s, Box::new(expr)),
                    e => return Err(Error::Param(e)),
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let tests = [
            ("x", "x"),
            ("x y", "(x y)"),
            ("x.y", "(x.y)"),
        ];

        todo!("write tests, and also set up left vs right assoc");

        for (input, exp) in tests {
            let tokens = Lexer::new(input).collect::<Result<Vec<_>, _>>().unwrap();

            assert_eq!(
                Parser::new(tokens.into_iter()).parse().unwrap().to_string(),
                exp,
            );
        }
    }
}
