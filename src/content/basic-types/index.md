# Basic Types

> Reference: Programming Rust

Rust has a few goals for the design of its type system, namely:

1. **Safety**:

	Rust has a *strong*, *static* type system which helps to rule out some
	classes of mistakes at compile-time (e.g. spelling mistakes!). For notorious
	errros such as `null` references and pointers, Rust adopts type-level
	guarantees such as the `Option` type and the `Result` type to force the user
	to handle the different possibilities instead of delaying to exceptions in
	run time.

2. **Efficiency**:

	Rust's type system is strong enough to allow fine-grained control over the
	level of abstractions that is suitable to the programmer's problem domain.
	When the programmer needs bare-metal / machine-level access, Rust has
	support for such needs. Conversely, Rust's type system also supports
	higher-level constructs such as *Algebraic Data Types* (via `enum`),
	`struct`s, as well as `trait`s and generics. Currently, Rust also has
	experimental support for associated types and constants, and is exploring
	support towards *dependent types*, *refinement types*, *generic ADTs*,
	*higher-kinded types* and more.

3. **Concision**:

	Rust has a good level of type inference built into the compiler. This means
	that the Rust compiler is able to perform much of the type inference
	requirement so users don't have to give explicit type signatures.

As of right now, Rust is **compiled*.

## List of Basic Types

| Type                               | Description                                                            | Example Value(s)                  |
| --- | --- | --- |
| `i8`, `i16`, `i32`, `i64`          | Signed integers, fixed number of bits                                  | `-32`                             |
| `u8`, `u16`, `u32`, `u64`          | Unsigned integers, fixed number of bits                                | `0`                               |
| `isize`, `usize`                   | Machine-sized unsigned and signed integers                             | `16`                              |
| `f32`, `f64`                       | IEEE-754 compliant floating-point numbers                              | `1.45`, `1.4f32`                  |
| `bool`                             | Boolean                                                                | `true`, `false`                   |
| `char`                             | UTF-8 character, 32-bits                                               | `*`                               |
| `()`                               | Unit type                                                              | `()`                              |
| `(char, i32)`                      | Tuple type (mixed product type)                                        | `('b', 0)`                        |
| `struct S { x: i32, y: f32 }`      | Struct with named fields                                               | `S { x: 0, y: 1.0 }`              |
| `struct T(i32, char);`             | Tuple-like structure (gives name)                                      | `T(0, 'a')`                       |
| `struct E`                         | Unit-like struct, no fields, zero-sized                                | `E`                               |
| `enum Option<T> { Some(T), None }` | Enum, algebraic data type (ADT)                                        | `Option::Some(2)`, `Option::None` |
| `Box<Attend>`                      | `Box` is an owning-pointer to some value allocated in heap             | `Box::new(3)`                     |
| `&T`                               | Immutable shared reference to some type `T`, non-owning                | `&name`                           |
| `&mut T`                           | Mutable reference to some type `T`, non-owning                         | `&mut name`                       |
| `String`                           | Dynamically-sized UTF-8 string, owned                                  | `"Hello World"`                   |
| `&str`                             | Non-owning reference to `str`                                          | `&s[0..12]`                       |
| `[T; N]`                           | Fixed-length array of homogeneous type `T` of size `N`                 | `[0; 256]`                        |
| `Vec<T>`                           | Dynamically-sized vector of homogeneous type `T`                       | `vec![0, 1, 2]`                   |
| `&[T]`                             | Immutable reference to a *slice*, a view into (a part of) an array     | `&v[1..3]`, `&x[..]`              |
| `&mut [T]`                         | Mutable reference to a *slice*                                         | `&mut v[1..3]`                    |
| `&Trait`                           | Immutable reference to any value which implements the trait `Trait`    | `value as &Any`                   |
| `&mut Trait`                       | Mutable reference to any value which implements the trait `Trait`      | `value as &mut Any`               |
| `fn(<param>, ...) -> R`            | Pointer to function with parameters `<param>, ...` and return type `R` | `i32::saturating_add`             |
| `|<param>, ...| <expr>`            | Closure capturing parameters `<param, ...` with body `<expr>`          | `| a, b | a + b`                  |

## Note on `usize` and `isize`

These two types, `usize` and `isize`, correspond to the size of a pointer on the
target platform – meaning that different target platforms may produce different
sizes.

## Arithmetic Overflow Checks

In *debug* builds, Rust checks for integer overflows and would produce a panic.

However, in *release* builds, overflows and underflows would cause *wrap*
behaviour. This should *not* be relied upon, and instead if the wrap behaviour
is intended, the `wrapping_add` method in `i*` and `u*` types should be used.

## Integer and Floating-Point Number Literals

### Suffixes

Integer literals may have their types appended, such as `32u8` to indicate the
size intended. The appended suffixes may also be omitted in which Rust will try
to infer the most suitable type, usually defaulting to the corresponding `isize`
which is usually the most efficient integer size on the target platform.

### Prefixes

Rust allows binary, octet and hexadecimal literals as well, by prefixing `0b`,
`0o` and `0x` prefixes respectively.

### Underscores

To separate the digits, Rust allows underscores `_` in between digits which are
discarded, e.g.`1_000_000`.

### Byte Literals

Rust allows *byte literals* by prepending a `b` in front of single quote, e.g.
`b'\''` which is the numerical value of a single quote in ASCII.

### Type Casting

Conversions between different numerical types are possible through type casts
via the `as` operator, e.g. `10u8 as u16`.

## Boolean Type

The `bool` type in Rust is very strict – no implicit conversions from other
types to `bool` is predefined, so there are no notions of "truthy" or "falsey"
values.

## Characters and Strings

Unlike in C, Rust's `char` type is a UTF-8 character, meaning that `char` is
32-bits or 4-bytes.

Rust `String` is *not* a vector of `char`s; instead, `String` is a vector of
UTF-8 bytes because some characters could be double-width.

Typical escape characters are supported in character and string literals.

### Escape Sequences

| Character       | Literal |
| --------------- | ------- |
| Single Quote    | `'\''`  |
| Backslash       | `'\\'`  |
| Newline         | `'\n'`  |
| Carriage return | `'\'r`  |
| Tab             | `'\t'`  |

Additionally, the code point could be written out:

- For Unicode point between `U+0000` and `U+007F`, characters may be written as
  `\xHH` with `HH` being two hex digits.
- For Unicode point beyond that range, characters may be written as `\u{HHHHHH}`
  where `HHHHHH` are hex digits.

A `char` may *not* be a surrogate pair half.

## Pointer Types

Rust does *not* have garbage collection to minimize heap allocations and stack
allocations are preferred by default.

### References

A `&T` is a shared (immutable) reference to some type `T`. It can point to some
memory location either in the stack of the heap.

- `&x` *borrows* a reference to `x`.
- Given some reference `r`, `*r` dereferences the reference to get its value.

Rust references are *never* `null` and immutable by default.

A `&mut T` is a mutable reference to some type `T`.

By tracking ownership + lifetimes of values, Rust can avoid mistakes such as:

- dangling pointers;
- double frees; and
- pointer invalidation.

### Boxes

`Box`es are the simplest way to perform a heap allocation.

```rust
let x = 12;
// b: Box<i32>
let b = Box::new(x);
```

When `b` goes out of scope, the memory is freed and reclaimed *unless* the value
`b` is *moved*, such as by returning it.

### Raw Pointers

Rust also provides the raw pointer types:

1. immutable raw pointer: `*const T`; and
2. mutable raw pointer: `*mut T`.

Such usage of raw pointers are inherently unsafe and dereferences must be
performed within `unsafe {}` blocks, to opt-out of the borrow-checker. It is up
to the programmer, then, to ensure memory safety and coherence within `unsafe`
blocks.

