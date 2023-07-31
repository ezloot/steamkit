# Steam Kit

[![Crates.io][crates-badge]][crates-url]
[![docs.rs][docs.rs-badge]][docs.rs-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/steamkit.svg
[crates-url]: https://crates.io/crates/steamkit
[docs.rs-badge]: https://img.shields.io/docsrs/steamkit.svg
[docs.rs-url]: https://docs.rs/steamkit
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE.md

This crate is designed to be a bunch of tools for interacting with the Steam API.

# Goals

- [ ] Be async-runtime agnostic-ish by at least offering features for popular runtimes.
- [ ] Have an easy to use API but still flexible.
- [ ] Maybe support `no-std`? ***Probably not happening.***

# To-do

- [x] Add `serde` feature to `steamkit-vdf`.
- [ ] Add `steamd` file parser and codegen.
  - [x] Enum parsing and codegen.
  - [ ] Class parsing and codgen.
  - [ ] Support constant values for classes.
  - [ ] Support default values for class members.
  - [ ] Trait for getting EMsg enum variant for classes.