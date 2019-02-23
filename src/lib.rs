#![feature(proc_macro_hygiene)]

/// Allows creating scopes in which an outer variable is cloned.
///
/// For example suppose we have a variable `foo` we want to clone into two closures.
/// This is normally quite clumsy:
/// let bar = {let foo = foo.clone(); move |baz| foo.do(baz)}.
///
/// With this macro we can express it without the extra {} as follows:
/// let bar =
///     #[clone(foo)]
///     move |baz| foo.do(baz);
/// 
/// Multiple (comma-separated) and mutable variables are supported.

extern crate proc_macro;

use proc_macro::*;

#[proc_macro_attribute]
pub fn clone(vars: TokenStream, block: TokenStream) -> TokenStream {
    use std::iter::FromIterator;

    // produce a right-hand side of a tuple let binding where each identifier is cloned
    let vars_rhs = TokenStream::from_iter(
        vars
        .clone()
        .into_iter()
        .filter_map(|t| {
            // strip out commas and muts from RHS (passed through unchanged on LHS)
            let ts = t.to_string();
            if ts != "," && ts != "mut" {
                Some(quote!{$t.clone(),})
            }
            else {
                None
            }
        }));

    // create a scope block containing the let binding and the inner block.
    let out = quote!{
        {
            let ($vars,) = ($vars_rhs);
            $block
        }
    };

    //println!("{}", out);

    out
}
