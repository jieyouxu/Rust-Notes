# Shared Ownership

> Reference: [std::rc::Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html)

> Reference: [std::sync::Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)

The single-owner restriction by default helps to prevent some nasty memory bugs
– but it is also a trade-off in expressiveness. Sometimes it is difficult to
find a single owner which has the suitable lifetime, and the programmer may want
a value to live until there are no more references to it. Rust provides:

- **Reference-Counted Pointer** `Rc`.
- **Atomically Reference-Counted Pointer** `Arc` (thread-safe).

Rust's type system and the borrow checker helps to enforce safe usage of these
shared references types.

- `Rc` is not thread-safe, and so has less overhead compared to `Arc` when there
  is no need for multi-threading.

## Reference-Counted Pointer `Rc`

```rust
use std::rc::Rc;

let a: Rc<String> = Rc::new("hello".to_string());
let b = a.clone();
let c = a.clone();
```

The type `Rc<T>` for any type `T` is a pointer to the heap-allocated `T` type
which has a reference count context associated with it.

- Cloning the `Rc<T>` does *not* copy the `T` value itself but instead creates a
  new pointer to it and increments the reference count.
- Each of `a`, `b` `c` `Rc<String>` pointers refer to the same heap memory
  containing a reference count and suitable space for holding the `String`.
- Whenever a `Rc<String>` pointer goes out of scope, then the reference count
  decrements by one.
- When all of the `Rc<String>` pointers go out of scope, and the reference count
  becomes zero, then it is safe to drop the `String` as well.

Values owned by `Rc<T>` pointers are *immutable*:

- Rust presents values from being simultaneously shared *and* mutable, which is
  a recipe for disaster.

### A Cyclic Dependency

Trying to manage memory with reference counts come with its own chanllenges:

- If there exists two reference-counted pointers pointing to each other, they
  will lock-up each other's reference count \( > 0 \) and so the values will
  never be freed.

