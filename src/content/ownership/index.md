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

