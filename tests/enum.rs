use simd_json_derive::Serialize;

#[test]
fn enum_stuff_01() {
    #[derive(simd_json_derive::Serialize)]
    enum Bla {
        Blubb,
        Wobbble(u8),
        Wobbble2(u8, u16),
        Gobble { k1: u8, k2: u16 },
    }

    let b = Bla::Blubb;
    assert_eq!(r#""Blubb""#, b.json_string().unwrap());
    let b = Bla::Wobbble(1);
    assert_eq!(r#"{"Wobbble":1}"#, b.json_string().unwrap());
    let b = Bla::Wobbble2(1, 2);
    assert_eq!(r#"{"Wobbble2":[1,2]}"#, b.json_string().unwrap());
    let b = Bla::Gobble { k1: 2, k2: 3 };
    assert_eq!(r#"{"Gobble":{"k1":2,"k2":3}}"#, b.json_string().unwrap());
}

#[test]
fn enum_stuff_02() {
    #[derive(simd_json_derive::Serialize)]
    enum Bla {
        Blubb,
        Wobble,
    }

    let b = Bla::Blubb;
    assert_eq!(r#""Blubb""#, b.json_string().unwrap());
    let b = Bla::Wobble;
    assert_eq!(r#""Wobble""#, b.json_string().unwrap());
}

#[test]
fn enum_stuff_01_lifeimte() {
    #[derive(simd_json_derive::Serialize)]
    enum Bla<'a, 'b> {
        Blubb,
        Wobbble(&'a str),
        Wobbble2(&'a str, &'b str),
        Gobble { k1: u8, k2: u16 },
    }

    let b = Bla::Blubb;
    assert_eq!(r#""Blubb""#, b.json_string().unwrap());
    let b = Bla::Wobbble("snot");
    assert_eq!(r#"{"Wobbble":"snot"}"#, b.json_string().unwrap());
    let b = Bla::Wobbble2("snot", "badger");
    assert_eq!(
        r#"{"Wobbble2":["snot","badger"]}"#,
        b.json_string().unwrap()
    );
    let b = Bla::Gobble { k1: 2, k2: 3 };
    assert_eq!(r#"{"Gobble":{"k1":2,"k2":3}}"#, b.json_string().unwrap());
}

#[test]
fn enum_ser() {
    #[derive(Serialize)]
    pub enum StoredVariants {
        YesNo(bool),
        Small(u8, i8),
    }

    let d = StoredVariants::YesNo(true);
    assert_eq!(r#"{"YesNo":true}"#, d.json_string().unwrap());

    let d = StoredVariants::Small(1, 2);
    assert_eq!(r#"{"Small":[1,2]}"#, d.json_string().unwrap());

    // let e = StoredVariants::from_str(s.as_mut_str()).unwrap();
}
