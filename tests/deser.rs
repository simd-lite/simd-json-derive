use simd_json_derive::{Deserialize, Serialize};

// #[test]
// fn unnamed1() {
//     #[derive(simd_json_derive::Serialize)]
//     struct Bla(u8);
//     let b = Bla(1);
//     println!("{}", b.json_string().unwrap());
//     assert_eq!(r#"1"#, b.json_string().unwrap())
// }
// #[test]
// fn unnamed2() {
//     #[derive(simd_json_derive::Serialize)]
//     struct Bla(u8, u16);
//     let b = Bla(1, 2);
//     println!("{}", b.json_string().unwrap());
//     assert_eq!(r#"[1,2]"#, b.json_string().unwrap())
// }

#[test]
fn deser() {
    #[derive(simd_json_derive::Serialize, simd_json_derive::Deserialize, PartialEq, Debug)]
    struct Bla {
        f1: Option<u8>,
        f2: String,
    };

    let b = Bla {
        f1: Some(1),
        f2: "snot".into(),
    };
    let mut s = b.json_string().unwrap();
    println!("{}", s);
    assert_eq!(r#"{"f1":1,"f2":"snot"}"#, s);
    let b1 = Bla::from_str(s.as_mut_str()).unwrap();
    assert_eq!(b, b1);
}

#[test]
fn opt() {
    #[derive(simd_json_derive::Deserialize, Debug, PartialEq)]
    struct Bla {
        logo: Option<String>,
        name: String,
    };

    let mut s = String::from(r#"{"name":"snot"}"#);
    let b = Bla::from_str(s.as_mut_str()).unwrap();
    assert_eq!(
        b,
        Bla {
            logo: None,
            name: "snot".into()
        }
    );

    let mut s = String::from(r#"{"name":"snot", "logo": null}"#);
    let b = Bla::from_str(s.as_mut_str()).unwrap();
    assert_eq!(
        b,
        Bla {
            logo: None,
            name: "snot".into()
        }
    );

    let mut s = String::from(r#"{"name":"snot", "logo": "badger"}"#);
    let b = Bla::from_str(s.as_mut_str()).unwrap();
    assert_eq!(
        b,
        Bla {
            logo: Some("badger".into()),
            name: "snot".into()
        }
    );

    let mut s = String::from(r#"{"logo": "badger"}"#);
    assert!(Bla::from_str(s.as_mut_str()).is_err());

    let mut s = String::from(r#"{}"#);
    assert!(Bla::from_str(s.as_mut_str()).is_err());

    let mut s = String::from(r#"{"name":"snot", "logo": 42}"#);
    assert!(Bla::from_str(s.as_mut_str()).is_err());

    let mut s = String::from(r#"{"name":"snot", "logo": "badger", "snot":42}"#);
    assert!(Bla::from_str(s.as_mut_str()).is_err());
}

// #[test]
// fn unnamed1_lifetime() {
//     #[derive(simd_json_derive::Serialize)]
//     struct BlaU1L<'a>(&'a str);
//     let b = BlaU1L("snot");
//     println!("{}", b.json_string().unwrap());
//     assert_eq!(r#""snot""#, b.json_string().unwrap())
// }

// #[test]
// fn unnamed2_lifetime() {
//     #[derive(simd_json_derive::Serialize)]
//     struct BlaU2L<'a, 'b>(&'a str, &'b str);
//     let b = BlaU2L("hello", "world");
//     println!("{}", b.json_string().unwrap());
//     assert_eq!(r#"["hello","world"]"#, b.json_string().unwrap())
// }

// #[test]
// fn named_lifetime() {
//     #[derive(simd_json_derive::Serialize)]
//     struct BlaN2L<'a, 'b> {
//         f1: &'a str,
//         f2: &'b str,
//     };

//     let b = BlaN2L {
//         f1: "snot",
//         f2: "badger",
//     };
//     println!("{}", b.json_string().unwrap());
//     assert_eq!(r#"{"f1":"snot","f2":"badger"}"#, b.json_string().unwrap())
// }
