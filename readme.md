# Introduction

The idea of this project would be to make a minimal programming language in Rust.
This will be based on the principles of lambda calculus. This will focus on a very minimal
implementation. With a strong stdlib.

# Types

## Nodes Types:

- Bindings
- Expressions

## Expression Types:

- Vars
- Lambdas (always one arg)
- Applications

# Process

1. Read file from args or repl
    - repl must share environment between repl evals
    - in the repl the bindings should return nothing
    - ends with an expression
    - files also end in the case of an expression

Example (ignore lang specifics)
```
$ cargo run
> add = x => y => x + y
> sub = x => y => x - y
> add 2 4
6
$
```

2. Preprocessing
    - Append stdlib textually prior to any processing
    - textually prepended

3. Lexing
    - Lex string into token types
    - Token Types
        - Ident
        - Arrow
        - Assign
        - Semicolon
        - Left Paren
        - Right Paren

4. Parsing
    - Build a program that includes a list of bindings and one final expression
    - Build into the expressions listed above

5. Executing
    - Each function binding stored in a hashmap (no eval yet)
    - Will apply the lambdas args one by one
    - do simplifiactions where possible
    - will try to find something the scope that shares the same resulting expression
      if able return that instead of the expanded/simplified expression

# Syntax

```
one = f => x => f x
two = f => x => f (f x)
inc n => f => x => f (n f x)
```
