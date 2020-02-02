# Basics

> Reference: Programming Rust

## Function

Rust adopts a C-like syntax, with the familiar scoping-by-braces syntax.

> **Greatest common divisor**
>
> ```rust
> fn gcd(mut m: u64, mut n: u64) -> u64 {
> 	assert!(m != 0 && n != 0);
> 	while m != 0 {
> 		if m < n {
> 			let t = m;
> 			m = n;
> 			n = t;
> 		}
> 		m %= n;
> 	}
> 	n
> }
>
> fn main() {
> 	println!("gcd({}, {}) = {}", 10, 4, gcd(10, 4));
> }
> ```
>
> *Source*: Programming Rust

A few things to note here:

- The `fn` keyword declares a function of the name `gcd`, and the block
  delimited by braces contains the function definition for `gcd`.
- In Rust, functions have a type signature - such type signatures are very
  useful both in helping the compiler to typecheck as well as to help the
  reader of the code to understand what the function does.
	+ The `gcd` function has the function signature

		```
		gcd(mut m: u64, mut n: u64) -> u64
		```

		meaning `gcd` takes two (2) parameters, two unsigned 64-bit integers
		(as denoted by `u64`) and returns an unsigned 64-bit integer.

	+ A name is associated with a type via the colon `:`, e.g. `m: u64`
	  declares `m` to be of type `u64`.
	+ The `gcd` function returns an unsigned 64-bit integer, as denoted by
	  the type `u64` after the arrow `->`.

## Rust Numbers

### Integers

The Rust machine integer types represent their size in bits and signedness:

- Signed integers:
	+ `i8`: 8-bit signed integer
	+ `i16`: 16-bit signed integer
	+ `i32`: 32-bit signed integer
	+ `i64`: 64-bit signed integer
	+ `i128`: 128-bit signed integer
- Unsigned integers:
	+ `u8`: 8-bit unsigned integer
	+ `u16`: 16-bit unsigned integer
	+ `u32`: 32-bit unsigned integer
	+ `u64`: 64-bit unsigned integer
	+ `u128`: 128-bit unsigned integer

Rust also has platform-dependent pointer-sized integer types:

- `isize`: for pointer-sized signed integers (platform-dependent).
- `usize`: for pointer-sized unsigned integers (platform-dependent).

### Floating-point Numbers

Rust has two floating-type numbers:

- `f32`: 32-bit single-precision floating-point number
- `f64`: 64-bit double-precision floating-point number

These both conform to the
[IEEE-754](https://standards.ieee.org/content/ieee-standards/en/standard/754-2019.html)
standard for floating-point arithmetic.

## Variables

Rust variables are *immutable by default*, like some of the popular functional
programming languages.

- An immutable variable declaration looks like:

	```rust
	let variable_name: variable_type;
	```

- A mutable variable declaration looks like:

	```rust
	let mut variable_name: variable_type;
	```

The `mut` keyword declares that the variable can be mutated. This also serve as
good documentation.

## Macros

Rust supports macros.

In the `gcd` example, we made a call to the `assert!` macro which ensures that
neither arguments to `gcd` may be zero (dividing-by-zero is a no-no!).

The `assert!` macro checks if its argument is `true`, and if so terminates the
program with a **panic**, outputing useful information including line number
and source location of where the assertion failed.

- This is useful when your program absolutely *cannot continue to execute* and
  needs to instantly crash (perhaps some out-of-memory allocation error?).
- There are ways for error-handling which are less intrusive.

Assertions via `assert!` in Rust are always checked regardless of compilation
mode (i.e. doesn't matter if it is `--debug` mode or `--release` optimized
mode).

Rust also has `debug_assertion!` which are optimized away if compiled for
`--release` mode.

## Let Statements

The `let` statement declares a local variable.

In the `gcd` function, we did not need to explicitly annotate the type of
the temporary variable `t` with `u64` in the assignment

```rust
let t = m;
```

because the compiler uses *type inference* to infer that `t` is of the same
type as variable `m` for which it is assigned to.

- We *could* annotate the type `t` with `u64`, but in this case it doesn't
  really contribute to readibility and has no documentation value, and only
  adds noise and verbosity to the code.

  ```rust
  let t: u64 = m;
  ```

## Returns and Value of Blocks

Rust has the `return` statement, like that of C, but the `gcd` function does
not need one because the expression `n` is at the *logical end* of the
definition block of the `gcd` function.

In Rust, if the body of a function ends with an *expression* (no semicolon!)
instead of an statement (which is an expression followed by the semicolon `;`),
that expression becomes the return value of the function.

More generally, the last expression of a block *is* the value of the block
when it is evaluated.

```rust
fn main() {
	let block_value = {
		println!("inside block!");
		2
	};
	println!("block_value = {}", block_value);
}
```

## Testing and Attributes

Rust has support for basic testing built into the language via the `#[test]`
**attribute**.

To unit test our gcd function (which is pure because we get the same output
if the inputs remain the same and we do not perform side-effects), we can write

```rust
#[test]
fn test_gcd() {
	assert_eq!(gcd(14, 15), 1);
	assert_eq!(gcd(1, 1), 1);
}
```

- The function `test_gcd` exercises `gcd` and checks its actual return value
  against the expected return value.
- The `#[test]` attribute marks `test_gcd` as a *test function*, which is
  skipped in normal compilations but is compiled and invoked when we run
  the tests for the package via the command:

  ```bash
  $ cargo test
  ```

**Attributes** such as `#[test]` are *compiler directives* for supplying
additional information to the compiler, e.g. for controlling warnings,
adjusting code style and conditional compilation, toggling experimental
language features, interoping with external code via FFI, etc.
