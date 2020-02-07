# Moves

For more flexibility, Rust also provides the ability to **move** values.

```rust
let mut s = "Hello".to_string();
s = "World".to_string(); 			// value "Hello" dropped
```

When `s` is reassigned to `"World"`, the prior value `"Hello"` is dropped.

```rust
let mut s = "Hello".to_string();
let t = s;
s = "World".to_string();
```

Note that in this example, because `t` assumes ownership of `"Hello"`, no value
is dropped since `s` becomes uninitialized at the time of reassignment of
`"World"`.

- Passing arguments to functions *moves* ownership from the caller to the called
  function's parameters.
- Returning value(s) from a function movies ownership back to the caller.

```rust
struct User {
	id: u64,
	name: String,
}

let mut users: Vec<User> = Vec::new();

users.push(
	User {
		id: 1,
		name: "LOL".to_string(),
	}
);
```

- Invoking `Vec::new()` allocates a new vector on the heap, and returns the
  ownership of the vector itself to the variable `users`.
- The `name` field of the constructed `User` is initialized with the return
  value from `to_string()`, allowing the struct to assume ownership of the
  `String`.
- The `User` struct is passed to the vector's `push` method which takes
  ownership and *moves* the struct on to the end of the vector. The vector
  becomes the owner of the new `User` and by transitivity owns the `name`
  `String`.

Such *moves* apply to the **value proper** instead of the allocated heap memory.
For contiguous memory types such as vectors and strings, the **value proper** is
a three-word header, and the allocated heap memory remains unchanged. The Rust
compiler is also able to perform some optimizations regarding moves to mitigate
its impacts.

## Moves and Control Flow

If a variable has its owned value moved away but never given a new value since,
then such value is considered *uninitialized* and will be rejected by the
compiler.

```rust
let s = "hello".to_string();
let t = s;					// "hello" now owned by `t`
println!("{}", s); 			// ERROR: `s` now uninitialized
```

## Moves and Indices

Rust may prevent the programmer from trying to move a value out from an arbitary
index out of an array because of performance constraints - it may be expensive
to track ownership changes with large vectors.

```rust
let mut v = Vec::new();
for i in 1..6 {
	v.push(i.to_string());
}
```

> Reference: [std::vec::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)

> Reference: [std::mem::replace](https://doc.rust-lang.org/std/mem/fn.replace.html)

To move values out without violating the borrow-checker, we can:

1. Pop a value off from the end of the vector:

	```rust
	let last = v.pop().unwrap();
	```

2. Move a value out of the of the array and replace with last element:

	```rust
	let mid = v.swap_remove(1);
	```

3. Replace another value in-replace for the target value we're taking out:

	```rust
	let target = std::mem::replace(&mut v[2], "swapped".to_string());
	```

Each of these methods perform moves but leaves the vector intact (each
consecutive field is properly initialized), even though the vector may become
smaller.

## Collection Types, Iterators and Moves

Collection types such as `Vec<T>` also usually offer iterators for consumption
of all elements.

```rust
let values = vec!["aaa", "bbb", "ccc"];

for mut val in values {
	val.push('!');
	println!("{}", val);
}
```

- When the vector `values` is passed to the `for` loop, the ownership of the
  vector is *moved* out of `values` making `values` uninitialized, with the
  `for` loop internally assuming ownership of the `values` vector.
- At each iteration, the `for` loop moves another element to `val`.

## Moves and `Option<T>`

If it is absolutely necessary for one to try to move a value out of an owner
which cannot be tracked by the compiler, the owner's type may be changed into
something that can represent an optional value (e.g. `Option<T>`).

```rust
struct User {
	id: u64,
	name: Option<String>,
}

let mut users = Vec::new();
users.push(User { id: 1, name: Some("hello".to_string()) });

// CANNOT do:
// let first_name = users[0].name;
// because Rust cannot "move out of indexed content".

// Bypass by substituting in a placeholder value which does not invalidate the
// ownership requirement.
let first_name = std::mem::replace(&mut users[0].name, None);
```

> Reference: [std::mem::take](https://doc.rust-lang.org/std/mem/fn.take.html)

Rust provides a convenience method for this pattern `.take()`:

```rust
let first_name = users[0].name.take();
```

Here, `std::mem::take()` simply moves the target value out and substitutes the
default value for the type `T` in-place.

