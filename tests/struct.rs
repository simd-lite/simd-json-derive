use simd_json_derive::{Deserialize, Serialize};

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
    #[derive(simd_json_derive::Serialize, simd_json_derive::Deserialize, PartialEq, Debug)]
    struct Bla {
        f1: u8,
        f2: String,
    }

    let b = Bla {
        f1: 1,
        f2: "snot".into(),
    };
    let mut s = b.json_string().unwrap();
    println!("{}", s);
    assert_eq!(r#"{"f1":1,"f2":"snot"}"#, s);
    let b1 = unsafe { Bla::from_str(s.as_mut_str()) }.unwrap();
    assert_eq!(b, b1);
}

#[test]
fn unnamed1_lifetime() {
    #[derive(simd_json_derive::Serialize)]
    struct BlaU1L<'a>(&'a str);
    let b = BlaU1L("snot");
    println!("{}", b.json_string().unwrap());
    assert_eq!(r#""snot""#, b.json_string().unwrap())
}

#[test]
fn unnamed2_lifetime() {
    #[derive(simd_json_derive::Serialize)]
    struct BlaU2L<'a, 'b>(&'a str, &'b str);
    let b = BlaU2L("hello", "world");
    println!("{}", b.json_string().unwrap());
    assert_eq!(r#"["hello","world"]"#, b.json_string().unwrap())
}

#[test]
fn named_lifetime() {
    #[derive(simd_json_derive::Serialize)]
    struct BlaN2L<'a, 'b> {
        f1: &'a str,
        f2: &'b str,
    }

    let b = BlaN2L {
        f1: "snot",
        f2: "badger",
    };
    println!("{}", b.json_string().unwrap());
    assert_eq!(r#"{"f1":"snot","f2":"badger"}"#, b.json_string().unwrap())
}

#[test]
fn borrowed() {
    #[derive(simd_json_derive::Serialize, simd_json_derive::Deserialize, PartialEq, Debug)]
    struct SIMDExample<'sin> {
        id: u64,
        #[serde(borrow)]
        id_str: &'sin str,
    }
    let mut s = r#"{"id":23,"id_str":"42"}"#.to_string();
    unsafe {
        assert_eq!(
            SIMDExample {
                id: 23,
                id_str: "42"
            },
            SIMDExample::from_str(s.as_mut_str()).unwrap()
        );
    }
}

#[test]
fn tpl_array() {
    #[derive(simd_json_derive::Serialize, simd_json_derive::Deserialize, PartialEq, Debug)]
    struct Bla {
        tpl: (u8, u8),
        array: [u8; 2],
    }
    let b = Bla {
        tpl: (1, 2),
        array: [3, 4],
    };
    println!("{}", b.json_string().unwrap());
    let mut s = r#"{"tpl":[1,2],"array":[3,4]}"#.to_string();
    assert_eq!(s, b.json_string().unwrap());
    unsafe {
        assert_eq!(b, Bla::from_str(s.as_mut_str()).unwrap());
    }
}
