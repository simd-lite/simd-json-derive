use simd_json_derive::Serialize;

#[test]
fn rename() {
    #[derive(simd_json_derive::Serialize)]
    struct Bla {
        #[serde(rename = "f3")]
        f1: u8,
        #[simd_json(rename = "f4")]
        f2: String,
        f5: u8,
    };

    let b = Bla {
        f1: 1,
        f2: "snot".into(),
        f5: 8,
    };
    println!("{}", b.json_string().unwrap());
    assert_eq!(r#"{"f3":1,"f4":"snot","f5":8}"#, b.json_string().unwrap())
}
