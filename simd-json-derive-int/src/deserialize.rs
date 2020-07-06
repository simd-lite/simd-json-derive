use proc_macro::{self, TokenStream};
use proc_macro2::{Ident, Span};
use quote::quote;
use simd_json::prelude::*;
use syn::token::Comma;
use syn::{
    parse_macro_input, punctuated::Punctuated, Data, DataEnum, DataStruct, DeriveInput, Field,
    Fields, FieldsNamed, FieldsUnnamed, Generics, Variant,
};

use crate::args::*;


/// Named struct as `Struct(u8)` or `Struct(u8, String)`
fn derive_named_struct(
    attrs: StructAttrs,
    ident: Ident,
    generics: Generics,
    fields: Punctuated<Field, Comma>,
) -> proc_macro::TokenStream {
    let mut keys = Vec::new();
    let mut values = Vec::new();

    for f in &fields {
        if let Some((name, ident)) =
            name(&attrs, f).and_then(|name| Some((name, f.ident.as_ref()?.clone())))
        {
            keys.push(name.trim_matches(':').trim_matches('"').to_string());
            values.push(ident);
        }
    }

    let expanded = quote! {
        impl #generics simd_json_derive::Deserialize for #ident #generics {
            #[inline]
            fn from_tape<'input>(__deser_tape: &mut simd_json_derive::Tape<'input>) -> simd_json::Result<Self>
            where
                Self: std::marker::Sized + 'input
            {
                #(
                    let mut #values = None;
                )*

                let __deser_size: usize = if let Some(simd_json::Node::Object(size, _)) = __deser_tape.next() {
                    size
                } else {
                    return Err(simd_json::Error::generic(simd_json::ErrorType::ExpectedMap));
                };
                for _ in 0..__deser_size {
                    if let Some(simd_json::Node::String(__deser_key)) = __deser_tape.next() {
                        match __deser_key {
                            #(
                                #keys => {
                                    #values = Some(simd_json_derive::Deserialize::from_tape(__deser_tape)?);
                                },
                            )*
                            _ => ()
                        }
                    };
                }


                Ok(Self {
                    #(
                        #values: #values.unwrap(),
                    )*
                })
            }
        }
    };
    TokenStream::from(expanded)
}

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match input {
        // // Unnamed struct
        // DeriveInput {
        //     ident,
        //     data:
        //         Data::Struct(DataStruct {
        //             fields: Fields::Unnamed(FieldsUnnamed { unnamed, .. }),
        //             ..
        //         }),
        //     generics,
        //     ..
        // } => derive_unnamed_struct(ident, generics, unnamed),
        // Named
        DeriveInput {
            ident,
            attrs,
            data:
                Data::Struct(DataStruct {
                    fields: Fields::Named(FieldsNamed { named, .. }),
                    ..
                }),
            generics,
            ..
        } => {
            let attrs = if let Some(attrs) = get_attr(&attrs, "simd_json") {
                struct_attrs(attrs)
            } else if let Some(attrs) = get_attr(&attrs, "serde") {
                struct_attrs(attrs)
            } else {
                StructAttrs::default()
            };
            derive_named_struct(attrs, ident, generics, named)
        }
        // DeriveInput {
        //     ident,
        //     data: Data::Enum(data),
        //     generics,
        //     ..
        // } => derive_enum(ident, data, generics),
        _ => TokenStream::from(quote! {}),
    }
}