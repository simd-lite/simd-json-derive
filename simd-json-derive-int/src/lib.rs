use proc_macro::TokenStream;
use proc_macro::{self};
use proc_macro2::{Ident, Span};
use quote::quote;
use simd_json::prelude::*;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed,
    Variant,
};

#[proc_macro_derive(Serialize)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match input {
        DeriveInput {
            ident,
            data:
                Data::Struct(DataStruct {
                    fields: Fields::Unnamed(FieldsUnnamed { unnamed, .. }),
                    ..
                }),
            ..
        } => {
            if unnamed.len() == 1 {
                let expanded = quote! {
                    impl simd_json_derive::Serialize for #ident {
                        fn json_write<W>(&self, writer: &mut W) -> std::io::Result<()>
                        where
                            W: std::io::Write {
                                self.0.json_write(writer)
                            }
                    }
                };
                TokenStream::from(expanded)
            } else {
                let keys: Vec<_> = unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| syn::Index::from(i))
                    .skip(1)
                    .collect();
                let expanded = quote! {
                    impl simd_json_derive::Serialize for #ident {
                        fn json_write<W>(&self, writer: &mut W) -> std::io::Result<()>
                        where
                            W: std::io::Write {
                                if let Err(e) = writer.write_all(b"[") {
                                    return Err(e)
                                }
                                    if let Err(e) = self.0.json_write(writer) {
                                        return Err(e)
                                    }
                                #(
                                    if let Err(e) = writer.write_all(b",") {
                                        return Err(e)
                                    };
                                    if let Err(e) = self.#keys.json_write(writer) {
                                        return Err(e)
                                    };
                                )*
                                writer.write_all(b"]")
                            }
                    }
                };
                TokenStream::from(expanded)
            }
        }
        DeriveInput {
            ident,
            data:
                Data::Struct(DataStruct {
                    fields: Fields::Named(FieldsNamed { named, .. }),
                    ..
                }),
            ..
        } => {
            let (mut keys, values): (Vec<_>, Vec<_>) = named
                .iter()
                .filter_map(|f| {
                    f.ident.clone().map(|ident| {
                        (
                            format!(
                                "{}:",
                                simd_json::OwnedValue::from(ident.to_string()).encode()
                            ),
                            ident,
                        )
                    })
                })
                .unzip();

            if let Some((first, rest)) = keys.split_first_mut() {
                *first = format!("{{{}", first);
                for r in rest {
                    *r = format!(",{}", r);
                }
            };

            let expanded = quote! {
                impl simd_json_derive::Serialize for #ident {
                    #[inline]
                    fn json_write<W>(&self, writer: &mut W) -> std::io::Result<()>
                    where
                        W: std::io::Write {
                            #(
                                if let Err(e) = writer.write_all(#keys.as_bytes()) {
                                    return Err(e);
                                }
                                if let Err(e) = self.#values.json_write(writer) {
                                    return Err(e);
                                }
                            )*
                            writer.write_all(b"}")
                        }
                }
            };
            TokenStream::from(expanded)
        }
        DeriveInput {
            ident,
            data: Data::Enum(d),
            ..
        } => {
            dbg!(&d);
            let variants = dbg!(d.variants);

            let (simple, variants): (Vec<_>, Vec<_>) =
                variants.into_iter().partition(|v| v.fields.is_empty());

            dbg!(&simple);
            let (named, unnamed): (Vec<_>, Vec<_>) = variants.iter().partition(|v| {
                if let Variant {
                    fields: Fields::Named(_),
                    ..
                } = v
                {
                    true
                } else {
                    false
                }
            });

            let (unnamed1, unnamed): (Vec<_>, Vec<_>) =
                unnamed.into_iter().partition(|v| v.fields.len() == 1);

            // enum no fields of Enum::Variant
            // They serialize as: "Varriant"

            let (simple_keys, simple_values): (Vec<_>, Vec<_>) = simple
                .iter()
                .map(|s| {
                    (
                        &s.ident,
                        format!(
                            "{}",
                            simd_json::OwnedValue::from(s.ident.to_string()).encode()
                        ),
                    )
                })
                .unzip();
            let simple = quote! {
                #(
                    #ident::#simple_keys => writer.write_all(#simple_values.as_bytes())
                ),*
            };

            // Unnamed enum variants with exactly 1 field of Enum::Variant(type1)
            // They serialize as: {"Varriant":..}

            let (unnamed1_idents, unnamed1_keys): (Vec<_>, Vec<_>) = unnamed1
                .iter()
                .map(|v| {
                    (
                        &v.ident,
                        format!(
                            "{{{}:",
                            simd_json::OwnedValue::from(v.ident.to_string()).encode()
                        ),
                    )
                })
                .unzip();
            let unnamed1 = quote! {
                #(
                    #ident::#unnamed1_idents(v) => {
                        if let Err(e) = writer.write_all(#unnamed1_keys.as_bytes()) {
                            return Err(e);
                        };
                        if let Err(e) = v.json_write(writer) {
                            return Err(e);
                        };
                        writer.write_all(b"}")
                    }
                ),*
            };

            // Unnamed enum variants with more then 1 field of Enum::Variant(type1, type2, type3)
            // They serialize as: {"Varriant":[.., .., ..]}

            let (unnamed_ident_and_vars, unnamed_keys): (Vec<_>, Vec<_>) = unnamed
                .iter()
                .map(|v| {
                    (
                        (
                            &v.ident,
                            (0..v.fields.len())
                                .map(|i| Ident::new(&format!("v{}", i), Span::call_site()))
                                .collect::<Vec<_>>(),
                        ),
                        format!(
                            "{{{}:[",
                            simd_json::OwnedValue::from(v.ident.to_string()).encode()
                        ),
                    )
                })
                .unzip();

            let (unnamed_idents, unnamed_var_names): (Vec<_>, Vec<_>) =
                unnamed_ident_and_vars.into_iter().unzip();

            let unnamed_vecs = unnamed_var_names.iter().map(|vs| {
                let (first, rest) = vs.split_first().unwrap();
                quote! {
                    if let Err(e) = #first.json_write(writer) {
                        return Err(e);
                    };
                    #(
                        if let Err(e) = writer.write_all(b",") {
                            return Err(e);
                        };
                        if let Err(e) = #rest.json_write(writer) {
                            return Err(e);
                        };
                    )*
                }
            });

            let unnamed_vars = unnamed_var_names.iter().map(|vs| quote! { #(#vs),* });

            let unnamed = quote! {
                #(
                    #ident::#unnamed_idents(#unnamed_vars) =>
                    {
                        if let Err(e) = writer.write_all(#unnamed_keys.as_bytes()) {
                            return Err(e);
                        };
                        #unnamed_vecs
                        writer.write_all(b"]}")
                    }
                ),*
            };

            // Named enum variants of the form Enum::Variant{key1: type, key2: type...}
            // They serialize as: {"Varriant":{"key1":..,"key2":..}}

            let mut named_bodies = Vec::new();
            for v in named {
                let named_ident = &v.ident;
                let fields: Vec<_> = v.fields.iter().cloned().map(|f| f.ident.unwrap()).collect();
                let (first, rest) = fields.split_first().unwrap();

                let start = format!(
                    "{{{}:{{{}:",
                    simd_json::OwnedValue::from(v.ident.to_string()).encode(),
                    simd_json::OwnedValue::from(first.to_string()).encode()
                );

                let rest_keys = rest
                    .iter()
                    .map(|f| format!(",{}:", simd_json::OwnedValue::from(f.to_string()).encode()));

                named_bodies.push(quote! {
                    #ident::#named_ident{#(#fields),*} => {
                        if let Err(e) = writer.write_all(#start.as_bytes()) {
                            return Err(e);
                        };
                        if let Err(e) = #first.json_write(writer) {
                            return Err(e);
                        };
                        #(
                            if let Err(e) = writer.write_all(#rest_keys.as_bytes()) {
                                return Err(e);
                            };
                            if let Err(e) = #rest.json_write(writer) {
                                return Err(e);
                            };

                        )*
                        writer.write_all(b"}}")
                    }
                });
            }
            let named = quote! {#(#named_bodies),*};

            let body_elements = [simple, unnamed1, unnamed, named];
            let match_body = quote! {
                #(#body_elements),*
            };

            let expanded = quote! {
                impl simd_json_derive::Serialize for #ident {
                    #[inline]
                    fn json_write<W>(&self, writer: &mut W) -> std::io::Result<()>
                    where
                        W: std::io::Write {
                            match self {
                                #match_body
                            }

                        }
                }
            };
            TokenStream::from(expanded)
        }
        _ => {
            dbg!(input);
            TokenStream::from(quote! {})
        }
    }
}
