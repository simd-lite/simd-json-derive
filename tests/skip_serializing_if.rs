use simd_json_derive::Serialize;

#[test]
fn skip_in_struct() {
    #[derive(Serialize)]
    struct Bla {
        #[serde(skip_serializing_if = "Option::is_none")]
        f1: Option<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        f2: Option<u8>,
        f3: Option<u8>,
    }

    let b = Bla {
        f1: None,
        f2: None,
        f3: None,
    };
    let s = b.json_string().unwrap();
    assert_eq!(r#"{"f3":null}"#, s);

    let b = Bla {
        f1: Some(1),
        f2: None,
        f3: None,
    };
    let s = b.json_string().unwrap();
    assert_eq!(r#"{"f1":1,"f3":null}"#, s);

    let b = Bla {
        f1: None,
        f2: Some(2),
        f3: None,
    };
    let s = b.json_string().unwrap();
    assert_eq!(r#"{"f2":2,"f3":null}"#, s);

    let b = Bla {
        f1: Some(1),
        f2: None,
        f3: Some(3),
    };
    let s = b.json_string().unwrap();
    assert_eq!(r#"{"f1":1,"f3":3}"#, s);
}

#[test]
fn skip_in_enum() {
    #[derive(Serialize)]
    enum Bla {
        Blubb {
            #[serde(skip_serializing_if = "Option::is_none")]
            f1: Option<u8>,
            #[serde(skip_serializing_if = "Option::is_none")]
            f2: Option<u8>,
            f3: Option<u8>,
        },
        Blargh {
            #[serde(skip_serializing_if = "Option::is_none")]
            f1: Option<u8>,
            #[serde(skip_serializing_if = "Option::is_none")]
            f2: Option<u8>,
            f3: Option<u8>,
        },
    }

    let b = Bla::Blubb {
        f1: None,
        f2: None,
        f3: None,
    };
    let s = b.json_string().unwrap();
    assert_eq!(r#"{"Blubb":{"f3":null}}"#, s);

    let b = Bla::Blubb {
        f1: Some(1),
        f2: None,
        f3: None,
    };
    let s = b.json_string().unwrap();
    assert_eq!(r#"{"Blubb":{"f1":1,"f3":null}}"#, s);

    let b = Bla::Blargh {
        f1: None,
        f2: Some(2),
        f3: None,
    };
    let s = b.json_string().unwrap();
    assert_eq!(r#"{"Blargh":{"f2":2,"f3":null}}"#, s);

    let b = Bla::Blargh {
        f1: Some(1),
        f2: None,
        f3: Some(3),
    };
    let s = b.json_string().unwrap();
    assert_eq!(r#"{"Blargh":{"f1":1,"f3":3}}"#, s);
}
