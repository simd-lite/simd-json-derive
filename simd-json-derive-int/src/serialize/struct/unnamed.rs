use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::token::Comma;
use syn::{punctuated::Punctuated, Field, Generics};

use crate::args::StructAttrs;

/// Unnamed struct as `Struct(u8)` or `Struct(u8, String)`
pub(crate) fn derive(
    _attrs: StructAttrs,
    ident: Ident,
    generics: Generics,
    fields: Punctuated<Field, Comma>,
) -> proc_macro::TokenStream {
    if fields.len() == 1 {
        let expanded = quote! {
            impl #generics simd_json_derive::Serialize for #ident #generics {
                fn json_write<W>(&self, writer: &mut W) -> std::io::Result<()>
                where
                    W: std::io::Write {
                        self.0.json_write(writer)
                    }
            }
        };
        TokenStream::from(expanded)
    } else {
        let keys: Vec<_> = fields
            .iter()
            .enumerate()
            .map(|(i, _)| syn::Index::from(i))
            .skip(1)
            .collect();
        let expanded = quote! {
            impl #generics simd_json_derive::Serialize for #ident #generics {
                fn json_write<W>(&self, writer: &mut W) -> std::io::Result<()>
                where
                    W: std::io::Write {
                        writer.write_all(b"[")?;
                        self.0.json_write(writer)?;
                        #(
                            writer.write_all(b",")?;
                            self.#keys.json_write(writer)?;
                        )*
                        writer.write_all(b"]")
                    }
            }
        };
        TokenStream::from(expanded)
    }
}
