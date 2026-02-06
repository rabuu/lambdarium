# λ-calculator
A calculator for the λ-calculus.

This is a tool to evaluate and play with pure λ-calculus terms.

## Syntax
This project differentiates between two types of λ-terms:
- The internal language (called _core_) that is just pure λ-calculus is used for all the actual operations and transformations.
- The surface language (called _meta_) is used for user input and gets automatically lowered to _core_. It supports features like permissive parenthesization, meta-definitions and substitution.

This is an example meta-program:
```
def <id> = \x.x;
\y.<id>(yy)
```

This would get lowered to the pure λ-term `(λy.((λx.x)(y y)))`.

## Main Operations
- β-reduction / evaluation
- Type inference in [STLC](https://en.wikipedia.org/wiki/Simply_typed_lambda_calculus)
