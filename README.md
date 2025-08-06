# Rust dylint rule to detect negation of integral types

### What it does

This rust detects the use of the unary operator `!` on integral types other than `bool`.

For instance, this code will be flagged:

```
let flags = 1u32;
let new_flags = !flags;
```

### Why is this bad?

It might be unexpected that `!` does bitwise negation on integral types. Most
other programming languages use `!` for logical negation.

In particular, it's easy to make a mistake when converting code from a
different programming language into Rust.

### Requirements

Install dylint:

```
cargo install cargo-dylint
```

### Known problems

Tests don't pass due to linking issues.

Only a specific nightly Rust version is supported. It identifies as
1.87-nightly and doesn't support features of newer stable Rust releases, such
as let chains.

### Example

When using this repository, change directory to the source you want to check
and run

```
cargo dylint --git https://github.com/proski/no_integer_negation
```

If you have a local clone of this repository, run

```
cargo dylint --path /PATH/TO/no_integer_negation
```
