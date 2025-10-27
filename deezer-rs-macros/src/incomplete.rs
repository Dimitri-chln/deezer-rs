use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, spanned::Spanned};

use crate::attribute::{CommaSeparatedMetaNameValues, attribute};

pub fn derive_inner(input: DeriveInput) -> syn::Result<TokenStream> {
    let attr = attribute(&input, "deezer")?;
    let list = attr.meta.require_list()?;
    let args = list.parse_args::<CommaSeparatedMetaNameValues>()?;

    let mut full_object_type = None;

    for meta in args.inner.iter() {
        if meta.path.is_ident("object") {
            full_object_type = Some(&meta.value);
        }
    }

    let full_object_type = full_object_type
        .ok_or_else(|| syn::Error::new(list.span(), "`object` must be specified"))?;

    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let token_stream = quote! {
        impl #impl_generics crate::objects::traits::IncompleteObject for #ident #type_generics #where_clause {
            type FullObject = #full_object_type;

            fn id(&self) -> crate::Id {
                self.id
            }
        }
    };

    Ok(token_stream)
}
