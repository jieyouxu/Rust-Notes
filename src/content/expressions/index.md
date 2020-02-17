# Expressions

Rust is primarily an *expression language*. `if` and `match` expressions are
expressions which evaluate to values instead of statements, unless transformed
into a statement through a trailing semicolon. This alleviates the need for the
ternary operator `<bool_expr> : <expr_if_true> ? <expr_if_false>`.

## Blocks, Semicolons

Blocks in Rust are *expressions*. A block can contain zero or more statements,
and an ending expression or statement; conceptually:

```enbf
<block>     ::= <statements>? <expression>     /* ends with expression */
            |   <statements>? <statement>

<statements>    ::= <statement>
                |   <statement> <statements>

<statement>     ::= <expression> ";"
```

Or in a pseudo Abstract Syntax Tree:

```rust
type Stmts = Vec<Stmt>;

enum Block {
    StatementBlock(Option<Stmts>, Expr),
    ExpressionBlock(Option<Stmts>, Stmt),
}
```

If a `<block>` ends in an `<expression>`, then the `<block>` evaluates to the
value of the `<expression>`. Otherwise, the statement block variant evaluates to
`()` (unit) representing empty value.

So semicolons do serve a more important role semantically – Rust uses the
semicolon `;` to distinguish between *expressions* and *statements*. Expressions
`e: T` produce values of type `T`, whereas statements always produce unit `()`.

## Declarations

To declare local variables, `let` declarations may be used.

Each `let` declaration has the form

```rust
let identifier: T = expr;
```

Where `identifier` is the name of the local variable of type `T`.

A variable can either be initialized by assigning to it `expr` upon declaration,
or be uninitialized if assignment is to be deferred.

Attempts to use unitialized variables will cause panics – to avoid undefined
behaviour such as accessing arbitary memory regions or runtime segmenatation
faults.

### Name-shadowing

Rust allows `let` bindings for a variable with name `identifier` to override
preceding bindings of the same name – this is called *name shadowing*.

`let` declaration for a variable of the same name recreates a new variable of
possibly a different type.

This is typically useful when transforming a value between different types. For
example,

```rust
let val = "123";
let val = val.parse::<i32>().unwrap();
```

In which case without name shadowing, the programmer will likely need to encode
the type in the variable name just so the two variables don't have name
collisions.

*Use name-shadowing responsibily. Using name-shadowing to improve readability,
not to hinder readability.*

### Item Declarations

Rust permits *item declarations* within blocks. Such *item declaration* can be
`fn`, `struct` and `use`.

```rust
fn outer_fn() -> i32 {
    let a = 2;

    fn increment_by_one(v: i32) -> i32 {
        v + 1
    }

    increment_by_one(a)
}
```

In such cases, when `increment_by_one` is declared within the outer block, its
effective scope is the entire block - that is, analogous to JavaScript's
behaviour in this case, the function declaration and definition is *hoisted* to
the top of the block.

Such nested `fn`s have different scoping rules compared to JavaScript though.
The nested `increment_by_one` does not have access to local variables or
arguments from its enclosing block, because nested `fn`s *do not capture its
context*. If such behaviour is needed, a **closure** is a more suitable
construct.

## `if` and `match` Expressions

### `if` Expression

Rust's `if` expression is similar to C.

```rust
if <predicate_1> {
    <block_expr_1>
} else if <predicate_2> {
    <block_expr_2>
// ...
} else {
    <block_n>
}
```

Rust does *not* permit implicit type casts from arbitary types to `bool`. Each
of the `<predicate_i>` conditions *must* be of type `bool` explicitly. This rule
is enforced for numbers as well.

Rust does not require parentheses for the `<predicate_i>` conditions.

Additionally, Rust requires that for each branch of the `if` expression, that
their respective body expressions must be homomorphic in their types. That is,
if one branch is of type `T`, then each of the other branches must also have
type `V: T` where `V` is compatible with `T`.

### Let Bindings

`if` expression can have `let` bindings to be more expressive.

```rust
if let <pattern> = <expr> {
    <block_1>
} else {
    <block_2>
}
```

This is useful especially for retrieving data out of `struct`s or `enum`s, such
as `Result<T, E>` or `Option<T>`.

```rust
if let Ok(n) = str.parse::<i32>() {
    println!("successfully parsed n = {}", n);
}
```

### `match` Expression

Rust's `match` expression is similar to C `switch` statements but more powerful.

Each `match` expression has the form

```rust
match <expr> {
    <case_expr_1> => <expr_body_1>,
    // ...
    _ => <expr_body_n>
}
```

Where `_` serves as the *catch-all* / *default* branch to guarantee exhaustive
matching. This means that if the overall `match` expression has type `T`, then
each of the branches must yield a value of type `V: T` that is compatible with
`T`.

```rust
match statusCode {
    403 => HttpStatus::Forbidden,
    404 => HttpStatus::NotFound,
    500 => HttpStatus::InternalServerError,
    _ => HttpStatus::Unknown,
}
```

Such trivial cases are optimized through *jump tables* or *branch tables* to
guarantee \\( \mathcal{O}(1) \\) time complexity.

Rust `match` also supports *pattern matching*, where the pattern match can occur
in the left-hand-side of the `=>` in each *arm* or branch of the `match`
expression.

```rust
match str_val.parse::<i32>() {
    Ok(value) => println!("value = {}", value),
    Err(_) => println!("parsing failed!"),
}
```

Rust requires that the `match` expression must be *exhaustive*. `match`
expressions that does not cover all the possible cases will cause the compiler
to emit an error at compile-time.

## Loops

Rust has three loop variants:

1. `while` loop.
2. `loop` (sugar for `while true`).
3. `for` loop.

Loops in Rust are also expressions, but they all produce `()` and do not
evaluate to other values.

### `while` Loop

Rust `while` loop is like C, except that the condition expression must evaluate
to type `bool`, and supports `let` bindings as well.

```rust
while <bool_expr> {
    // ...
}

while let <pattern> = <expr> {
    // ...
}
```

### `loop` loop

Infinite loops (aka `while true`) can be written via the `loop` syntax sugar.

```rust
loop {
    // ...
}
```

### `for` loop

> Reference: [std::iter::Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)

```rust
for <pattern> in <iterable_expr> {
    <body_expr>
}
```

Rust's `for` loop iterates over its expression that evaluates to an `Iterator`.

The minimal definition for Rust's `Iterator` trait is given as

```rust
pub trait Iterator {
    type Item;

    pub next(&mut self) -> Option<Self::Item>;
}
```

Note `mut` is needed because `next` should advance the iterator which will
change its internal state. The possibility that the `Iterator` finishes
iterating is encoded by the return type `Option<Self::Item>`, and is expressed
with the value `None`.

Interally, Rust desugars the `for` expression like so:

```
while let Some(value) = iterable_expr.next() {
    body_expr
}
```

Rust has sugar for **range**s, where each *range* is a struct with fields
`start` and `end`.

> Reference: [std::ops::Range](https://doc.rust-lang.org/std/ops/struct.Range.html)

The `std::ops::Range` `struct` is defined (simplified) as:

```rust
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Range<T>
    where T: Clone + PartialEq + Eq + Hash
{
    start: T;
    end: T;
}
```

And the operator `<begin?> .. <end?>` is syntax sugar for using the
`std::ops::Range` `struct`, where `Range<T>: Iterator<T>`.
