# Introduction

This will be a hackathon project that should be able to be completed in a matter of 2 days.

The goal is to implement a **very** minimal language that is "Turing complete" in Rust.
The language will be **heavily** based on the ideas of lambda calculus. The language will
be kept minimal, and include a STD LIB.

# Process

Plain Text -> Preprocessor -> Lexer -> Parser -> Evaluator -> Simplifier

## Preprocessor

1. Read the file specified in the CLI to a rust string.
2. Prepped the STD LIB file into the plain text. This can be done by using the `assert_str` macro in rust.
3. Find all instances of the following regex in the file `^(.+?)=(.*)$`.
4. Given the captures from the regex above, **in reverse order**, substitute the left side of the `=`
    with the right side surrounded in parenthesis.
5. The resulting string will then be outputted to a file of the same name with the ending of `.chi` instead of
    `.ch`.

Example:
```
IDENT_A = f => x => f x
IDENT_B = f => y => f y
IDENT_B IDENT_A a

to

(f => y => f y) (f => x => f x) a
```

Note: Look into using the regex crate to capture all instances.

## Lexer

The lexer will take the preprocessed code and turn it into tokens.

The tokens are the following:

```rust
enum Token {
    Var(String),
    Arrow,
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
    type Item = Token;
    ...
}
```

Cases of next char:

- ' ' | '\n': skip it (call again w/ recursion)
- '=': if .peek() is '>' then eat both and return arrow.
- '(' | ')': Paren tokens
- else: eat and collect until we hit something that is not `0..9 | a..z | _` and return it as a var.

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
x => y => z
to
(x => (y => z))

Applies are left associative.
x y z
to
((x y) z)

Precedence is as follows.
x => y z => a
to
(x => ((y z) => a))
```

This will be done using pratt parsing with the following precedences.

```rust
enum Prec {
    Lowest, // )
    Lambda, // =>
    Call, // ident
}
```

The pratt parsing will look like the following.

```
// psuedo code
fn parse_expr(self, prec) -> Expr {
    let lhs = parse_prefix();
    while prec < self.curr.prec {
        lhs = parse_infix(lhs);
    }
    return lhs
}
```

## Evaluator

<!-- The goal of this is to take a Expression and simplify it to a single value. -->
<!-- The type structure for the values will be: -->

<!-- ``` -->
<!-- struct Value { -->
<!--     Bound(String), -->
<!--     Free(String, Uuid), -->
<!--     Fn(String, Expression, Env), -->
<!--     App(Box<Value>, Box<Value>), -->
<!-- } -->
<!-- ``` -->

## Simplifier

<!-- Will attempt to show native datatypes. In human readable form. -->
<!-- Such as: -->
<!-- - Church Numerals -->
<!-- - Church Booleans -->
<!-- - Cons List -->
<!-- - Pair -->
<!-- This will be done by doing alpha, beta, and n reductions, and by pattern matching on the -->
<!-- shapes of these data-types. -->

# STD LIB

Necessary Functions

## Arithmetic

- 0-10
- succ
- pred
- add
- sub
- mult
- div

## Boolean

- if
- eq
- is_zero
- true
- false

## Tuple

- pair
- fst
- snd

## List

- cons
- nil

## Loops

- Y combinator

# CLI

Preprocessor: `church process <filename>`

Lexer -> Evaluator: `church run <filename>`

Lexer -> Simplify: `church run <filename> -s` or `church run <filename> --simplify`

NOTE: Clap would be **fire** for this...

# Resources

- <https://youtu.be/ViPNHMSUcog?si=aCoHfzR6RUfWVN-n>
- <https://youtu.be/KuVUfbWoROw?si=KjtZz55prHMU8YeZ>
