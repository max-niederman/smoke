# Language Design

Smoke is, except as otherwise specified, identical to _Crafting Interpreter's_ Lox programming language.

## Statements & Expressions

The most important way in which Smoke deviates from Lox's design is in its expressions. In Lox, everything is an expression. All side effects should be reflected in the form of a return value.

## Data Types

In addition to double-precision floating points, there is also the `Integer` type for integers.

## Identifiers

`fun` is replaced by `fn`.
`var` is replaced by `let`.

## Static Typing

Unlike Lox, Smoke is statically typed. This prevents mistakenly using the wrong type. Type definitions use the ubiquitous syntax:

```smoke
fn foo(bar: Integer) -> String {
  // ...
}

let baz: String = foo(0);
```

Type inference is allowed everywhere by omission.

## Control Flow

A Rust-like `match` will be added for pattern matching.

## Object Orientation

Smoke has no concept of objects, instead using the functional paradigm.
