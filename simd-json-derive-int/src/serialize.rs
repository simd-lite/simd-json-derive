use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput};

use crate::args::StructAttrs;

mod r#enum;
mod r#struct;

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    match input {
        // struct
        DeriveInput {
            ident,
            attrs,
            data: Data::Struct(defn),
            generics,
            ..
        } => r#struct::derive(StructAttrs::parse(attrs), ident, generics, defn),
        DeriveInput {
            ident,
            data: Data::Enum(data),
            generics,
            attrs,
            ..
        } => r#enum::derive(StructAttrs::parse(attrs), ident, data, generics),
        _ => TokenStream::from(quote! {}),
    }
}
