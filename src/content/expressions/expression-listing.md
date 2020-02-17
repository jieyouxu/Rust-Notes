# Expression Listing

| Expression             | Example                  | Related traits                                                                                                                                                                                                  |
|------------------------|--------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Array literal          | `[1, 2]`                 |                                                                                                                                                                                                                 |
| Repeated array literal | `[0; 50]`                |                                                                                                                                                                                                                 |
| Tuple                  | `(1, 'a')`               |                                                                                                                                                                                                                 |
| Struct literal         | `Point {x: 0, y: 0} `    |                                                                                                                                                                                                                 |
| Tuple field access     | `x.0`                    | [std::ops::Deref](https://doc.rust-lang.org/std/ops/trait.Deref.html), [std::ops::DerefMut](https://doc.rust-lang.org/std/ops/trait.DerefMut.html)                                                              |
| Struct field access    | `struct_name.field_name` | `Deref`, `DerefMut`                                                                                                                                                                                             |
| Method invocation      | `target.run()`           | `Deref`, `DerefMut`                                                                                                                                                                                             |
| Function invocation    | `stdin()`                | [std::ops::Fn](https://doc.rust-lang.org/std/ops/trait.Fn.html), [std::ops::FnMut](https://doc.rust-lang.org/std/ops/trait.FnMut.html), [std::ops::FnOnce](https://doc.rust-lang.org/std/ops/trait.FnOnce.html) |
| Indexed access         | `x[i]`                   | [std::ops::Index](https://doc.rust-lang.org/std/ops/trait.Index.html), [std::ops::IndexMut](https://doc.rust-lang.org/std/ops/trait.IndexMut.html), `Deref`, `DerefMut`                                         |
| Try                    | `File::open("src")?`     | [std::ops::Try](https://doc.rust-lang.org/std/ops/trait.Try.html)                                                                                                                                               |


| Expression | Example                                  | Related traits                                                                                                                                                       |
|------------|------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Grouping   | `(1 + 1)`                                |                                                                                                                                                                      |
| Block      | `{ a; b }`                               |                                                                                                                                                                      |
| If         | `if expr { body }`                       |                                                                                                                                                                      |
| If-else    | `if expr { body_1 } else { body_2 }`     |                                                                                                                                                                      |
| If-let     | `if let Some(v) = expr { v } else {0}`   |                                                                                                                                                                      |
| Match      | `match expr { Some(v) => v, None => 0 }` |                                                                                                                                                                      |
| For        | `for val in values {}`                   | [std::iter::Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html), [std::iter::IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) |
| While      | `while expr { }`                         |                                                                                                                                                                      |
| While-let  | `while let Some(x) = iter.next() { }`    |                                                                                                                                                                      |
| Loop       | `loop {}`                                |                                                                                                                                                                      |


| Expression          | Example         | Related traits                                                                  |
|---------------------|-----------------|---------------------------------------------------------------------------------|
| Logical/bitwise NOT | `!expr`         | [std::ops::Not](https://doc.rust-lang.org/std/ops/trait.Not.html)               |
| Logical AND         | `a && b`        |                                                                                 |
| Logical OR          | `a              |                                                                                 |
| Negation            | `-num`          | [std::ops::Neg](https://doc.rust-lang.org/std/ops/trait.Neg.html)               |
| Dereference         | `*ptr`          | `Deref`, `DerefMut`                                                             |
| Borrow              | `&val`          |                                                                                 |
| Type cast           | `val as u32`    |                                                                                 |
| Multiplication      | `n * 1`         | [std::ops::Mul](https://doc.rust-lang.org/std/ops/trait.Mul.html)               |
| Division            | `n / 2`         | [std::ops::Div](https://doc.rust-lang.org/std/ops/trait.Div.html)               |
| Addition            | `n + 2`         | [std::ops::Add](https://doc.rust-lang.org/std/ops/trait.Add.html)               |
| Subtraction         | `n - 2`         | [std::ops::Sub](https://doc.rust-lang.org/std/ops/trait.Sub.html)               |
| Remainer (modulo)   | `n % 2`         | [std::ops::Rem](https://doc.rust-lang.org/std/ops/trait.Rem.html)               |
| Left shift          | `n << 2`        | [std::ops::Shl](https://doc.rust-lang.org/std/ops/trait.Shl.html)               |
| Right shift         | `n >> 2`        | [std::ops::Shr](https://doc.rust-lang.org/std/ops/trait.Shr.html)               |
| Bitwise AND         | `n & 1`         | [std::ops::BitAnd](https://doc.rust-lang.org/std/ops/trait.BitAnd.html)         |
| Bitwise OR          | `n              | 1`                                                                              |
| BItwise XOR         | `n ^ 2`         | [std::ops::BitXor](https://doc.rust-lang.org/std/ops/trait.BitXor.html)         |
| Less than           | `n < 0`         | [std::cmp::PartialOrd](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) |
| Less than equal     | `n <= 0`        | `PartialEq`                                                                     |
| Greater than        | `n > 0`         | `PartialEq`                                                                     |
| Greater than equal  | `n >= 0`        | `PartialEq`                                                                     |
| Equal               | `n == 0`        | `PartialEq`                                                                     |
| Not equal           | `n != 0`        | `PartialEq`                                                                     |
| Range               | `start .. stop` | [std::ops::Range](https://doc.rust-lang.org/std/ops/struct.Range.html)          |
| Assignment          | `a = b`         |                                                                                 |
| Compound assignment | *omitted*       | `std::ops::xxxAssign`                                                           |
| Closure             | `⎮a, b⎮ a + b`  |                                                                                 |
