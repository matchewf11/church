use crate::token::Token;
use std::{iter::Peekable, str::Bytes};

pub struct Lexer<'a>(Peekable<Bytes<'a>>);

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self(input.bytes().peekable())
    }

    fn read_ident(&mut self) -> String {
        let mut res = String::new();
        while let Some(c) = self.0.peek()
            && c.is_ascii_alphanumeric()
        {
            res.push(self.0.next().unwrap() as char);
        }
        res
    }
}

#[derive(Debug)]
pub struct Error;

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;
        Some(Ok(match self.0.peek()? {
            b' ' | b'\n' | b'\t' => {
                self.0.next();
                return self.next();
            }
            b'(' => {
                self.0.next();
                LParen
            }
            b')' => {
                self.0.next();
                RParen
            }
            b'.' => {
                self.0.next();
                Dot
            }
            _ => Var(self.read_ident()),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    #[test]
    fn test_lexer() {
        let input = "(x.foo x)";
        assert_eq!(
            Lexer::new(input).collect::<Result<Vec<_>, _>>().unwrap(),
            vec![
                LParen,
                Var("x".to_string()),
                Dot,
                Var("foo".to_string()),
                Var("x".to_string()),
                RParen,
            ],
        );
    }
}
