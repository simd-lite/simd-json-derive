use proc_macro::{self, TokenStream};
use proc_macro2::{Ident, Span};
use quote::quote;
use simd_json::prelude::*;
use syn::token::Comma;
use syn::{
    parse_macro_input, punctuated::Punctuated, Data, DataEnum, DataStruct, DeriveInput, Field,
    Fields, FieldsNamed, FieldsUnnamed, Generics, Path, PathSegment, Type, TypePath, Variant,
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
    let mut id: u64 = 1;
    let mut all: u64 = 0;
    let mut all_needed: u64 = 0;
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

        if let Some((name, ident)) =
            name(&attrs, f).and_then(|name| Some((name, f.ident.as_ref()?.clone())))
        {
            let name = name.trim_matches(':').trim_matches('"').to_string();
            let bit = 1 << id;
            id += 1;
            all = all | bit;
            if is_option {
                options.push(ident.clone());
                opt_ids.push(bit);
                getters.push(quote! { #ident.and_then(::std::convert::identity) })
            } else {
                all_needed = all_needed | bit;
                getters.push(quote! { #ident.expect(concat!("failed to get field ", #name))  })
            }
            keys.push(name);
            values.push(ident);
            ids.push(bit);
        }
    }

    let expanded = quote! {
        impl #generics ::simd_json_derive::Deserialize for #ident #generics {
            #[inline]
            fn from_tape<'input>(__deser_tape: &mut ::simd_json_derive::Tape<'input>) -> ::simd_json::Result<Self>
            where
                Self: std::marker::Sized + 'input
            {
                let mut __seen: u64 = 0;
                let mut __err: Option<::simd_json::Error> = None;
                let mut __res: Self = unsafe{::std::mem::MaybeUninit::uninit().assume_init()};
                let __deser_size: usize = if let Some(::simd_json::Node::Object(size, _)) = __deser_tape.next() {
                    size
                } else {
                    ::std::mem::forget(__res);
                    return Err(::simd_json::Error::generic(::simd_json::ErrorType::ExpectedMap));
                };

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
                                _ => {
                                    // FIXME: this should be togglable
                                    __err = Some(::simd_json::Error::generic(::simd_json::ErrorType::ExpectedMap));
                                    break
                                }
                            }
                        },
                        _ => {
                            unreachable!()
                        }
                    }
                }
                if (__seen & #all_needed) != #all_needed && __err.is_none() {
                    __err = Some(::simd_json::Error::generic(::simd_json::ErrorType::ExpectedMap))
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
