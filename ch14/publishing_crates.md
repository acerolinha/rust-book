# Publishing a Crate to Crates.io

## Making Useful Documentation Comments

- Documentation comments can be made with `///` instead of `//`. 
- Supports Markdown syntax.

Example:

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

To generate and open HTML documentation for a crate, run:

```bash
$ cargo doc --open
```
- Common sections in documentation comments:
    - `Examples`
    - `Panics`
    - `Errors`
    - `Safety`

The `cargo test` command will run any code examples in the documentation comments.

- To document contained items, use `//!` instead of `///`.

```rust
//! # My Crate
//! 
//! `my_crate` is a collection of utiltiies to make performing
//! certain calculations more convenient.
```

- Useful for describing crates and modules
- Explain the overall purpose of the container

## Exporting a Convenient Public API with `pub use`

The `pub use` keyword allows re-exporting items from one module to another.

```rust
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;
```

- Useful to expose nested modules
- Doesn't affect the internal organization

## Setting Up a Crates.io Account

- Create an account on [crates.io](https://crates.io/)
- Generate token on [crates.io/me](https://crates.io/me)
- Add token to local machine with `cargo login <token>`

## Adding Metadata to a New Crate

- Metadata is added to the `[package]` section of the `Cargo.toml` file
- Names must be unique on crates.io
- Required fields are: `name`, `description`, `license` and `version`

Example:
```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the
computer has chosen."
license = "MIT OR Apache-2.0"
```

## Publishing to Crates.io

Use the `cargo publish` command to publish a crate to crates.io.

- Publishing is permanent

To publish a new version, update the `version` field in `Cargo.toml` and run `cargo publish` again.

To deprecate a version, use the `cargo yank --vers <version>` command.

Example:
```bash
$ cargo yank --vers 1.0.1
```

To undo a yank, use the `cargo yank --vers <version> --undo` command.

Example:
```bash
$ cargo yank --vers 1.0.1 --undo
```