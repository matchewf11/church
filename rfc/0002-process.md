# Plain Text

The code that the user writes for the program.

# Preprocessor

This will be able to handle all the macros into one single expression.

# Lexer

This will turn text into a list of tokens.
This should be agnostic to any macros.

# Parser

This should be able to build an ast from a list of tokens.

# Evaluator

This should be able to get the expression in normal form.

# Simplifier

This should be able to make the output prettier
for easier user viewing.
