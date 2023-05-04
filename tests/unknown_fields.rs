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
        err.to_string()
            .contains("missing fields: `snot`, `something`"),
        "Err: {} was wrong",
        err
    );

    s = r#"{"snot": [65535, 65536]}"#.to_string();
    let res = SomeFields::from_str(s.as_mut_str());
    assert!(res.is_err());
    let err = res.err().unwrap();
    assert!(
        err.to_string().contains("missing fields: `something`"),
        "Err: {} was wrong",
        err
    );
}
