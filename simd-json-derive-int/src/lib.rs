use proc_macro::TokenStream;

mod args;
mod serialize;

#[proc_macro_derive(Serialize, attributes(serde, simd_json))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    serialize::derive(input)
}
