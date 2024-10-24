use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{DataEnum, Fields, GenericParam, Generics, Variant};

use crate::args::StructAttrs;

pub(super) fn derive(
    _attrs: StructAttrs,
    ident: Ident,
    generics: Generics,
    data: DataEnum,
) -> proc_macro::TokenStream {
    let params = &generics.params;
    let (all_generics, derive_lt) = match params.first() {
        None => (quote! { <'input> }, quote! { 'input }),
        Some(GenericParam::Lifetime(lifetime)) => (quote! { <#params> }, quote! { #lifetime }),
        Some(_) => (quote! { <'input, #params> }, quote! { 'input }),
    };

    // let mut body_elements = Vec::new();
    let variants = data.variants;
    let (simple, variants): (Vec<_>, Vec<_>) =
        variants.into_iter().partition(|v| v.fields.is_empty());
    let (named, unnamed): (Vec<_>, Vec<_>) = variants.iter().partition(|v| {
        matches!(
            v,
            Variant {
                fields: Fields::Named(_),
                ..
            }
        )
    });

    let (unnamed1, unnamed): (Vec<_>, Vec<_>) =
        unnamed.into_iter().partition(|v| v.fields.len() == 1);
    if !named.is_empty() {
        todo!(
            "ENUM variants with named fields are not supported: {:?}",
            named
        );
    }
    let (unnamed_keys, unnamed_values): (Vec<_>, Vec<_>) = unnamed
        .iter()
        .map(|s| {
            (
                &s.ident,
                (
                    s.ident.to_string(),
                    s.fields
                        .iter()
                        .enumerate()
                        .map(|f| format_ident!("_unnamed_{}", f.0))
                        .collect::<Vec<_>>(),
                ),
            )
        })
        .unzip();
    let (unnamed_values, unnamed_fields): (Vec<_>, Vec<_>) = unnamed_values
        .into_iter()
        .map(|(v, f)| {
            (
                v,
                (
                    f.len(),
                    quote! {
                        #(
                            {
                                let #f = ::simd_json_derive::Deserialize::from_tape(__deser_tape)?;
                                #f
                            }
                        ),*
                    },
                ),
            )
        })
        .unzip();
    let (unnamed_len, unnamed_fields): (Vec<_>, Vec<_>) = unnamed_fields.into_iter().unzip();
    let unnamed = quote! {
        #(
            Some(::simd_json::Node::String(#unnamed_values)) => {
                match __deser_tape.next() {
                  Some(::simd_json::Node::Array{len: #unnamed_len, ..}) => Ok(#ident::#unnamed_keys(#unnamed_fields)),
                  _ => Err(::simd_json_derive::de::Error::FieldNotAnArray(#unnamed_values))
                }

           },
        )*
    };
    // unnamed 1
    let (unnamed1_keys, unnamed1_values): (Vec<_>, Vec<_>) = unnamed1
        .iter()
        .map(|s| (&s.ident, s.ident.to_string()))
        .unzip();
    let unnamed1 = quote! {
        #(
            Some(::simd_json::Node::String(#unnamed1_values)) => Ok(#ident::#unnamed1_keys(::simd_json_derive::Deserialize::from_tape(__deser_tape)?)),
        )*
    };

    let (simple_keys, simple_values): (Vec<_>, Vec<_>) = simple
        .iter()
        .map(|s| (&s.ident, s.ident.to_string()))
        .unzip();
    let simple = quote! {
        #(
            Some(::simd_json::Node::String(#simple_values)) => Ok(#ident::#simple_keys),
        )*
    };
    let expanded = quote! {
        impl #all_generics ::simd_json_derive::Deserialize <#derive_lt> for #ident #generics {
            #[inline]
            fn from_tape(__deser_tape: &mut ::simd_json_derive::Tape<#derive_lt>) -> ::simd_json_derive::de::Result<Self>
            where
                Self: std::marker::Sized + #derive_lt
            {
                match __deser_tape.next() {
                    #simple
                    Some(::simd_json::Node::Object{len: 1, ..}) => {
                        match __deser_tape.next() {
                            #unnamed1
                            #unnamed
                            Some(::simd_json::Node::String(__other)) => Err(::simd_json_derive::de::Error::UnknownEnumVariant(__other.to_string()).into()),
                            Some(_) => Err(::simd_json_derive::de::Error::InvalidEnumRepresentation),
                            None => Err(::simd_json_derive::de::Error::EOF)
                        }
                    },
                    Some(__other) => Err(::simd_json_derive::de::Error::InvalidEnumRepresentation),
                    None => Err(::simd_json_derive::de::Error::EOF)
                }
            }
        }
    };
    TokenStream::from(expanded)
}
