# Copy Types

Types such as large structs, vectors, strings and other heap-allocated data
types may be expensive to copy and so they are more suited for *move*s.

Some types such as integers and characters are cheap to copy, which may make it
more efficient compared to a move.

```rust
let x: i32 = 1; 	// not dropped
let y = x; 			// value of `x` copied into `y`
```

## The `Copy` Trait

> Reference: [std::marker::Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html)

Any type which implements the `Copy` trait will be copied when assigning a value
instead of being moved, leaving the source of the assignment still initialized
and the target of the assignment receiving a copy.

Note that only types which can be copied bit-for-bit can properly qualify as a
`Copy` type.

### User-Defined Copy Types

Rust makes user-defined `struct`s and `enum`s are not `Copy` by default.

- If all of the fields of the `struct` is `Copy`; or
- if all of the variants of the `enum` is `Copy`; then

The user can *mark* the user-defined `struct` or `enum` as a `Copy` type by
annotating the type definition with the `#[derive(Copy)]` attribute.

```rust
#[derive(Copy)]
struct Label { id: u32 }
```

