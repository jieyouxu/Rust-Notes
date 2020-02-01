# Concurrency

Rust's type system and the **borrow checker** (`borrowck`) prevents code which
contains memory errors from compiling (unless you use `unsafe`).

> Reference: [nikomatsakis](https://www.slideshare.net/nikomatsakis/rust-concurrency-tutorial-2015-1202)

> Great talk by Niko Matsakis: [C++Now 2017: Niko Matsakis "Rust: Hack Without Fear!"](https://www.youtube.com/watch?v=lO1z-7cuRYI)

> Blog post: [Fearless Concurrency](https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html)

- This is the **ownership/borrowing** system + traits:
	+ Does not need runtime (compared to C++).
	+ Guaranteed memory safety (no need for garbage collection).
	+ No data races.

## Principled Aliasing and Mutations

With the ownership and borrowing system in Rust, by utilizing suitable scoping
rules (**lifetime**s) and the type system, we can have principled aliasing and
mutations for avoiding problems such as data races and liveness issues.

The key is to have *mutually exclusive* **aliasing** versus **mutations** which
are enforced by Rust's type system.

| Type     | Ownership         | Alias Allowed | Mutation Allowed |
|----------|-------------------|---------------|------------------|
| `T`      | Owned             | ❌             | ✅                |
| `&T`     | Shared reference  | ✅             | ❌                |
| `&mut T` | Mutable reference | ❌             | ✅                |

## Concurrency Paradigms

Rust does not have "default" concurrency paradigm, instead it supports all of
them:

| Concurrency Paradigm  | Addressed by Ownership | Addressed by Borrowing | Notable Usages       |
|-----------------------|------------------------|------------------------|----------------------|
| Fork-join             |                        | ✅                 | C                    |
| Message passing       |                        | ✅                 | Erlang/Elixir        |
| Locking               | ✅                 | ✅                 | C, Java              |
| Lock-free             | ✅                 | ✅                 | C, Java              |
| Futures (Async/Await) | ✅                 | ✅                 | JavaScript (node.js) |

> Great talk by Jon Gjengset on [Noria: Lock-free concurrent database](https://www.youtube.com/watch?v=s19G6n0UjsM&t=1118s)
>
> Lock-free, eventually consistent, concurrent multi-value map: [jonhoo/rust-evmap](https://github.com/jonhoo/rust-evmap)
