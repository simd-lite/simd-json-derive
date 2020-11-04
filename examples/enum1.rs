#[derive(simd_json_derive::Deserialize)]
pub enum StoredVariants {
    YesNo(bool),
    Small(u8, i8),
    Signy(i64),
    Stringy(String),
}

fn main() {}
