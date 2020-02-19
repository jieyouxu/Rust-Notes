# Error Handling

The two basic types of error-handling revolves around Rust's `Result<T, E>` type
and panics.

- Typical recoverable errors can be handled via `Result`s.
- The errors that *should never happen* and when they do occur the program
  should crash are addressed through panics.

## Panics

When a program produces a **panic**, it should indicate that something is wrong
with the program itself such that it could not recover from that fault.

Typical cases look like:

- Array out-of-bounds index access.
- Integer division-by-zero.
- Calling `.unwrap` on an `None` variant of `Option`.
- Failing an `assert`.

> Reference: [std::panic](https://doc.rust-lang.org/std/panic/index.html)

The `panic!()` macro is also provided for the user to trigger a panic.

Typically, panics should *only* be used as a *last-resort* type of deal and
indicate critical, unhandleable failures.

WHen a panic do occurs though, Rust can either

- *Unwind* the stack to produce a *stack backtrace*; or
- Abort the process.

### Unwinding

A typical error message looks like

```
thread 'main' panicked at 'attempt to divide by zero', main.rs:3780
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

As suggested, the user can set the `RUST_BACKTRACE=1` environment variable to
enable debug backtrace dump.

After the error message is printed, all local temporaries, variables and
arguments for the current function in scope is dropped to be cleaned up, and
transitively heap-allocated memory, file handles, and other resources will be
freed up to prevent memory leaks.

When all relevant `drop` handlers are called then the current thread exists with
a non-zero exit code indicating failure; and when the main thread exists, so
does the process.

Panic occurs on a per-thread basis.

> Reference: [std::panic::catch\_unwind](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html)

Recover from panics and to *catch* stack unwinding is provided through
`std::panic::catch_unwind()` and related facilities.

### Aborting

Unwinding the is default behaviour, but if panics are triggered from within
`drop` methods which are responsible for trying to cleanup, then the process is
aborted as this is considered fatal.

Rust can also have customized panic behaviour. When compiled with `-C
panic=abort` then the first panic triggered in the program will abort the
process, which can reduce size of compiled binary.

## Result

> Reference: [std::result](https://doc.rust-lang.org/std/result/index.html)

Rust don't use run-time exceptions to avoid the run-time overhead. Functions use
`Result<T, E>` to encode the possiblity that some computation may fail.

```rust
/// The `Result` enum encodes the possibility of a computation either
/// *succeeding* with the `Ok(T)` variant, or *failing* with the `Err(E)`
/// variant.
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Handling Errors

Instead of try-catch, use `match` on expressions that produces a `Result` to
exhaustively handle both `Ok(T)` and `Err(E)` cases.

Since `match` may be somewhat verbose, `Result<T, E>` defines some helper
methods.

- `result.is_ok() -> bool` and `result.is_err() -> bool` are helper methods for
  checking which variant the `Result` is.
- `result.ok() -> Option<T>` maps the `Result<T, E>` into `Option<T>` when we
  don't need information on the error, and only needs to check if the value is
  present. If the computation is successful, `Some(T)` is returned, and if the
  computation failed, then `None` is returned.
- `result.err() -> Option<E>` maps the `Result<T, E>` into `Option<E>` when we
  are more concerned with the error cause.
- `result.unwrap_or(fallback_value)` returns either the success value, or the
  `fallback_value` instead.
- `result.unwrap_or_else(fallback_producer)` returns either the success value,
  or produces the fallback value from the fallback value produce
  `fallback_producer` closure or function.
- `result.unwrap()` either returns the success value or fails with a panic.
- `result.expect()` is like `.unwrap()` but produces a message on panic.
- `result.as_ref()` maps a `Result<T, E>` into a `Result<&T, &E>` to borrow a
  reference to the success or error values.
- `result.as_mut()` maps a `Result<T, E>` into a `Result<&mut T, &mut E>` to
  borrow a mutable reference to the success or error values.

Apart from the `as_ref()` and `as_mut()` methods, other methods consume the
`result` for which they are invoked upon.

### Type Aliases

Sometimes a module's `Result` type omits the error type because a module's
functions and methods share an error type. This typically happens because the
module defines a type alias such as

```rust
pub type Result<T> = std::result::Result<T, std::io::Error>;
```

### Error Propagation

When errors are meant to be dealt by the caller, or if error-handling should be
deferred, errors then can be *propagated*.

Rust has the `?` operator which either continues with the computation by
evaluating to the successful result, or in the case of failure, returns early
from the function with the error cause.

The `?` operator requires that the enclosing function has a `Result<T, E>`
return type, and that the expressions for which the `?` operator is invoked upon
must have a homogeneous error type.

The `?` operator is merely syntax sugar for

```rust
let _ = match expr {
    Ok(success_value) => success_value,
    Err(err) => return Err(err),
}
```

Sometimes error types may be different and do not automatically convert between
each other.

- It is possible to define custom error types and to implement conversions from
  other types to the custom error. The `error-chain` crate is useful for this
  route.

> Reference: [std::convert::From](https://doc.rust-lang.org/std/convert/trait.From.html)

All std error types can be converted to `Box<std::error::Error>` which can
represent *any std error*.

For example,

```rust
type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;
```

Though in this case the return type masks the actual type of errors that the
caller should expect.

It is possible to selectively handle one error and let other error types
propgate out through the `err.downcast_ref::<E>()` for error type `E`.

