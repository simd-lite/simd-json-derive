use simd_json_derive::{Deserialize, Serialize};

#[test]
fn owned_value() {
    let input = r#"{"snot":["badger",true,false,12.5,null,{"inner":[{}]},[[]]]}"#.to_string();
    let value = simd_json::owned::to_value(unsafe { input.clone().as_bytes_mut() })
        .expect("Expected the literal to work");
    let res = value.json_string();
    assert!(res.is_ok());
    let mut serialized = res.ok().unwrap();
    assert_eq!(input, serialized);
    let deserialized = unsafe { simd_json::owned::Value::from_str(serialized.as_mut_str()) }
        .expect("Expected serialized input to be deserialized ok");
    println!("{deserialized}");
    assert_eq!(value, deserialized);
}
