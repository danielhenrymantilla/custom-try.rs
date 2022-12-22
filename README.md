# `::custom-try`

Macro to customize the behavior of `?`

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)](
https://github.com/danielhenrymantilla/custom-try.rs)
[![Latest version](https://img.shields.io/crates/v/custom-try.svg)](
https://crates.io/crates/custom-try)
[![Documentation](https://docs.rs/custom-try/badge.svg)](
https://docs.rs/custom-try)
[![MSRV](https://img.shields.io/badge/MSRV-1.56.0-white)](
https://gist.github.com/danielhenrymantilla/8e5b721b3929084562f8f65668920c33)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](
https://github.com/rust-secure-code/safety-dance/)
[![License](https://img.shields.io/crates/l/custom-try.svg)](
https://github.com/danielhenrymantilla/custom-try.rs/blob/master/LICENSE-ZLIB)
[![CI](https://github.com/danielhenrymantilla/custom-try.rs/workflows/CI/badge.svg)](
https://github.com/danielhenrymantilla/custom-try.rs/actions)

<!-- Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template -->

## Examples

```rust
use ::custom_try::custom_try;

#[repr(transparent)]
pub struct FfiResult {
    pub status_code: ::std::os::raw::c_int,
}

impl FfiResult {
    pub const OK: Self = Self { status_code: 0 };
    pub const ERR: Self = Self { status_code: -1 };
}

macro_rules! unwrap_option {( $option:expr $(,)? ) => (
    match $option {
        Some(thing) => thing,
        None => return FfiResult::ERR,
    }
)}

#[custom_try(unwrap_option!)]
extern "C" fn ffi_function() -> FfiResult {
    let x = the_answer_to_life_the_universe_and_everything()?;
    println!("{}", x);
    FfiResult::OK
}

/// If you only have one case of `?` semantics, you can default to that one
/// using the default `r#try!` macro name.
use unwrap_option as r#try;

#[custom_try]
extern "C" fn ffi_function2() -> FfiResult {
    let x = the_answer_to_life_the_universe_and_everything()?;
    println!("{}", x);
    FfiResult::OK
}
#
# fn the_answer_to_life_the_universe_and_everything()
#   -> Option<i32>
# {
#     None
# }
```
