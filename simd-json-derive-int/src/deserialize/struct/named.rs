use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated, token::Comma, Field, GenericParam, Generics, Ident, Path, PathSegment,
    Type, TypePath,
};

use crate::args::StructAttrs;

/// Named struct as `Struct(u8)` or `Struct(u8, String)`
pub(crate) fn derive(
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
            fn from_tape(__deser_tape: &mut ::simd_json_derive::Tape <#derive_lt>) -> ::simd_json_derive::de::Result<Self>
            where
                Self: std::marker::Sized + #derive_lt
            {
                let __deser_len: usize = if let Some(::simd_json::Node::Object{len, ..}) = __deser_tape.next() {
                    len
                } else {
                    return Err(::simd_json_derive::de::Error::InvalidStructRepresentation);
                };

                #(let mut #value_locals = None;)*
                #(let mut #option_locals = None;)*

                for _ in 0..__deser_len {
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
                                    return Err(::simd_json_derive::de::Error::UnknownField {
                                        unknown_field: __unknown_field.to_string(),
                                        possible_field_names: &[ #(#value_keys,)* #(#option_keys,)* ]
                                    });
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
                            #values: #value_locals.ok_or_else(|| ::simd_json_derive::de::Error::MissingField(#value_keys))?,
                        )*
                })
            }
        }
    };
    TokenStream::from(expanded)
}
