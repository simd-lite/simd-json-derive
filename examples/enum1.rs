use simd_json_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum StoredVariants {
    YesNo(bool),
    Small(u8, i8),
    Signy(i64),
    Stringy(String),
}

fn main() {
    let x = StoredVariants::Signy(-1);
    let mut serialized = x.json_string().expect("serialization shouldnt fail :(");
    let deserialized = StoredVariants::from_str(serialized.as_mut_str())
        .expect("serialized stuff should be deserializable");
    println!("Serialized: {x:?}");
    println!("Deserialized: {deserialized:?}");
}
