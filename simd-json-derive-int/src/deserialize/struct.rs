pub(super) mod named;
pub(super) mod unnamed;

use syn::{DataStruct, Fields, FieldsNamed, FieldsUnnamed, Generics, Ident};

use crate::args::StructAttrs;

pub(crate) fn derive(
    attrs: StructAttrs,
    ident: Ident,
    generics: Generics,
    defn: DataStruct,
) -> proc_macro::TokenStream {
    match defn {
        DataStruct {
            fields: Fields::Unnamed(FieldsUnnamed { unnamed, .. }),
            ..
        } => unnamed::derive(attrs, ident, generics, unnamed),
        // Named
        DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        } => named::derive(attrs, ident, generics, named),
        DataStruct {
            fields: Fields::Unit,
            ..
        } => {
            todo!("Unit structs are not implemented")
        }
    }
}
