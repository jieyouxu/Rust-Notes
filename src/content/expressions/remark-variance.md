# Subtyping Variance

> Reference: [Subtyping and Variance](https://doc.rust-lang.org/nomicon/subtyping.html)

> Reference: [Variance in Scala](https://docs.scala-lang.org/tour/variances.html)

**Variance** is a property for *type constructors* with respect to their type
parameter. Given a type `I[T]` where `I` is a higher-kinded type constructor of
kind `* -> *`, generic over a single type parameter `T`, then *variance* is a
property of `I` with respect to `T`.

- `I` is **covariant** iff `I<Sub>` is a subtype of `I<Super>`.

- `I` is **contravariant** iff `I<Super>` is a subtype of `I<Sub>` (reversed).

- `I` is **invariant** if no subtyping exists between `I<Super>` and `I<Sub>`.

    + This is the case for Java's generics: `List<T>` is invariant over its type
      parameter `T`, causing `List<Dog>` to not be a subtype of
      `List<Animal>` even if `Dog extends Animal`.

If some type `J` is a higher-kinded type constructor with more than one type
parameter, then variance can be described with respect to one of the type
parameters individually.

Typically, *covariance* is the most common type of variance.

Rust needs to consider variance also with respect to *lifetimes*, which is in
fact also part of the type system!

| Type            | Lifetime Variance (`'a`) | `T`            | `U`       |
|-----------------|--------------------------|----------------|-----------|
| `&'a T`         | covariant                | covariant      |           |
| `&'a mut T`     | covariant                | invariant      |           |
| `Box<T>`        |                          | covariant      |           |
| `Vec<T>`        |                          | covariant      |           |
| `UnsafeCell<T>` |                          | invariant      |           |
| `Cell<T>`       |                          | invariant      |           |
| `fn(T) -> U`    |                          | contravariant* | covariant |
| `*const T`      |                          | covariant      |           |
| `*mut T`        |                          | invariant      |           |

* Note the single source of contravariance, in the parameters to a function!

