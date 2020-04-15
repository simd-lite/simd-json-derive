use proc_macro::TokenStream;
use proc_macro::{self};
use proc_macro2;
use quote::quote;
use simd_json::prelude::*;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed,
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
            dbg!(unnamed);
            let expanded = quote! {
                impl simd_json_derive::Serialize for #ident {
                    fn json_write<W>(&self, writer: &mut W) -> std::io::Result<()>
                    where
                        W: std::io::Write {

                            Ok(())
                        }
                }
            };
            TokenStream::from(expanded)
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
        _ => {
            dbg!(input);
            TokenStream::from(quote! {})
        }
    }
}
