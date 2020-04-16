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

#[test]
fn enum_stuff() {
    #[derive(simd_json_derive::Serialize, serde::Serialize)]
    enum Bla {
        Blubb,
        Wobbble(u8),
        Wobbble2(u8, u16),
        Gobble { k1: u8 },
    };

    let b = Bla::Blubb;
    println!("{}", serde_json::to_string(&b).unwrap());
    assert_eq!(r#""Blubb""#, b.json_string().unwrap());
    let b = Bla::Wobbble(1);
    println!("{}", serde_json::to_string(&b).unwrap());
    assert_eq!(r#"{"Wobbble":1}"#, b.json_string().unwrap());
    let b = Bla::Wobbble2(1, 2);
    println!("{}", serde_json::to_string(&b).unwrap());
    assert_eq!(r#"{"Wobbble":[1,2]}"#, b.json_string().unwrap());
    let b = Bla::Gobble { k1: 2 };
    println!("{}", serde_json::to_string(&b).unwrap());
    assert_eq!(r#"{"Gobble":{"k1":2}}"#, b.json_string().unwrap());
    assert!(false);
}
