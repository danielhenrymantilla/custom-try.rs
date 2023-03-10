# The following snippets fail to compile

## Arg expects an optional macro path or a bang-terminated macro path

```rust ,compile_fail
use ::custom_try::*;

#[::custom_try::custom_try(ill-formed)]
fn foo() {}
```

## Arg defaults to `r#try!()`, which will fail if not in scope

```rust ,compile_fail
#![no_implicit_prelude] // ensure no `::core::r#try!` accidentally in scope.

#[::custom_try::custom_try]
fn foo() { ()? }

fn main() {}
```

## Fail if custom-provided path is not in scope:

```rust ,compile_fail
use ::custom_try::*;

#[::custom_try::custom_try(non_existent)]
fn foo() { ()? }
```

<!-- Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template -->
