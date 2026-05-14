## Preprocessor

1. Read the file specified in the CLI to a rust string.
2. Prepend the STD LIB file into the plain text. This can be done by using the `assert_str` macro in rust.
3. Find all instances of the following regex in the file `^(.+?)=(.*)$`.
4. Given the captures from the regex above, **in reverse order**, substitute the left side of the `=`
    with the right side surrounded in parenthesis. This is to be returned as a string for the lexer.

Example:
```
IDENT_A = f . x . f x
IDENT_B = f . y . f y
IDENT_B IDENT_A a

to

(f . y . f y) (f . x . f x) a
```

Note: Look into using the regex crate to capture all instances.
