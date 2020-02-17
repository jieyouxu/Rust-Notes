# Remark: `PartialEq` vs `Eq`

> Reference: [std::cmp::PartialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)

> Reference: [std::comp:Eq](https://doc.rust-lang.org/std/cmp/trait.Eq.html)

These two traits are different; namely, `Eq` is a stronger equivalence
relation compared to `PartialEq` because:

- `PartialEq` requires:

  + **Symmetry**:

    \\[
      \forall a, b \colon a == b \to b == a
    \\]

  + **Transitivity**:

    \\[
      \forall a, b, c \colon a == b \land b == c \to a == c
    \\]

- `Eq` requires *reflexitivity* in addition to *symmetry* and *transitivity*:

  + **Reflexitivity**:

    \\[
      \forall a \colon a == a    
    \\]

There are specific types and values for which *reflexitivity* may not be
satisfied. A good example of this is floating-point numbers, `f32` and
`f64`, where the special value `NaN` is specifically defined in IEEE-754 to
be \\( \mathtt{NaN} \mathbin{\texttt{!=}} \mathtt{NaN} \equiv \mathtt{true} \\).

And so, the `std::ops::Range` cannot be used with floating-point numbers
because `f32` and `f64` are not `Eq` – which makes sense, given the
existence of `NaN`!
