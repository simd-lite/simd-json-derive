use proc_macro2::{Ident, Literal};
use simd_json::prelude::*;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Token, Field};

pub(crate) struct FieldAttrs {
    rename: Option<String>,
}

impl Parse for FieldAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut rename = None;
        while !input.is_empty() {
            let attr: Ident = input.parse()?;
            match attr.to_string().as_str() {
                "rename" => {
                    let _eqal_token: Token![=] = input.parse()?;
                    let name: Literal = input.parse()?;

                    rename = Some(name.to_string().trim_matches('"').to_string());
                }
                other => {
                    return Err(syn::Error::new(
                        attr.span(),
                        format!("unexpected attribute `{}`", other),
                    ))
                }
            }
            if !input.is_empty() {
                let _comma_token: Token![,] = input.parse()?;
            }
        }
        Ok(FieldAttrs { rename })
    }
}

pub(crate) enum RenameAll {
    None,
    CamelCase,
}

fn capitalize(field: &str) -> String {
    let mut chars = field.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

impl RenameAll {
    fn apply(&self, field: &str) -> String {
        match self {
            RenameAll::None => String::from(field),
            RenameAll::CamelCase => {
                let mut parts = field.split('_');
                let first = parts.next().unwrap();
                format!(
                    "{}{}",
                    first,
                    parts.map(capitalize).collect::<Vec<String>>().join("")
                )
            }
        }
    }
}

pub(crate) struct StructAttrs {
    rename_all: RenameAll,
}

impl Default for StructAttrs {
    fn default() -> Self {
        StructAttrs {
            rename_all: RenameAll::None,
        }
    }
}

impl Parse for StructAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut rename_all = RenameAll::None;

        while !input.is_empty() {
            let attr: Ident = input.parse()?;
            match attr.to_string().as_str() {
                "rename_all" => {
                    let _eqal_token: Token![=] = input.parse()?;
                    let name: Literal = input.parse()?;

                    match name.to_string().as_str() {
                        r#""camelCase""# => rename_all = RenameAll::CamelCase,
                        other => {
                            return Err(syn::Error::new(
                                attr.span(),
                                format!("unexpected rename_all type `{}`", other),
                            ))
                        }
                    }
                }
                other => {
                    return Err(syn::Error::new(
                        attr.span(),
                        format!("unexpected field attribute `{}`", other),
                    ))
                }
            }
        }
        Ok(StructAttrs { rename_all })
    }
}

pub(crate) fn field_attrs(attr: &Attribute) -> FieldAttrs {
    attr.parse_args::<FieldAttrs>()
        .expect("failed to parse attributes")
}

pub(crate) fn struct_attrs(attr: &Attribute) -> StructAttrs {
    attr.parse_args::<StructAttrs>()
        .expect("failed to parse attributes")
}

pub(crate) fn get_attr<'field>(
    attrs: &'field [Attribute],
    name: &str,
) -> Option<&'field Attribute> {
    attrs
        .iter()
        .filter(|a| a.path.get_ident().map(|i| i == name).unwrap_or_default())
        .next()
}

pub(crate) fn name(struct_attr: &StructAttrs, field: &Field) -> Option<String> {
    if let Some(attr) = get_attr(&field.attrs, "simd_json")
        .map(field_attrs)
        .and_then(|a| a.rename)
    {
        Some(format!("{}:", simd_json::OwnedValue::from(attr).encode()))
    } else if let Some(attr) = get_attr(&field.attrs, "serde")
        .map(field_attrs)
        .and_then(|a| a.rename)
    {
        Some(format!("{}:", simd_json::OwnedValue::from(attr).encode()))
    } else {
        field.ident.as_ref().map(|ident| {
            format!(
                "{}:",
                simd_json::OwnedValue::from(struct_attr.rename_all.apply(&ident.to_string()))
                    .encode()
            )
        })
    }
}
