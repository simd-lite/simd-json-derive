use simd_json_derive::{Deserialize, Serialize};

#[test]
fn deny_unknown_fields() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[simd_json(deny_unknown_fields)]
    struct Strict {
        snot: String,
        badger: f64,
        opt: Option<bool>,
    }

    let mut s = r#"{"snot":"foo", "badger":0.5, "unknown": "bla"}"#.to_string();
    let res = Strict::from_str(s.as_mut_str());
    assert!(res.is_err());
    let err = res.err().unwrap();
    assert!(err
        .to_string()
        .contains("unknown field `unknown`, expected one of `snot`, `badger`, `opt`"));

    let mut s = r#"{"unknown": "bla", "snot":"foo", "badger":0.5}"#.to_string();
    let res = Strict::from_str(s.as_mut_str());
    assert!(res.is_err());
    let err = res.err().unwrap();
    assert!(err
        .to_string()
        .contains("unknown field `unknown`, expected one of `snot`, `badger`, `opt`"));
}

#[test]
fn allow_unknown_fields() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct NonStrict<'de> {
        text: &'de str,
        num: u8,
    }

    let mut s = r#"{"text":"foo", "unknown": [1,2], "num": 3}"#.to_string();
    let res = NonStrict::from_str(s.as_mut_str());
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap(),
        NonStrict {
            text: "foo",
            num: 3
        }
    );
    let mut s = r#"{"unknown": {"snot": "badger"}, "num": 1, "text":"foo"}"#.to_string();
    let res = NonStrict::from_str(s.as_mut_str());
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap(),
        NonStrict {
            text: "foo",
            num: 1
        }
    );
}

#[test]
fn missing_required_fields() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct SomeFields {
        snot: (u16, u32),
        badger: Option<String>,
        something: Vec<i64>,
    }

    let mut s = r#"{}"#.to_string();
    let res = SomeFields::from_str(s.as_mut_str());
    assert!(res.is_err());
    let err = res.err().unwrap();
    assert!(
        err.to_string().contains("missing field: `snot`"),
        "Err: {} was wrong",
        err
    );

    s = r#"{"snot": [65535, 65536]}"#.to_string();
    let res = SomeFields::from_str(s.as_mut_str());
    assert!(res.is_err());
    let err = res.err().unwrap();
    assert!(
        err.to_string().contains("missing field: `something`"),
        "Err: {} was wrong",
        err
    );
}
