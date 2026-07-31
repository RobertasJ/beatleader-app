use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Ident, Result, Token,
    parse::{Parse, ParseStream},
};

#[derive(Default)]
pub struct Attributes {
    pub bon: Option<Ident>,
    pub maybe_ext: Option<Ident>,
    pub container_ext: Option<Ident>,
    pub container_with_content_ext: Option<Ident>,
}

impl Attributes {
    pub fn marker_extensions(&self) -> [Option<(Ident, TokenStream)>; 3] {
        [
            self.maybe_ext
                .clone()
                .map(|token| (token, quote!(MaybeExt))),
            self.container_ext
                .clone()
                .map(|token| (token, quote!(ContainerExt))),
            self.container_with_content_ext
                .clone()
                .map(|token| (token, quote!(ContainerWithContentExt))),
        ]
    }
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attrs = Self::default();

        while !input.is_empty() {
            let ident: Ident = input.parse()?;

            let slot = if ident == "bon" {
                &mut attrs.bon
            } else if ident == "maybe_ext" {
                &mut attrs.maybe_ext
            } else if ident == "container_ext" {
                &mut attrs.container_ext
            } else if ident == "container_with_content_ext" {
                &mut attrs.container_with_content_ext
            } else {
                return Err(syn::Error::new_spanned(ident, "invalid attribute"));
            };

            if slot.replace(ident).is_some() {
                return Err(syn::Error::new_spanned(
                    slot.as_ref().unwrap(),
                    "duplicate attribute",
                ));
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(attrs)
    }
}
