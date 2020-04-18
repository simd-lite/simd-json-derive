use simd_json_derive::Serialize;

#[test]
fn unnamed1() {
    #[derive(simd_json_derive::Serialize)]
    struct Bla(u8);
    let b = Bla(1);
    println!("{}", b.json_string().unwrap());
    assert_eq!(r#"1"#, b.json_string().unwrap())
}
#[test]
fn unnamed2() {
    #[derive(simd_json_derive::Serialize)]
    struct Bla(u8, u16);
    let b = Bla(1, 2);
    println!("{}", b.json_string().unwrap());
    assert_eq!(r#"[1,2]"#, b.json_string().unwrap())
}

#[test]
fn named() {
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
