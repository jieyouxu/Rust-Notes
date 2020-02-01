# Example: Simple Web Server

We can create a simple web server to serve a static HTML page as the front
end to our `gcd` function.

## Initialize a Executable Package

```bash
$ cargo new --bin gcd_web_server_v1
$ cd gcd_web_server_v1
```

## Editing Cargo.toml

> Reference: [Cargo.toml Manifest](https://doc.rust-lang.org/cargo/reference/manifest.html)

```toml
{{#include ../../../examples/gcd_web_server_v1/Cargo.toml}}
```

Note that the `[dependencies]` section in the `Cargo.toml` file specifies the
names and versions of crates published on [crates.io](https://crates.io).

## Version 1

```rust
{{#include ../../../examples/gcd_web_server_v1/src/main.rs::93}}
```

> Reference: [iron 0.6.1](http://ironframework.io/)

- We first import the required crates, from the standard library as well as
  the `iron` framework.
- We construct an HTTP server via `Iron::new()` and serve it on the specified
  `SocketAddr`.
- We register the handler `get_form` for the route `/`; a route handler takes
  a `Request` and constructs a suitable `Response`.
- For the page content, we use a *raw string literal* denoted via
  `r#"<CONTENT>"#`:

	```rust
	r#"
		double quotes here "" need not be escaped
	"#
	```
- We use `match` expressions to handle possible error cases and return
  suitable HTTP error status codes and messages early.

  A match expression matching against a `Result` enum takes the form of
  ```rust
  match expression {
 	Ok(value) => {}
 	Err(e) => {}
  }
  ```

  The Rust compiler will force the programmer to *exhaustively* match against
  the `expression` to help your catch mistakes.

- Note the `request.get_ref::<UrlEncodedBody>()` expression, which tries to
  parse the `request`'s body as url-encoded format for forms.

  Here, `::<UrlEncodedBody>` is the `type parameter` for `get_ref` which
  specifies that `get_ref` should retreive the body as `UrlEncodedBody` type.
