use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod attribute;
mod incomplete;
mod object;

#[proc_macro_derive(DeezerObject, attributes(deezer))]
pub fn derive_deezer_object(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    object::derive_inner(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(DeezerIncomplete, attributes(deezer))]
pub fn derive_deezer_incomplete(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    incomplete::derive_inner(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
