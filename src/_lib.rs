#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]

#[allow(unused)]
use {
    ::core::{
        ops::Not as _,
    },
};

#[cfg(COMMENTED_OUT)] // <- Remove this when used!
/// The crate's prelude.
pub
mod prelude {
    // …
}

/// Replace `expr?` occurrences in the decorated item with `r#try!( expr )`.
///
/// The `r#try!` default path can be overridden by giving it as an argument
/// to the attribute macro:
///
/// ## Examples
///
/// ### 1. For FFI
///
/**  - ```rust
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
            None => return $crate::FfiResult::ERR,
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
    # fn main() {}
    # fn the_answer_to_life_the_universe_and_everything() -> Option<i32> { None }
    ``` */
pub use ::custom_try_proc_macros::custom_try;

// macro internals
#[doc(hidden)] /** Not part of the public API */ pub
mod ඞ {
    pub use ::core; // or `std`
}

#[doc = include_str!("compile_fail_tests.md")]
mod _compile_fail_tests {}
