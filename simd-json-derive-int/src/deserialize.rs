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
    let mut keys = Vec::new();
    let mut values = Vec::new();
    let mut getters = Vec::new();
    let mut options = Vec::new();
    let mut ids = Vec::new();
    let mut opt_ids = Vec::new();
    let mut id: u64 = 0;
    let mut all_needed: u64 = 0;
    let deny_unknown_fields: bool = attrs.deny_unknown_fields();
    let params = &generics.params;
    let (all_generics, derive_lt) = match params.first() {
        None => (quote! { <'input> }, quote! { 'input }),
        Some(GenericParam::Lifetime(lifetime)) => (quote! { <#params> }, quote! { #lifetime }),
        Some(_) => (quote! { <'input, #params> }, quote! { 'input }),
    };
    for f in &fields {
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

        if let Some((name, ident)) = attrs
            .name(f)
            .and_then(|name| Some((name, f.ident.as_ref()?.clone())))
        {
            let name = name.trim_matches(':').trim_matches('"').to_string();
            let bit = 1 << id;
            id += 1;
            if is_option {
                options.push(ident.clone());
                opt_ids.push(bit);
                getters.push(quote! { #ident.and_then(::std::convert::identity) })
            } else {
                all_needed |= bit;
                getters.push(quote! { #ident.expect(concat!("failed to get field ", #name))  })
            }
            keys.push(name);
            values.push(ident);
            ids.push(bit);
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
                let __deser_size: usize = if let Some(::simd_json::Node::Object(size, _)) = __deser_tape.next() {
                    size
                } else {
                    return Err(::simd_json::Error::generic(::simd_json::ErrorType::ExpectedMap));
                };
                let mut __seen: u64 = 0;
                let mut __err: Option<::simd_json::Error> = None;
                let mut __res: Self = unsafe{::std::mem::MaybeUninit::uninit().assume_init()};

                for _ in 0..__deser_size {
                    match __deser_tape.next() {
                        Some(::simd_json::Node::String(__deser_key)) =>  {
                            match __deser_key {
                                #(
                                #keys => {
                                    match ::simd_json_derive::Deserialize::from_tape(__deser_tape) {
                                        Ok(v) => unsafe {
                                            __seen |= #ids;
                                            ::std::ptr::write(&mut __res.#values, v);
                                        }
                                        Err(e) => {
                                            __err = Some(e);
                                            break
                                        }
                                    }

                                },
                                )*
                                __unknown_field if #deny_unknown_fields => {
                                    use ::serde::de::Error;
                                    __err = Some(::simd_json::Error::unknown_field(__unknown_field, &[ #(#keys),* ]));
                                    break
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
                if (__seen & #all_needed) != #all_needed && __err.is_none() {

                    use ::serde::de::Error;
                    // extract the missing field names
                    let mut __missing_field_ids: u64 = (__seen ^ #all_needed);
                    let mut __missing_fields: Vec<&'static str> = Vec::with_capacity(__missing_field_ids.count_ones() as usize);
                    while __missing_field_ids != 0 {
                        #(
                            if #ids & __missing_field_ids == #ids {
                                __missing_fields.push(#keys);
                                __missing_field_ids ^= #ids; // clear out the bit
                            }
                        )*
                    }
                    __err = Some(::simd_json::Error::custom(format!("{}: `{}`", "missing fields", __missing_fields.join("`, `"))));
                }

                if let Some(e) = __err {
                    unsafe{
                        let #ident {
                            #(
                                #values,
                            )*
                        } = __res;
                        #(
                        if #ids & __seen == 0 {
                            #[allow(clippy::forget_ref)]
                            ::std::mem::forget(#values);
                        };
                        )*
                    }
                    Err(e)
                } else {
                    unsafe {
                        #(
                        if #opt_ids & __seen == 0 {
                            ::std::ptr::write(&mut __res.#options, None);
                        };
                        )*
                    }
                    Ok( __res )
                }


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
