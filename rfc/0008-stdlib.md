# STD LIB

This language is very theoretical. Therefore, we need a lot of basic functions to make the
language useable at all! The goal is to use as little of lambda calculus as possible. Leaning
on primitives as other functions even for a runtime cost.

## Church Numbers

Numbers should go from 0 to 10.

```
0 = f . x. x
1 = f . x . f x
...
10 = ...
```

## Numerical Operators

Addition is cheap, but subtraction takes a little more effort.
Will be left out for now.

```
succ 0 => 1
add 1 2 => 3
mult 2 3 => 6
```

## Boolean

```
true = a . b . a
false = a . b . b
```

## Control Flow

```
if = c.t.f. c t f

if true a b => a
if false a b => b
```

## Tuple

```
pair = a.b.f. f a b
fst = p. p (a.b. a)
snd = p. p (a.b. b)
```

## Lists

Lists are going to be based on how they appear in lisp
with a lot of the same semantics.

cons and nil to build a list
```
cons 1 (cons 2 (cons 3 nil)) => [1, 2, 3]
```

car and cdr to access elements
```
car (cons 1 (cons 2 nil)) => 1
cdr (cons 1 (cons 2 nil)) => [2]
```

Also be sure to include builtin map, filter, and reduce.

## Loops

Y-Combinators needed
```
foo_rec = ...
foo = Y foo_rec
```

## Functions

Function composition, like the `.` operator in haskell.
