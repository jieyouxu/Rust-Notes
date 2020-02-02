# Example: Handling Command Line Arguments

> Reference: Programming Rust

```rust
use std::io::Write;
use std::str::FromStr;
#
#fn gcd(mut m: u64, mut n: u64) -> u64 {
#	assert!(m != 0 && n != 0);
#	while m != 0 {
#		if m < n {
#			let t = m;
#			m = n;
#			n = t;
#		}
#		m %= n;
#	}
#	n
#}

fn main() {
	let mut numbers = Vec::new();

	for arg in std::env::args().skip(1) {
		numbers.push(
			u64::from_str(&arg)
				.expect("failed to parse argument")
		);
	}

	if numbers.len() == 0 {
		writeln!(std::io::stderr(), "Usage: gcd <INTEGER>...").unwrap();
		std::process::exit(1);
	}

	let mut d = numbers[0];
	for m in &numbers[1..] {
		d = gcd(d, *m);
	}

	println!("gcd of {:?} is {}", numbers, d);
}
```

*Source*: Programming Rust

Some notes for the snippet above:

- We import standard library **traits** `Write` and `FromStr` into the source
  file scope with the `use` import declaration.
- A **trait** is an interface specifying supported operations for a family of
  types for which each of the member type can implement.
- Even though the identifiers `Write` and `FromStr` are not *directly* used,
  they must be in scope so we can use their methods.

## The `Write` Trait

> Reference: [std::io::Write](https://doc.rust-lang.org/std/io/trait.Write.html)

Any type that implements the `Write` trait is provided with the `write_fmt`
method that writes some formatted text to a stream (a default implementation).

The `std::io::Stderr` type implements `Write`, and the `writeln!` macro can be
used to print error messages, which *expands* to the `write_fmt` method.

## The `FromStr` Trait

> Reference: [std::str::FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html)

Any type that implements the `FromStr` trait is required to implement a
`from_str` method which tries to convert a value from a string.

The `u64` type implements `FromStr` and we can invoke `u64::from_str` to
parse the command-line arguments as unsigned integers.

## The Entry Point (`fn main`)

Conforming with the tradition of C, binary executables are reqired to have
an entry point specified by the function named `main`.

Since we do not return any values from `main` here, we can omit the return
type declaration. The same holds for other functions which do not return
any value, analogous to functions returning `void` in C:

```c
void fn_with_no_return_value() {}
```

## Declaring and Initializing a Vector

> Reference: [std::vec::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)

```rust
let mut numbers = Vec::new();
```

To hold the list of numbers from the CLI arguments, we declare a local
mutable variable `numbers` initialized to an empty vector. The `Vec` type
is a growable list, supporting `push()` and `pop()` operations to the end
of the vector.

Since Rust can infer that the vector only contains `u64` values, we do not
need to provide type annotations for numbers.

- Type inference has enough information to infer that `numbers` only accept
  `u64` values because of the type of the argument passed to `push()` as well
  as passing elements of the vector to `gcd` which only accepts `u64` values.

## Control Flow: the `for` Loop

```rust
for arg in std::env::args().skip(1) {
	// ...
}
```

The `for` loop is used to process the arguments by assigning `arg` to
each argument sequentially, and evaluating the body of the loop.

> Reference: [std::env::args](https://doc.rust-lang.org/std/env/fn.args.html)

The function `std::env::args()` returns an **iterator** which is an abstraction
to allow us to traverse some container, supporting access to each element
sequentially and indicates when all elements are exhausted.

- Rust is usually able to unroll iterators into loops (and possibly perform
  loop-unrolling and other optimizations) to reduce the iterator overhead
  of function calls and to achieve better cache locality to speed up the code.

Iterators in Rust usually also provide helpful convenience methods that can
be used to simplify code, e.g. `skip()`. Since the first argument to the
CLI process is always its name, we can call `skip(1)` to skip over the
first element by producing a new iterator to omit the first value.

- This again can be optimized away into loops.

## Vector Operations

In the `for` loop, we wrote

```rust
numbers.push(u64::from_str(&arg).expect("failed to parse argument"));
```

A number of things are happening here, and let's examine the statement from
inside-out:

- We are firstly taking an immutable reference to an argument `arg` with the
  ampersand operator `&`, `&arg`.
- We pass this reference to the `from_str` method implemented on the `u64`
  type.
- The `u64::from_str` method tries to parse the string into an `u64`, and
  returns a `Result<T, E>` type which is an enumeration (`enum`) of two
  possibilities:
  	+ A *successful computation*, denoted by `Ok(T)`; or
  	+ A *failed computation*, denoted by `Err(E)`.

  	> Reference: [std::Result](https://doc.rust-lang.org/std/result/)

  	```rust
  	enum Result<T, E> {
		Ok(T),
		Err(E),
	}
  	```

  	In the context of `u64::from_str`, `Ok(u64)` indicates a successful parse,
  	and `Err(e)` indicates a failed parse with `e` being the value explaining
  	why the parse failed.

  	*Note: this is analogous to Haskell's `Either` typeclass which also encodes
  	such binary possibilities of success or failure at the type level,
  	enforcing error-handling at compile time. Enforcing error-handling at the
  	type-level means if errors are not handled, they fail early at compile
  	time.*

  	+ Functions which perform I/O and/or interact with the operating system
  	  (OS) all return `std::Result` types with `Ok` carrying the successful
  	  value or some type `T`, and the `Err` variant with an error code from
  	  the ssytem.
  	+ To conform with the zero-cost abstraction principle, Rust does *not* have
  	  exceptions (at least in the Rust 2018 edition). Error handling is
  	  typically performed via `Result` types or via panics.
- We ensure that the parse is successful using the `expect` method defined
  on the `Result` type. Should the parse fail, `expect` panics with the
  error message extracted from the `Err(e)`. A successful parse simply
  evaluates to a `u64` value.
- We `push` the `u64` value to the `numbers` vector.
- If we don't receive any numbers as arguments, i.e. if the check
  `numbers.len() == 0` fails, we print an error message to the standard error
  output stream via `std::io:stderr()` and invoking the `writeln!` macro.
  The `unwrap` method checks that we successfully printed the error message
  to `stderr`. We then abort the process with an non-zero exit code `1`
  which by convention indicate abnormal exit.
- Then, we use `d` to track the GCD for the numbers we already processed.
- Note the for loop

  ```rust
  for m in &numbers[1..] {
  	d = gcd(d, *m);
  }
  ```

  	+ Here we are trying to iterate over a `Vec` which could be of arbitary
  	  size which could potentially be very large.
  	+ Rust delegates the responsibility of memory management to the programmer,
  	  but helpds to check memory correctness.
  	+ When we iterate, we don't need to take **ownership** of the vector but
  	  instead we only *borrow* its elements so we can read their values.
  	  The `&` operator for `&numbers[1..]` borrows a *reference* to the
  	  vectors elements for the second element to the last element, which
  	  is known a **slice** (`numbers[1..]`), a readonly *view* into the vector
  	  collection.
  	+ The `m` borrows each element in `numbers` sequentially, and so `m`
  	  is a reference of type `&u64`.
  	+ We derefence `m` via the dereferencing operator `*m` so we can read its
  	  value.
  	+ Because the ownership of the vector and its elements belong to the local
  	  variable `numbers`, Rust knows it can safely free the vector and its
  	  contents when the name `numbers` go out of scope and the end of `main`.

Note that Rust assumes successful execution if `main` finishes executing,
and we need to explicitly invoke methods such as `expect` or
`std::process:exit` to terminate the process with non-zero error status codes,
unlike C/C++.
