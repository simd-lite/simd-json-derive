use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, Data, DeriveInput};

use crate::args::*;

mod r#struct;

mod r#enum;
pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match input {
        // struct
        DeriveInput {
            ident,
            attrs,
            data: Data::Struct(defn),
            generics,
            ..
        } => r#struct::derive(StructAttrs::parse(attrs), ident, generics, defn), // Named
        DeriveInput {
            ident,
            attrs,
            data: Data::Enum(defn),
            generics,
            ..
        } => r#enum::derive(StructAttrs::parse(attrs), ident, generics, defn),
        _ => unimplemented!("This was trying to derive something odd"),
    }
}
