use proc_macro::{self, TokenStream};
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::token::Comma;
use syn::{
    parse_macro_input, punctuated::Punctuated, Data, DataEnum, DataStruct, DeriveInput, Field,
    Fields, FieldsNamed, FieldsUnnamed, GenericParam, Generics, Path, PathSegment, Type, TypePath,
    Variant,
};

use crate::args::*;

/// Named struct as `Struct(u8)` or `Struct(u8, String)`
fn derive_named_struct(
    attrs: StructAttrs,
    ident: Ident,
    generics: Generics,
    fields: Punctuated<Field, Comma>,
) -> proc_macro::TokenStream {
    let mut value_keys = Vec::new();
    let mut value_locals = Vec::new();
    let mut values = Vec::new();

    let mut options = Vec::new();
    let mut option_locals = Vec::new();
    let mut option_keys = Vec::new();

    let deny_unknown_fields: bool = attrs.deny_unknown_fields();
    let params = &generics.params;
    let (all_generics, derive_lt) = match params.first() {
        None => (quote! { <'input> }, quote! { 'input }),
        Some(GenericParam::Lifetime(lifetime)) => (quote! { <#params> }, quote! { #lifetime }),
        Some(_) => (quote! { <'input, #params> }, quote! { 'input }),
    };
    for (id, f) in fields.iter().enumerate() {
        let mut is_option = false;
        if let Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) = &f.ty
        {
            if let Some(PathSegment { ident, .. }) = segments.first() {
                is_option = ident == "Option";
            }
        }

        let ident = f.ident.clone().expect("Missing ident");
        let name = attrs.name(f);
        let name = name.trim_matches(':').trim_matches('"').to_string();
        if is_option {
            options.push(ident.clone());
            option_locals.push(format_ident!("__option_{}", id));
            option_keys.push(name);
        } else {
            values.push(ident);
            value_locals.push(format_ident!("__value_{}", id));
            value_keys.push(name);
        }
    }

    let expanded = quote! {
        impl #all_generics ::simd_json_derive::Deserialize <#derive_lt> for #ident #generics {
            #[inline]
            #[allow(clippy::forget_copy)]
            #[allow(clippy::forget_non_drop)]
            fn from_tape(__deser_tape: &mut ::simd_json_derive::Tape <#derive_lt>) -> ::simd_json::Result<Self>
            where
                Self: std::marker::Sized + #derive_lt
            {
                use ::serde::de::Error;
                let __deser_size: usize = if let Some(::simd_json::Node::Object(size, _)) = __deser_tape.next() {
                    size
                } else {
                    return Err(::simd_json::Error::generic(::simd_json::ErrorType::ExpectedMap));
                };

                #(let mut #value_locals = None;)*
                #(let mut #option_locals = None;)*

                for _ in 0..__deser_size {
                    match __deser_tape.next() {
                        Some(::simd_json::Node::String(__deser_key)) =>  {
                            match __deser_key {
                                #(
                                #value_keys => {
                                    let v = ::simd_json_derive::Deserialize::from_tape(__deser_tape)?;
                                    #value_locals = Some(v);
                                }
                                )*
                                #(
                                #option_keys => {
                                    #option_locals = ::simd_json_derive::Deserialize::from_tape(__deser_tape)?;
                                }
                                )*
                                __unknown_field if #deny_unknown_fields => {
                                    return Err(::simd_json::Error::unknown_field(__unknown_field, &[ #(#value_keys,)* #(#option_keys,)* ]));
                                }
                                _ => {
                                    // ignore unknown field
                                    ::simd_json_derive::__skip(1, __deser_tape)
                                }
                            }
                        },
                        // There are no more elements
                        _ => break
                    }
                }
                Ok(#ident {
                        #(
                            #options: #option_locals,
                        )*
                        #(
                            #values: #value_locals.ok_or_else(|| ::simd_json::Error::custom(format!("missing field: `{}`", #value_keys)))?,
                        )*
                })
            }
        }
    };
    TokenStream::from(expanded)
}

fn derive_unnamed_struct(
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
        unimplemented!("Only newtype unnamed structs are supported by now")
    }
}

fn derive_enum(
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
        panic!(
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
                  Some(::simd_json::Node::Array(#unnamed_len, _)) => Ok(#ident::#unnamed_keys(#unnamed_fields)),
                  _ => Err(::simd_json::Error::generic(::simd_json::ErrorType::ExpectedArray))   // FIXME
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
            fn from_tape(__deser_tape: &mut ::simd_json_derive::Tape<#derive_lt>) -> ::simd_json::Result<Self>
            where
                Self: std::marker::Sized + #derive_lt
            {
                match __deser_tape.next() {
                    #simple
                    Some(::simd_json::Node::Object(1, _)) => {
                        match __deser_tape.next() {
                            #unnamed1
                            #unnamed
                            Some(__other) => Err(::simd_json::Error::generic(::simd_json::ErrorType::ExpectedMap)), // FIXME
                            None => Err(::simd_json::Error::generic(::simd_json::ErrorType::ExpectedMap)) // FIXME
                        }
                    },
                    Some(__other) => Err(::simd_json::Error::generic(::simd_json::ErrorType::ExpectedMap)), // FIXME
                    None => Err(::simd_json::Error::generic(::simd_json::ErrorType::ExpectedMap)) // FIXME
                }
            }
        }
    };
    TokenStream::from(expanded)
}

pub(crate) fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match input {
        // Unnamed
        DeriveInput {
            ident,
            attrs,
            data:
                Data::Struct(DataStruct {
                    fields: Fields::Unnamed(FieldsUnnamed { unnamed, .. }),
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
            derive_unnamed_struct(attrs, ident, generics, unnamed)
        } // Named
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
        DeriveInput {
            ident,
            attrs,
            data: Data::Enum(data),
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
            derive_enum(attrs, ident, generics, data)
        }
        _ => unimplemented!("This was trying to derive something odd"),
    }
}
