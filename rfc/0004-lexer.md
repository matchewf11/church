## Lexer

The lexer will take the preprocessed code and turn it into tokens.

The tokens are the following:

```rust
enum Token {
    Var(String),
    Dot,
    LParen,
    RParen,
}
```

The lexer will make a newtype of a peekable byte iterator.

```rust
struct Lexer<'a>(Peekable<Bytes<'a>>);

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self(input.bytes().peekable())
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Error>;
    ...
}
```

Cases of next char:

- ' ' | '\n': skip it (call again w/ recursion)
- '(' | ')': Paren tokens
- '.': Dot Token
- else: eat and collect until we hit something that is not `0..9 | a..z | _` and return it as a var.
