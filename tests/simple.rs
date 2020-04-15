#[test]
fn unnamed() {
    #[derive(simd_json_derive::Serialize)]
    struct Bla(u8);
    assert!(false)
}
#[test]
fn named() {
    use simd_json_derive::Serialize;
    #[derive(simd_json_derive::Serialize)]
    struct Bla {
        f1: u8,
        f2: String,
    };

    let b = Bla {
        f1: 1,
        f2: "snot".into(),
    };
    println!("{}", b.json_string().unwrap());
    assert_eq!(r#"{"f1":1,"f2":"snot"}"#, b.json_string().unwrap())
}
