use simd_json_derive::{Deserialize, Serialize};

// #[test]
// fn unnamed1() {
//     #[derive(Serialize)]
//     struct Bla(u8);
//     let b = Bla(1);
//     println!("{}", b.json_string().unwrap());
//     assert_eq!(r#"1"#, b.json_string().unwrap())
// }
// #[test]
// fn unnamed2() {
//     #[derive(Serialize)]
//     struct Bla(u8, u16);
//     let b = Bla(1, 2);
//     println!("{}", b.json_string().unwrap());
//     assert_eq!(r#"[1,2]"#, b.json_string().unwrap())
// }

#[test]
fn deser() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum SnotBadger {
        Snot,
        Badger,
    }
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Bla {
        f1: Option<u8>,
        f2: String,
        f3: SnotBadger,
    }

    let b = Bla {
        f1: Some(1),
        f2: "snot".into(),
        f3: SnotBadger::Snot,
    };
    let mut s = b.json_string().unwrap();
    println!("{}", s);
    assert_eq!(r#"{"f1":1,"f2":"snot","f3":"Snot"}"#, s);
    let b1 = Bla::from_str(s.as_mut_str()).unwrap();
    assert_eq!(b, b1);
}

#[test]
fn opt() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct MyString(String);
    #[derive(Deserialize, Debug, PartialEq)]
    struct Bla {
        logo: Option<String>,
        name: MyString,
    }

    let mut s = String::from(r#"{"name":"snot"}"#);
    let b = Bla::from_str(s.as_mut_str()).unwrap();
    assert_eq!(
        b,
        Bla {
            logo: None,
            name: MyString("snot".into())
        }
    );

    let mut s = String::from(r#"{"name":"snot", "logo": null}"#);
    let b = Bla::from_str(s.as_mut_str()).unwrap();
    assert_eq!(
        b,
        Bla {
            logo: None,
            name: MyString("snot".into())
        }
    );

    let mut s = String::from(r#"{"name":"snot", "logo": "badger"}"#);
    let b = Bla::from_str(s.as_mut_str()).unwrap();
    assert_eq!(
        b,
        Bla {
            logo: Some("badger".into()),
            name: MyString("snot".into())
        }
    );

    let mut s = String::from(r#"{"logo": "badger"}"#);
    assert!(Bla::from_str(s.as_mut_str()).is_err());

    let mut s = String::from(r#"{}"#);
    assert!(Bla::from_str(s.as_mut_str()).is_err());

    let mut s = String::from(r#"{"name":"snot", "logo": 42}"#);
    assert!(Bla::from_str(s.as_mut_str()).is_err());

    let mut s = String::from(r#"{"name":"snot", "logo": "badger", "snot":42}"#);
    assert_eq!(
        Bla {
            name: MyString("snot".to_string()),
            logo: Some("badger".to_string())
        },
        Bla::from_str(s.as_mut_str()).expect("Didn't ignore unknown field 'snot'")
    );
}
#[test]
fn event() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct Ids(Vec<(u64, u64)>);

    #[derive(Deserialize, Debug, PartialEq)]
    struct Event {
        id: Ids,
    }

    let mut s = String::from(r#"{"id":[[0,0]]}"#);
    let e = Event::from_str(s.as_mut_str()).unwrap();

    assert_eq!(
        e,
        Event {
            id: Ids(vec![(0, 0)]),
        }
    );
}

#[test]
fn enum_ser() {
    #[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
    pub enum StoredVariants {
        YesNo(bool),
        Small(u8, i8),
        Signy(i64),
        Stringy(String),
        Res(Result<u8, String>),
    }

    let mut s = String::from(r#"{"Small":[1,2]}"#);
    let e = StoredVariants::from_str(s.as_mut_str()).unwrap();
    assert_eq!(StoredVariants::Small(1, 2), e);

    let mut s = String::from(r#"{"YesNo":true}"#);
    let e = StoredVariants::from_str(s.as_mut_str()).unwrap();
    assert_eq!(StoredVariants::YesNo(true), e);

    let mut s = String::from(r#"{"Res":{"Ok":42}}"#);
    let e = StoredVariants::from_str(s.as_mut_str()).unwrap();
    assert_eq!(StoredVariants::Res(Ok(42)), e);

    let mut s = String::from(r#"{"Res":{"Err":"snot"}}"#);
    let e = StoredVariants::from_str(s.as_mut_str()).unwrap();
    assert_eq!(StoredVariants::Res(Err(String::from("snot"))), e);

    let e = StoredVariants::Res(Ok(42)).json_string().unwrap();
    assert_eq!(r#"{"Res":{"Ok":42}}"#, e);

    let e = StoredVariants::Res(Err(String::from("snot")))
        .json_string()
        .unwrap();
    assert_eq!(r#"{"Res":{"Err":"snot"}}"#, e);
}

#[test]
fn bar_string() {
    #[derive(Deserialize)]
    struct FooString {
        bar: String,
    }

    let mut json = br#"{"bar":"baz"}"#.to_vec();
    let res = FooString::from_slice(&mut json);
    assert!(res.is_ok());
    assert_eq!(res.unwrap().bar, "baz");
}

#[test]
fn foo_str() {
    #[derive(Deserialize)]
    struct FooStr<'de> {
        foo: &'de str,
    }
    let mut json = br#"{"foo":"bar"}"#.to_vec();
    let res = FooStr::from_slice(&mut json);
    assert!(res.is_ok());
    assert_eq!(res.unwrap().foo, "bar");
}
