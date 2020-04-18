use simd_json_derive::Serialize;

#[test]
fn enum_stuff() {
    #[derive(simd_json_derive::Serialize)]
    enum Bla {
        Blubb,
        Wobbble(u8),
        Wobbble2(u8, u16),
        Gobble { k1: u8, k2: u16 },
    };

    let b = Bla::Blubb;
    assert_eq!(r#""Blubb""#, b.json_string().unwrap());
    let b = Bla::Wobbble(1);
    assert_eq!(r#"{"Wobbble":1}"#, b.json_string().unwrap());
    let b = Bla::Wobbble2(1, 2);
    assert_eq!(r#"{"Wobbble2":[1,2]}"#, b.json_string().unwrap());
    let b = Bla::Gobble { k1: 2, k2: 3 };
    assert_eq!(r#"{"Gobble":{"k1":2,"k2":3}}"#, b.json_string().unwrap());
}
