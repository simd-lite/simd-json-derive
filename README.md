# simd-json-derive

[![Latest version](https://img.shields.io/crates/v/simd-json-derive.svg)](https://crates.io/crates/simd-json-derive)
[![documentation](https://img.shields.io/docsrs/simd-json-derive)](https://docs.rs/simd-json-derive)
![License](https://img.shields.io/crates/l/simd-json-derive.svg)


Derives for high performance JSON serialisation and deserialisation.

## Usage

```rust

#[derive(Serialize, Deserialize, Debug)]
#[simd_json(deny_unknown_fields, rename_all = "camelCase")]
struct MyStruct {
    first_field: String,
    #[simd_json(rename = "foo")]
    second_field: Option<usize>
}

fn main -> Result<(), simd_json::Error> {
    let my_struct = MyStruct {
        first_field: "i am first".to_string(),
        second_field: None
    }
    println!("Before: {my_struct:?}");
    let mut json_string = my_struct.json_string()?;
    let deserialized = MyStruct::from_str(json_string.as_mut_str())?;
    println!("After: {deserialized:?}");
}
```

## Supported Attributes

Attributres are supported for both `#[simd_json(...)]` and for compatibilty also for `#[serde(...)]` and follow the same naming conventions as serde.

For fields:

* `rename = "new_name"` - renames a field

For structs:

* `rename_all = "camelCase"` - renames all (not otherwise renamed) based on the rule, `camelCase` is currently supported
* `deny_unknown_fields` - Errors if unknown fields are encountered

### All Thanks To Our Contributors:
<a href="https://github.com/NightMare-Vortex/simd-json-derive/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=NightMare-Vortex/simd-json-derive" />
</a>
