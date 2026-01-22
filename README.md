# Freedom Models

[![Crates.io](https://img.shields.io/crates/v/freedom-models.svg)](https://crates.io/crates/freedom-models)
[![Documentation](https://docs.rs/freedom-models/badge.svg)](https://docs.rs/freedom-models/)

Contains data structures which map to Freedom resources to simplify usage
of the [Freedom API](https://github.com/ATLAS-Space-Operations/rust-freedom-api)

## HATEOAS Trait

In addition, the library exposes a single trait `Hateoas`. This is
useful for navigating the
[HATEOAS](https://en.wikipedia.org/wiki/HATEOAS) structure of the API.

## Unstable Flag

By default, all public structs in the crate are marked
`non_exhaustive`. This is because we may--at any time--add a field to
one or more of the structs, and this should not constitute a breaking
change for users.

However, for users who would like to construct the models for testing
purposes or as their baseline we do provide the `unstable` feature
flag, which disables this behavior when set.

By using the `unstable` flag you accept that your construction of
models might break between releases.
