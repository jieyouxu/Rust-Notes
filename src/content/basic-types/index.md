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

As of right now, Rust is *compiled*.

## List of Basic Types

| Type                                | Description                                                            | Example Value(s)                  |
| ----------------------------------  | ---------------------------------------------------------------------- | --------------------------------- |
| `i8`, `i16`, `i32`, `i64`           | Signed integers, fixed number of bits                                  | `-32`                             |
| `u8`, `u16`, `u32`, `u64`           | Unsigned integers, fixed number of bits                                | `0`                               |
| `isize`, `usize`                    | Machine-sized unsigned and signed integers                             | `16`                              |
| `f32`, `f64`                        | IEEE-754 compliant floating-point numbers                              | `1.45`, `1.4f32`                  |
| `bool`                              | Boolean                                                                | `true`, `false`                   |
| `char`                              | UTF-8 character, 32-bits                                               | `*`                               |
| `()`                                | Unit type                                                              | `()`                              |
| `(char, i32)`                       | Tuple type (mixed product type)                                        | `('b', 0)`                        |
| `struct S { x: i32, y: f32 }`       | Struct with named fields                                               | `S { x: 0, y: 1.0 }`              |
| `struct T(i32, char);`              | Tuple-like structure (gives name)                                      | `T(0, 'a')`                       |
| `struct E`                          | Unit-like struct, no fields, zero-sized                                | `E`                               |
| `enum Option<T> { Some(T), None }`  | Enum, algebraic data type (ADT)                                        | `Option::Some(2)`, `Option::None` |
| `Box<Attend>`                       | `Box` is an owning-pointer to some value allocated in heap             | `Box::new(3)`                     |
| `&T`                                | Immutable shared reference to some type `T`, non-owning                | `&name`                           |
| `&mut T`                            | Mutable reference to some type `T`, non-owning                         | `&mut name`                       |
| `String`                            | Dynamically-sized UTF-8 string, owned                                  | `"Hello World"`                   |
| `&str`                              | Non-owning reference to `str`                                          | `&s[0..12]`                       |
| `[T; N]`                            | Fixed-length array of homogeneous type `T` of size `N`                 | `[0; 256]`                        |
| `Vec<T>`                            | Dynamically-sized vector of homogeneous type `T`                       | `vec![0, 1, 2]`                   |
| `&[T]`                              | Immutable reference to a *slice*, a view into (a part of) an array       | `&v[1..3]`, `&x[..]`              |
| `&mut [T]`                          | Mutable reference to a *slice*                                           | `&mut v[1..3]`                    |
| `&Trait`                            | Immutable reference to any value which implements the trait `Trait`    | `value as &Any`                   |
| `&mut Trait`                        | Mutable reference to any value which implements the trait `Trait`      | `value as &mut Any`               |
| `fn(<param>, ...) -> R`             | Pointer to function with parameters `<param>, ...` and return type `R` | `i32::saturating_add`             |
| `&#x7c; <param>, ... &#x7c; <expr>` | Closure capturing parameters `<param, ...` with body `<expr>`          | ` &#x7c; a, b &#x7c; a + b`       |

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

## Arrays, Vectors and Slices

Rust has three basic types for representing a contiguous sequence of values in
memory:

1. **Fixed-size Array**: `[T; N]`

	Represents an an array of `N` values of type `T`. The size of an array is
	determined at compile-time and is enforced as part of the type; immutable.

2. **Vector**: `Vec<T>`

	Represents a dynamically allocated sequence of values of type `T`. Elements
	of a vector lives on the heap to allow resizing.

3. **Shared Slices**: `&[T]` and `&mut [T]`

	- *Shared slice*: `&[T]`. A reference to a sequence of elements which is
	  part of some container such as an array or vector. Can be a sub-range of
	  the container. Immutable reference.

	- *Mutable slice*: `&mut [T]`. A reference to a sequence of element of some
	  container, can be used to modify arguments but cannot be shared.

The length can be obtained for each of these container types via the `len()`
function defined for these types. Accessing an element at index `i` can be done
via `v[i]` where `v` is the name of the container. Out-of-bounds will produce a
panic, and indicies into such container types is required to be of type `usize`.

### Arrays

#### Array Literals

Array literals can be declared with the syntax

```
name: [T; N] = [<element>, ...];
```

With `T` being the type and `N` the number of elements.

```rust
let elements: [u32; 6] = [1, 2, 3, 4, 5, 6];
let strings = ["a", "b", "c"];
```

Additionally, it is possible to initialize the entire array

```rust
let elements = [true; 100];
```

### Vectors

Each vector `Vec<T>` is a resizable array allocated on the heap, containing
elements of type `T`.

The `vec!` macro may be used to create a vector, similar in syntax to arrays.

```rust
let mut v = vec![1, 2, 3];
```

Again, it is possible to repeat some value for a number of times

```rust
let mut v = vec![true; 100];
```

The `vec!` macro is a syntax sugar for creating a new vector via `Vector::new`
and pushing the desired elements on to it via `v.push()`.

It is also possible to build a vector from an iterator, such as

```rust
let v: Vec<i32> = (0..5).collect();
```

#### Initializing with Capacity

To resize the internal array representation within a `Vec<T>`, Rust will need to
create a larger new array and copy contents from the old array over. This is
both space and time costly, so the programmer should take care to specify a
suitable initial capacity whenever possible, by using the alternative
constructor `Vec::with_capacity`.

### Slices

A *slice* `[T]` (with no length specified) represents a *region* of an array or
vector, and is always passed by reference since the length is arbitrary.

A reference to some slice is a *fat pointer*, containing:

| Word | Value                                     |
| ---- | ----------------------------------------- |
| `0`  | Pointer to element `i = 0` of the slice   |
| `1`  | Number of elements contained in the slice |

Rust is able to automatically convert between `&Vec<T>` and `&[T; N]` slice
references given the sizes are compatible.

Compared to normal references, a slice reference is a *non-owning* pointer to
multiple values (technically, to the first element of the part of the container
with multiple elements).

#### Sub-slicing

A reference to a slice of an array or vector, or a sub-slice of an existing
slice can be obtained via an integer range

```rust
let v = [true; 10];
let w = &v[5..9];
```

Given some source slice named `v`, then the following table lists the syntax for
getting sub-slices

| Reference syntax | Meaning                                                                            |
| ---------------- | ---------------------------------------------------------------------------------- |
| `&v[A..B]`       | A sub-slice starting from index `A` (inclusive) and ends at index `B` (exclusive). |
| `&v[A..]`        | A sub-slice starting at index `A`.                                                 |
| `&v[..B]`        | A sub-slice ending at index `B`.                                                   |

Note that indices are checked for out-of-bounds and a panic will be produced
upon illegal index access to prevent issues such as illegal memory access or
leaking possibly sensitive information in memory (e.g. heartbleed).

## Strings

Rust, like C++, has two string types:

1. Immutable string: `&str`.
2. Mutable, growable string: `String`.

### String Literals

Strings are delimited with double quotes `"`, and share backslash sequences with
`char` literals (with the difference that single quotes do not need backslashes
but double quotes do need escaping).

A string literal may span multiple lines. The newline character is included
provided that no backslash `\` ends the line.

```rust
let str_1 = "hello
	world";
let str_2 = "no \
	newline"
```

### Raw String Literals

For convenience, Rust allows *raw string literals* by prepending the modifier
`r` before the string literal.

```rust
let raw_string = r"C:\system32\folder";
let regex_pattern = Regex::new(r"\d+");
```

If double quotes need to be contained within, a longer version for raw string
literal is supported to remedy the problem of no escape characters being
recognized:

```rust
let raw_string_v2 = r###"
	println!("hello world!");	
"###;
```

### Byte Strings

For efficiency reasons, Rust also supports *byte string literals*. A *byte
string* is a slice of `u8` values instead of 4-byte `char` values.

A byte string literal is created by prepending `b` before the starting double
quote `"`.

```rust
let byte_str = b"HELLO";
```

Byte strings can only contain ASCII characters because of the size limit of 1
byte (escape sequences with `\xAA` is supported still).

### String and Memory

Rust strings are stored as sequence of UTF-8 characters by default, but they are
*not* stored as array of `char`s because UTF-8 is a *variable-width* encoding.
Some characters may take up only 1 byte, while others can take up multiple
bytes.

- A `String` is an *owned* data type. It represents a resizable buffer holding
  UTF-8 content, and is allocated on the heap. Under the hood, `String` is an
  alias for `Vec<u8>` vector but implemented to check that its content is a
  valid UTF-8 character sequence.

- A `&str` string slice is a reference to some part of a UTF-8. It is borrows
  some string owned by someone else. Similar to vector or array slices, `&str`
  is a fat pointer with both the pointer to starting character as well as length
  of string container. It is basically a `&[u8]` slice that is promised to hold
  well-formed UTF-8.

- A *string literal* is preallocated and usually stored in read-only `.data`
  segment within the compiled binary.
	+ Note that one may have to explicitly annotate the string literal with
	  `'static` lifetime:

	  ```rust
	  let compile_time_constant_str: &'static str = "abc";
	  ```

### The `String` Type 

> Reference: [std::str](https://doc.rust-lang.org/std/str/index.html).

- `&str` is similar to `&[T]`;
- `String` is similar to `Vec<T>`.

| Behaviour                         | `Vec<T>`            | `String`            |
| --------------------------------- | ------------------- | ------------------- |
| Auto buffer free                  | Yes                 | Yes                 |
| Resizable                         | Yes                 | Yes                 |
| `::new()` and `::with_capacity()` | Yes                 | Yes                 |
| `.reserve()` and `.capacity()`    | Yes                 | Yes                 |
| `.push()` and `.pop()`            | Yes                 | Yes                 |
| Range syntax `v[start..end]`      | Yes (`-> &[T]`)     | Yes (`-> &str`)     |
| Auto conversion                   | `&Vec<T>` to `&[T]` | `&String` to `&str` |
| Inherites methods                 | From `&[T]`         | From `&str`         |

#### Creation Methods

- The `.to_string()` method converting from a string slice `&str` to a `String`.
- The `format!()` macro.
- The `.concat()` and `.join(separator)` methods to build a new `String` from
  existing strings or string slices.

#### Usage

- Equality is supported by `==` and `!=` operators.
- Case conversion:
	+ `.to_lowercase()`
	+ `.to_uppercase()`
- Substring contains check `.contains(substr)`.
- Replacing: `.replace(regex, replacement)`.
- Trimming whitespace: `.trim()`.
- Splitting by separator: `.split(pattern)`.
- Starts with: `.starts_with()`.
- Ends with: `.ends_with()`.

And more methods.

### Alternative String Types

Sometimes `String`s are not necessarily valid Unicode, usually for interoperate
reasons.

| Usage                                            | Suitable Data Type            | Documentation and Resources                                                                                     |
|--------------------------------------------------|-------------------------------|-----------------------------------------------------------------------------------------------------------------|
| Unicode text                                     | `String`, `&str`              | [std::str](https://doc.rust-lang.org/std/str/index.html)                                                        |
| Filenames                                        | `std::path::PathBuf`, `&Path` | [std::path](https://doc.rust-lang.org/std/path/index.html)                                                     |
| Binary data                                      | `Vec<u8>`, `&[u8]`            | [std::vec](https://doc.rust-lang.org/std/vec/index.html), [u8](https://doc.rust-lang.org/std/primitive.u8.html) |
| Environment variables, CLI arguments from the OS | `OsString`, `&OsStr`          | [std::ffi::OsString](https://doc.rust-lang.org/std/ffi/struct.OsString.html)                                    |
| C libraries, null-terminated strings             | `std::ffi::CString`, `&CStr`  | [std::ffi::CString](https://doc.rust-lang.org/std/ffi/struct.CString.html)                                      | 

