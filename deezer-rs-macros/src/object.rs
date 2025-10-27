use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, spanned::Spanned};

use crate::attribute::{CommaSeparatedMetaNameValues, attribute};

pub fn derive_inner(input: DeriveInput) -> syn::Result<TokenStream> {
    let attr = attribute(&input, "deezer")?;
    let list = attr.meta.require_list()?;
    let args = list.parse_args::<CommaSeparatedMetaNameValues>()?;

    let mut endpoint = None;

    for meta in args.inner.iter() {
        if meta.path.is_ident("endpoint") {
            endpoint = Some(&meta.value);
        }
    }

    let endpoint =
        endpoint.ok_or_else(|| syn::Error::new(list.span(), "`endpoint` must be specified"))?;

    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let token_stream = quote! {
        impl #impl_generics crate::objects::traits::Object for #ident #type_generics #where_clause {
            const ENDPOINT: &str = #endpoint;
        }
    };

    Ok(token_stream)
}
