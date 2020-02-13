# References

Rust provides **references**. which are *non-owning* pointers.

- References do not affect the lifetimes of their referents.
- To prevent the problem of *dangling references*, references cannot outlive
  their referents.
- The action of creating references is also termed **borrowing** - when the
  borrowed value is no longer used, it must be returned to its original owner.

References allow the cases where you don't actually need the ownership to a
value, but merely needs access to it.

References are *never null* – so you don't encounter the infamous
`NullPointerException` type of situations. No concept such as `nullptr` or
`NULL` exist by default and variables must be intialized before used, and
integers won't be implicitly coerced to references.

If some value needs to be either a valid reference or absent, use `Option<&T>`
to encode the possibility of absent with `None` as the "null" pointer.

## Kinds of References

Two kinds of references exist in Rust:

1. **Shared reference**.

	- A *shared reference* in Rust allows the user to read the referent but
	  *not* modify its referent (i.e. *immutable reference*). In each scope (
	  and by transitivity nested scopes contained within that scope), there may
	  be many shared references to some value due to immutability.

	- A *shared reference* is taken by the ampersand operator `&e` on some
	  expression `e`. Should the expression `e` be of type `T`, i.e. `e: T`, 
	  then its shared reference will have type `&T`.

	- Shared references automatically are `Copy`, allowing the programmer to
	  easily create copies of the shared references which will point to the same
	  target value.

2. **Mutable reference**.

	- A *mutable reference* allows both read and modify access to its referent.
	  To ensure that no data races occur, a *mutable reference* must be mutually
	  exclusive to any other references to the referent.
	
	- A *mutable reference* is taken by the expression `&mut e` for some
	  expression `e`, and if `e: T`, then the mutable reference has type
	  `&mut T`.
	
	- To enforce on the mutual exclusitivity, mutable references do *not*
	  implement the `Copy` trait.

The mutual exclusitivity between either many *shared references* or a single
*mutable reference* is the attempt to capture and enforce the rule of *multiple
readers or single writer, but not both* via the type system, checked at compile
time.

The mutual exclusitivity rule is transitively enforced to the borrowed value's
owner.

- If *shared references* exist for some value, then the owner of the value
 cannot modify it as well to also prevent data races.
- If a *mutable reference* exists for some value, then the owner cannot be used
  until the mutable reference is dropped.

The core idea is to quarantine reading from writing in separate isolated turns
to ensure memory safety to prevent data races.

## Terminology

When an argument (value) is passed to a function via ownership moving or by
copying, then this is **pass-by-value**.

If a reference to the value is passed as the argument to a function, then this
is **pass-by-reference**.

## References Are Values

Rust's references are similar to C++ references in that they are simply
addresses at the underlying implementation level.

To dereference a reference `r`, use the asterisk operator `*r`.

To reduce boilerplate for derferencing referencies, Rust's dot operator `.`
implicitly dereferences the left-hand side operand if the operand is a
reference. This behaviour is similar to C's arrow operator `a->b` where `a` is a
pointer.

```rust
struct A { x: bool };
let e = A { x: true };
let e_ref = &e;
println!("e.x = {}", e_ref.x);
println!("e.x = {}", (*e_ref).x); // equivalent in semantics
```

The dot `.` operator can also implicitly borrow a reference to its
left-hand-side operand for a method call if needed.

```rust
let mut v = vec![1, 2, 3];
v.sort();
(&mut v).sort(); // equivalent in semantics
```

### Assigning to References

```rust
let x = 1;
let y = 2;

let mut r = &x;
r = &y;
```

### References to References

Legal, but adds an additional layer of indrection.

```rust
let x: i32 = 1;
let r: &i32 = &x;
let rr: &&i32 = &r;
let rrr: &&&i32 = &rr;
```

### Comparing References

> Reference: [std::cmp:Eq](https://doc.rust-lang.org/std/cmp/trait.Eq.html)

> Reference: [std::cmp::PartialOrd](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)

Rust's comparison operators, by default, always tries to implicitly dereference
references on both sides to compare the original values they point to, for any
number of layers of references.

```rust
let x = 1;
let y = 1;

let rx = &x;
let ry = &y;

let rrx = &rx;
let rry = &ry;

assert!(rrx == rry);
assert!(rx == ry);
assert!(x == y);
```

> Reference: [std::ptr::eq](https://doc.rust-lang.org/std/ptr/fn.eq.html)

If *pointer equality* or *address equality* is desired, then
`std::ptr::eq(p1, p2)` should be used instead.

## Special References

Previous references are single-byte simple addresses.

Rust also has two types of *fat pointers* (*fat references*) which include
additional metadata information for their referents. 

1. **Slice reference**. A slice reference is a two-byte fat pointer:

	| Byte index  | Purpose          |
	| ----------- | ---------------- |
	| 0           | Starting address |
	| 1           | Length of slice  |

2. **Trait object**. A trait object is a reference to a value which implements
   some trait:

	| Byte index | Purpose                               |
	| ---------- | ------------------------------------- |
	| 0          | Address                               |
	| 1          | Trait's implementation for invocation |

## Reference Safety

### Local Variable Borrows

Scope rules are enforced to ensure memory safety.

```rust
{
	let outer;
	{
		let inner = 1;
		outer = &inner;
	} // `inner` dropped, outer deinitialized
	println!(*outer); // ILLEGAL: `outer` is not initialized due to scoping
}
```

Dereferencing `outer` would be dereferencing a dangling pointer because `inner`
is dropped at the end of the inner scope, and so Rust prohibits such construct.

This is the **lifetime** system which Rust uses to check memory safety. Each
reference has a *lifetime* which is constrained to restrict what usages are
deemed safe.

A *lifetime* is some span of the program which a reference is *safe* to use; it
may be a lexical scope (e.g. within the current pair of braces), a statement, an
expression, or tied to the scope of some variable, etc.

Lifetimes are only compile-time concepts and do not have runtime
representations so it does not have overhead.

Such lifetime rules are enforced recursively. For example, if the programmer has
a reference to a slice containing references, e.g. `[&T]`, then the slice's
lifetime must outlive all of its elements' lifetime. The same goes for enums,
structs, tuples and the likes.

### Reference Parameters

Rust's concept of global variable is a `static` variable, which has equivalent
lifetime as the running program itself (global in terms of lifetime, but not in
terms of visibility).

- Static variables *must* be initialized.
- Mutable static variables are *not* thread safe, and also face reentrancy
  issues even in single-threaded code. A mutable static can only be used within
  an `unsafe` block.

```rust
static mut GLOBAL_VAR: &i32; // ILLEGAL: not initialized
fn f(p: &i32) { GLOBAL_VAR = p; }
```

```rust
static mut GLOBAL_VAR: &i32 = &128;
fn f(p: &i32) { // ILLEGAL: still have lifetime issues
	unsafe {
		GLOBAL_VAR = p; // GLOBAL_VAR outlives p!
	}
}
```

The code above omits *lifetime parameters* because Rust can perform *lifetime
ellision* in some cases. It can also be explicitly written out:

```rust
static mut GLOBAL_VAR: &i32 = &128;
fn f<'a>(p: &'a i32) {
	unsafe {
		GLOBAL_VAR = p; // GLOBAL_VAR outlives p!
	}
}
```

Here the *lifetime variable* `'a` is taken as the **lifetime parameter** of `f`.
As the reference `p` also has lifetime `'a`. Because `'a` is required to be of
any lifetime, the problem is with the assignment from `p` to `GLOBAL_VAR`.

Because `GLOBAL_VAR`'s lifetime is that of the program, `p` may not live
sufficiently long enough and so this assignment can't be deemed as safe.

```rust
static mut GLOBAL_VAR: &i32 = &128;

fn f(p: &'static i32) {
	unsafe {
		GLOBAL_VAR = p;
	}
}
```

This is safe because now `p` has the same lifetime as `GLOBAL_VAR`, both being
`'static`.

### Struct Member Reference Lifetime

If some `struct` contains reference as members, then safety for those references
are enforced as well.

If a *reference type* is used inside another type's definition, then the
reference type's *lifetime* must be explicitly specified:

```rust
struct S {
	value: &'static i32,
}
```

The `'static` lifetime here limits what `value` can refer to significantly. If
we want to be more flexible with `value`'s lifetime, we can specify `S` to take
a *lifetime parameter*.

```rust
struct S<'a> {
	value: &'a i32,
}
```

This is to say that `S` has a *generic lifetime parameter*. `S` itself has a
lifetime.

Whenever a new `S` value is created, its lifetime is constrained by its own
lifetime as well as its generic lifetime parameter `'a`. Then, the lifetime
parameter `'a` is required to outlive that of the container struct `S`.

If there is another `struct T` which takes an `S` type as member, its lifetime
parameter cannot be elided. Either an explicit lifetime must be provided,
like `'static`, or `T` must also take a lifetime parameter.

```rust
struct T {
	s: S,  // illegal
}
```

```rust
struct T {
	s: S<'static>,
}
```

```rust
struct T<'a> {
	s: <'a>,
}
```

These lifetime parameters, when they cannot be elided, also serve as valuable
documentation.


For example, a function such as:

```rust
fn parse_record<'i>(input: &'i [u8]) -> Record<'i> { ... } 
```

Indicates that if a `Record` is returned from the function, then it's member
reference(s) must be pointing to some part or element of the input slice due to
the lifetime parameter being the same.

## Distinct Lifetime Parameters

If there is some struct such as

```rust
struct S<'a> {
	x: &'a i32,
	y: &'a i32,
}
```

But this is problematic if you wish to try to reference to two variables from an
inner and outer scope, because their lifetimes do not unify to the single `'a`.

Instead, two lifetime variables could be specified:

```rust
struct S<'a, 'b> {
	x: &'a i32,
	y: &'b i32,
}
```

Then, they *could* have different lifetimes, but they also could be the same,
which is more flexible.

The same applies to function signatures, although lifetime parameters could
potentially complicate the function signature.

```rust
fn take_first<'a, 'b>(p1: &'a i32, p2: &'b i32) -> &'a i32 { p1 }
```

### Lifetime Parameter Elision

The Rust compiler performs lifetime parameter elision for most of the trivial
cases, so the programmer does not need to always explicitly spell out the
lifetimes.

