//! Crate not intended for direct use.
//! Use https:://docs.rs/custom-try instead.
// Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template
#![allow(nonstandard_style, unused_imports)]

use ::core::{
    mem,
    ops::Not as _,
};
use ::proc_macro::{
    TokenStream,
};
use ::proc_macro2::{
    Span,
    TokenStream as TokenStream2,
    TokenTree as TT,
};
use ::quote::{
    format_ident,
    quote,
    quote_spanned,
    ToTokens,
};
use ::syn::{*,
    parse::{Parse, Parser, ParseStream},
    punctuated::Punctuated,
    Result, // Explicitly shadow it
    spanned::Spanned,
};

mod args;
mod visitor;

///
#[proc_macro_attribute] pub
fn custom_try(
    args: TokenStream,
    input: TokenStream,
) -> TokenStream
{
    custom_try_impl(args.into(), input.into())
    //  .map(|ret| { println!("{}", ret); ret })
        .unwrap_or_else(|err| {
            let mut errors =
                err .into_iter()
                    .map(|err| Error::new(
                        err.span(),
                        format_args!("`#[custom_try::custom_try]`: {}", err),
                    ))
            ;
            let mut err = errors.next().unwrap();
            errors.for_each(|cur| err.combine(cur));
            err.to_compile_error()
        })
        .into()
}

fn custom_try_impl(
    args: TokenStream2,
    input: TokenStream2,
) -> Result<TokenStream2>
{
    // By default deny any attribute present.
    let args::Args { macro_name } = parse2(args)?;
    let mut input: Item = parse2(input)?;

    visit_mut::VisitMut::visit_item_mut(
        &mut visitor::Visitor { macro_name },
        &mut input,
    );

    Ok(input.into_token_stream())
}
