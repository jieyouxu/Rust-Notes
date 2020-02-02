# Preliminaries

> Reference: Programming Rust

Rust is a systems programming language with type safety and efficiency in mind.

## Infrastructure, Toolchain and Utilities

The `rustup` installer helps to manage the system/user Rust version.

- [Rust installer: rustup.rs](https://rustup.rs).
- [Rust language home: rust-lang.org](https://www.rust-lang.org).

`rustup` is similar to `rvm` for ruby and `nvm` for node.js to enable easier
upgrades to the system/user Rust version, with

```bash
$ rustup update
```

Upon installation, three binary executables should be available in your
`$PATH`:

1. Build tool / package manager `cargo`:

	```bash
	$ cargo --version
	```

2. Rust compiler `rustc`:

	```bash
	$ rustc --version
	```

3. Rust documentation generator `rustdoc`:

	```bash
	$ rustdoc --version
	```

### Cargo

Cargo is the build tool and package manager for Rust.

Packages are called "*crates*" and can be found on
[crates.io](https://crates.io/), Rust's package registry, like that of NPM.

Documentation available at [The Cargo Book](https://doc.rust-lang.org/cargo/index.html).

#### $PATH Setup

If you wish to install packages for global use via `cargo install`, you may
need to add the following to your `$PATH`:

```bash
export $PATH="$PATH:$HOME/cargo/.bin"
```

Which contains the binaries of downloaded packages.

#### Generating New Package

```bash
cargo new --bin hello
```

`cargo new` can generate either:

1. a binary executable with the `--bin` flag; or
2. a library package with the `--lib` flag.

The package root has a `Cargo.toml` which holds the metadata build and
dependency information for the package, similar to `package.json` for `npm`.

#### Configuration File (Cargo.toml)

Each `Cargo.toml` metadata file contains something like:

```toml
[package]
name = "hello"
version = "0.1.0"
authors = ["Name <no-reply@example.com>"]

[dependencies]
some_package_name = "0.1.0"
```

Usually packages follow the [semver 2.0.0](https://semver.org/) versioning
scheme to track the level of changes in version variations.

#### Compiling the Package

```bash
$ cargo build [OPTIONS]
```

Reference:

- [cargo-build](https://doc.rust-lang.org/cargo/commands/cargo-build.html)

#### Building and Running the Executable (cargo run)

With the `--bin` flag supplied, Cargo can request `rustc` to compile your code
and run the built executable via

```bash
$ cargo run
```

The package will be built and placed on the `target/` directory at the root
folder of the package.

Reference:

- [cargo-run](https://doc.rust-lang.org/cargo/commands/cargo-run.html)

#### Running Tests for the Package (cargo test)

```bash
$ cargo test
```

#### Cleaning Build and Generated Files (cargo clean)

Useful for deleting built files and temporaries:

```bash
cargo clean
```

Reference:

- [cargo-clean](https://doc.rust-lang.org/cargo/commands/cargo-clean.html)

### Rust Documentation

Rust has extensive documentation available online at [docs.rust-lang.org](https://doc.rust-lang.org).

You can view a local copy if you installed Rust via `rustup`:

```bash
rustup doc --std
```
