# Ownership and Lifetimes

> Reference: [Rustnomicon | Ownership and Lifetimes](https://doc.rust-lang.org/nomicon/ownership.html)

As a systems programming languages, Rust gives the control for memory management
to the programming.

- The **lifetimes** (that is, the duration for which the memory pointed to by
  the reference is valid) of the variables is determined by the programmer.
  Through carefully designed **ownership**, **borrowing** and **scoping rules**,
  Rust can automatically free memory and other resources when references to such
  resources are no longer valid. This is like the concept of *resource
  acquisition is initialization* (RAII) in C++.

- Through the same **ownership**, **borrowing** and **scoping rules**, Rust can
  check that no memory bugs such as *dangling pointers*, *double free* and data
  races can occur at compile-time (for *safe* Rust).

## Memory Management

Memory management is handled differently by different programming languages.

- C/C++ satisfies memory management control by giving all control to the
  programmer. However, safetyness of such memory operations are not guaranteed
  and mistakes such as *double free*, *dangling pointers* and the likes are easy
  to produce and difficult to get correct.

- To prevent such memory bugs, some programming languages like Java use Garbage
  Collection (GC) to automatically allocate and free memory. The trade-off is
  then the programmer has no explicit control over memory, and GC has overheads
  as well and might not be suitable for very resource-intensive embedded
  systems.

The experiences from these languages reveal a potential trade-off between
control over memory *versus* memory safetyness. But Rust uses its **ownership**,
**borrowing** and **scoping rules** to maximize the utility in both aspects.

Rust does this by carefully imposing *restrictions* over which programs are
accepted as "correct" by the compiler.

## An Opt-in-By-Default, Opt-out-By-Exception Safety Check

When such safety features are not sufficient in expressiveness, Rust also allows
the programming to explicitly opt-out of the borrow-checker by annotating unsafe
Rust code with `unsafe {}` blocks. In this case, the responsibility of checking
memory safety and coherence is delegated to the programmer. The idea is that
even if a small part of the program must be `unsafe`, we can design a *safety
wrapper* around the `unsafe` block and only a small portion of our program needs
to be checked for the culprits of segfaults and the likes.

## Ownership

Every value in Rust has a **single owner** which determines its **lifetime**,
the duration for which the memory region which is allocated to hold the value is
valid. When this owner is **dropped** (i.e. *freed*), the owned value is also
**dropped**.

- A variable *owns* its value.
- When the variable goes out of scope, so does its value (i.e. the value is
  dropped when the value goes out of scope).

```rust
fn lifetime_demo() {
	let mut a = vec![1,2,3]; 		// `a` allocated here
	for i in 0..a.len() { 			// `i` allocated here
		a.push(i * 2);
	} 								// `i` freed here
	println!("{:?}", a);
} 									// `a` out of scope, memory for `a`'s value freed here
```

Here, `a` is the fat pointer pointing to the `Vec<i32>`. The pointer `a` itself
is allocated on the stack frame, whereas the backing vector buffer is allocated
on the heap.

- When `a` goes out of scope, Rust can see that the buffer's owner is out of
  scope, so it can safely free the buffer `Vec<i32>` with the same lifetime.

A `Box<T>` is a pointer to some value of type `T` stored on the heap. We can
allocate some space on the heap by calling `Box::new(value)` which returns a
pointer to the heap. Since the `Box` owns the allocated heap memory, when the
`Box` is dropped the heap memory is freed too.


```rust
{
	let point = Box::new((0, 0)); 		// `point` allocated here
	let ps = format!("{:?}", point); 	// `ps` allocated here
	println!("Point = {}", ps);
} 										// both dropped here
```

Notice that `point` has a longer lifetime than `ps`, in which we say that 
`point` *outlives* `ps`.

Like variables, `struct`s, `tuple`s, arrays, vectors own their elements.

### Ownership Tree

A composite type, like a `struct`, owns its data type, which in turn may be
other complex data types which also own their data types.

- *Owners* and the values which they *own* form an **ownership tree**.
	+ The *parent* is the *owner*.
	+ The *children* is the values *owned* by the *parent*.
	+ When the *root* is gone, the entire tree can be freed from the leaves to
	  the nodes then eventually to the root.

```
                             root
               +------------+    +-------------+
               |                               |
               v                               v
            node_1                           node_2
    +-------+    +--------+          +-------+  +  +--------+
    |                     |          |          |           |
    v                     v          v          v           v
leaf_1_1              leaf_1_2    leaf_2_1    node_2_1     leaf_2_2
                                                +
                                                |
                                                v
                                            leaf_2_1_1
```

In this example, when `root` goes out of scope, then Rust will need to
recursively free bottom-up. The leaf at the deepest level (`leaf_2_1_1`) is
dropped first, then Rust can drop the leaves at the third layer (`leaf_1_1`,
`leaf_1_2`, `leaf_2_1`, `leaf_2_2`). Rust can also check that `node_2_1` has no
children, so the node itself can also be freed. Similarily, the nodes at the
second level can then be freed (`node_1`, `node_2`), followed by eventually the
`root` node.

To ensure memory safety, Rust's borrow-checker prevents the programmer from
expressing an ownership tree with cycles. That is, safe Rust accepts only
**acyclic ownership trees** so there cannot be cyclic ownership. This is less
expressive than other languages which allow arbitrary graphics containing
cycles, *but* this limitation is precisely what grants safe Rust memory safety.

Notice that in this model, it is difficult to express some data structures such
as a doubly linked list due to the restriction on cyclic ownership in safe
Rust. This is the case for the careful use of `unsafe` Rust.

For more flexibility, Rust also incorporates the concepts of:

- **Moves**: it is possible to move values from one owner to another owner,
  allowing the programmer to build, rearrange and teardown the ownership tree.
- **Reference-counted Pointers**:
	+ Rust's standard library provides *reference-counted* pointer type `Rc`
	  and *atomically reference-counted* pointer type `Arc` which supports
	  multiple owners, but with their own restrictions for safety.
- **Borrowing**: the programmer can *borrow* a reference to an owned value;
  *references* are non-owning pointers with limited lifetimes.


