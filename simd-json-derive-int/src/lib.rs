use proc_macro::TokenStream;

mod args;
mod deserialize;
mod serialize;

#[proc_macro_derive(Serialize, attributes(serde, simd_json))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    serialize::derive(input)
}

#[proc_macro_derive(Deserialize, attributes(serde, simd_json))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    deserialize::derive(input)
}
