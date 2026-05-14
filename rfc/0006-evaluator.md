## Evaluator

The goal of this is to take a expression and simplify it to a single value.
Then perform alpha, beta, and n reductions, and hopefully get the expression in normal form.

The type structure for the values will be the folllowing:

```rust
type Symbol = String;
struct Value {
    Bound(usize), // will use De Bruijn indices
    Free(Symbol),
    Fn(Symbol, Expression),
    App(Box<Value>, Box<Value>),
}
```

I will implement this using a substitution based method.

Some Notable Tests:

- recursion
- `(x . x x) (x . x x)`
- the entire stdlib
