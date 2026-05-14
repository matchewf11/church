## Parser

The goal of the parser is to turn the stream of tokens into a single expression.

```rust
enum Expr {
    Lambda(String, Box<Expr>)
    Apply(Box<Expr>, Box<Expr>)
    Var(String),
}
```

It will take a similar structure to the peekable iterator from the lexer.

```
Lambdas are right associative.
x . y . z
to
(x . (y . z))

Applies are left associative.
x y z
to
((x y) z)

Precedence is as follows.
x . y z . a
to
(x . ((y z) . a))
```

This will be done using pratt parsing with the following precedences.

```rust
enum Prec {
    Lowest, // )
    Lambda, // .
    Call, // ident
}
```

The pratt parsing will look like the following.

```
// psuedo code
fn parse_expr(self, prec) -> Expr {
    let lhs = self.parse_prefix();
    while prec < self.curr.prec {
        lhs = self.parse_infix(lhs);
    }
    return lhs
}
```
