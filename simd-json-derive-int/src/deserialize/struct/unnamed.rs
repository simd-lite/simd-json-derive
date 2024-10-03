use proc_macro::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Field, GenericParam, Generics, Ident};

use crate::args::StructAttrs;

pub(crate) fn derive(
    _attrs: StructAttrs,
    ident: Ident,
    generics: Generics,
    fields: Punctuated<Field, Comma>,
) -> proc_macro::TokenStream {
    let params = &generics.params;
    let (all_generics, derive_lt) = match params.first() {
        None => (quote! { <'input> }, quote! { 'input }),
        Some(GenericParam::Lifetime(lifetime)) => (quote! { <#params> }, quote! { #lifetime }),
        Some(_) => (quote! { <'input, #params> }, quote! { 'input }),
    };

    if fields.len() == 1 {
        // This is a newtype

        let expanded = quote! {
            impl #all_generics ::simd_json_derive::Deserialize <#derive_lt> for #ident #generics {
                #[inline]
                fn from_tape(__deser_tape: &mut ::simd_json_derive::Tape<#derive_lt>) -> ::simd_json::Result<Self>
                where
                    Self: std::marker::Sized + #derive_lt
                {
                    match ::simd_json_derive::Deserialize::from_tape(__deser_tape) {
                        Ok(__inner) => Ok(Self(__inner)),
                        Err(e) => Err(e)
                    }
                }
            }
        };
        TokenStream::from(expanded)
    } else {
        todo!("Only newtype unnamed structs are supported by now")
    }
}
