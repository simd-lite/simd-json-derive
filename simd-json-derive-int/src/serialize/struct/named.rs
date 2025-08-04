use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Field, Generics};

use crate::args::StructAttrs;

/// Named struct as `Struct(u8)` or `Struct(u8, String)`
pub(crate) fn derive(
    attrs: StructAttrs,
    ident: Ident,
    generics: Generics,
    fields: Punctuated<Field, Comma>,
) -> proc_macro::TokenStream {
    let mut keys = Vec::new();
    let mut values = Vec::new();
    let mut skip_if = Vec::new();

    for f in &fields {
        let ident = f.ident.clone().expect("Missing ident");
        let name = attrs.name(f);
        keys.push(name);
        values.push(ident);
        skip_if.push(attrs.skip_serializing_if(f));
    }
    let expanded = if skip_if.iter().all(Option::is_none) {
        if let Some((first, rest)) = keys.split_first_mut() {
            *first = format!("{{{first}");
            for r in rest {
                *r = format!(",{r}");
            }
        };

        quote! {
            impl #generics simd_json_derive::Serialize for #ident #generics {
                #[inline]
                fn json_write<W>(&self, writer: &mut W) -> std::io::Result<()>
                where
                    W: std::io::Write {
                        #(
                            writer.write_all(#keys.as_bytes())?;
                            self.#values.json_write(writer)?;
                        )*
                        writer.write_all(b"}")
                    }
            }
        }
    } else {
        let writes = keys
            .iter()
            .zip(values.iter())
            .zip(skip_if.iter())
            .map(|((k, v), s)| {
                if let Some(s) = s {
                    quote! {
                        if !#s(&self.#v) {
                            if has_written_key {
                                writer.write_all(b",")?;
                            }
                            has_written_key = true;
                            writer.write_all(#k.as_bytes())?;
                            self.#v.json_write(writer)?;
                        }
                    }
                } else {
                    quote! {
                        if has_written_key {
                            writer.write_all(b",")?;
                        }
                        has_written_key = true;
                        writer.write_all(#k.as_bytes())?;
                        self.#v.json_write(writer)?;
                    }
                }
            })
            .collect::<Vec<_>>();
        quote! {
            impl #generics simd_json_derive::Serialize for #ident #generics {
                #[inline]
                fn json_write<W>(&self, writer: &mut W) -> std::io::Result<()>
                where
                    W: std::io::Write {
                        writer.write_all(b"{")?;
                        let mut has_written_key = false;
                        #(
                            #writes
                        )*
                        writer.write_all(b"}")
                    }
            }
        }
    };
    TokenStream::from(expanded)
}
