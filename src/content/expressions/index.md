# Expressions

Rust is primarily an *expression language*. `if` and `match` expressions are
expressions which evaluate to values instead of statements, unless transformed
into a statement through a trailing semicolon. This alleviates the need for the
ternary operator `<bool_expr> : <expr_if_true> ? <expr_if_false>`.

## Blocks, Semicolons

Blocks in Rust are *expressions*. A block can contain zero or more statements,
and an ending expression or statement; conceptually:

```enbf
<block> 	::= <statements>? <expression>     /* ends with expression */
			|   <statements>? <statement>

<statements> 	::= <statement>
				|   <statement> <statements>

<statement> 	::= <expression> ";"
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

