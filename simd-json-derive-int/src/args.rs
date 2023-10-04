use proc_macro2::{Ident, Literal};
use simd_json::prelude::*;
use simd_json::OwnedValue;
use syn::{
    parse::{Parse, ParseStream},
    LitStr, Path,
};
use syn::{Attribute, Field, Token};

#[derive(Debug, Default)]
pub(crate) struct FieldAttrs {
    rename: Option<String>,
    skip_serializing_if: Option<Path>,
}

impl Parse for FieldAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = FieldAttrs::default();

        while !input.is_empty() {
            let attr: Ident = input.parse()?;
            match attr.to_string().as_str() {
                "rename" => {
                    let _eqal_token: Token![=] = input.parse()?;
                    let name: LitStr = input.parse()?;

                    attrs.rename = Some(name.value());
                }
                "skip_serializing_if" => {
                    let _eqal_token: Token![=] = input.parse()?;
                    let function: LitStr = input.parse()?;

                    let path: Path = function.parse()?;

                    attrs.skip_serializing_if = Some(path);
                }
                "borrow" => (),
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
        Ok(attrs)
    }
}

#[derive(Debug)]
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
                let first = parts.next().expect("zero length name");
                format!(
                    "{}{}",
                    first,
                    parts.map(capitalize).collect::<Vec<String>>().join("")
                )
            }
        }
    }
}
#[derive(Debug)]
pub(crate) struct StructAttrs {
    rename_all: RenameAll,
    deny_unknown_fields: bool,
}

impl Default for StructAttrs {
    fn default() -> Self {
        StructAttrs {
            rename_all: RenameAll::None,
            deny_unknown_fields: false,
        }
    }
}

impl Parse for StructAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut rename_all = RenameAll::None;
        let mut deny_unknown_fields = false;
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
                "deny_unknown_fields" => {
                    deny_unknown_fields = true;
                }
                other => {
                    return Err(syn::Error::new(
                        attr.span(),
                        format!("unexpected field attribute `{}`", other),
                    ))
                }
            }
            if !input.is_empty() {
                let _comma_token: Token![,] = input.parse()?;
            }
        }
        Ok(StructAttrs {
            rename_all,
            deny_unknown_fields,
        })
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
        .find(|a| a.path().get_ident().map(|i| i == name).unwrap_or_default())
}

impl StructAttrs {
    pub(crate) fn deny_unknown_fields(&self) -> bool {
        self.deny_unknown_fields
    }

    pub(crate) fn skip_serializing_if(&self, field: &Field) -> Option<Path> {
        get_attr(&field.attrs, "simd_json")
            .or_else(|| get_attr(&field.attrs, "serde"))
            .map(field_attrs)
            .and_then(|a| a.skip_serializing_if)
    }
    pub(crate) fn name(&self, field: &Field) -> String {
        if let Some(attr) = get_attr(&field.attrs, "simd_json")
            .map(field_attrs)
            .and_then(|a| a.rename)
        {
            format!("{}:", OwnedValue::from(attr).encode())
        } else if let Some(attr) = get_attr(&field.attrs, "serde")
            .map(field_attrs)
            .and_then(|a| a.rename)
        {
            format!("{}:", OwnedValue::from(attr).encode())
        } else {
            let f = field
                .ident
                .as_ref()
                .expect("Field is missing ident")
                .to_string();
            format!("{}:", OwnedValue::from(self.rename_all.apply(&f)).encode())
        }
    }
}
