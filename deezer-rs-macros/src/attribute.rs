use std::fmt::Display;

use syn::{
    Attribute, DeriveInput, Ident, MetaNameValue, Token, parse::Parse, punctuated::Punctuated,
    spanned::Spanned,
};

pub fn attribute<'i, I>(input: &'i DeriveInput, ident: &I) -> syn::Result<&'i Attribute>
where
    I: ?Sized + Display,
    Ident: PartialEq<I>,
{
    input
        .attrs
        .iter()
        .find(|attr| attr.meta.path().is_ident(ident))
        .ok_or(syn::Error::new(
            input.span(),
            format!("no `{ident}` attribute found"),
        ))
}

pub struct CommaSeparatedMetaNameValues {
    pub inner: Punctuated<MetaNameValue, Token![,]>,
}

impl Parse for CommaSeparatedMetaNameValues {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            inner: Punctuated::parse_terminated(input)?,
        })
    }
}
