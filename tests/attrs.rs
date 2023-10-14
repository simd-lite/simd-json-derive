use simd_json_derive::{Deserialize, Serialize};

#[test]
fn rename() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Bla<'f3> {
        #[serde(rename = "f3")]
        f1: u8,
        #[simd_json(rename = "f4")]
        f2: String,
        #[serde(borrow, rename = "f1")]
        f3: &'f3 str,
        f5: u8,
    }

    let b = Bla {
        f1: 1,
        f2: "snot".into(),
        f3: "snot",
        f5: 8,
    };
    let mut serialized = b.json_string().unwrap();
    println!("{}", &serialized);

    assert_eq!(r#"{"f3":1,"f4":"snot","f1":"snot","f5":8}"#, serialized);
    let b1 = Bla::from_str(&mut serialized).expect("Expected serde roundtrip with rename to work");
    println!("{:?}", &b1);
    assert_eq!(b, b1);
}

#[test]
fn rename_all_camelcase() {
    #[derive(simd_json_derive::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Bla {
        field_one: u8,
        field_two: String,
        #[serde(rename = "f3")]
        field_three: u8,
    }

    let b = Bla {
        field_one: 1,
        field_two: "snot".into(),
        field_three: 8,
    };
    println!("{}", b.json_string().unwrap());
    assert_eq!(
        r#"{"fieldOne":1,"fieldTwo":"snot","f3":8}"#,
        b.json_string().unwrap()
    )
}
