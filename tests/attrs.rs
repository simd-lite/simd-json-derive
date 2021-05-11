use simd_json_derive::{Deserialize, Serialize};

#[test]
fn rename() {
    #[derive(Serialize, Deserialize)]
    struct Bla<'f3> {
        #[serde(rename = "f3")]
        f1: u8,
        #[simd_json(rename = "f4")]
        f2: String,
        #[serde(borrow)]
        f3: &'f3 str,
        f5: u8,
    }

    let b = Bla {
        f1: 1,
        f2: "snot".into(),
        f3: "snot",
        f5: 8,
    };
    println!("{}", b.json_string().unwrap());
    assert_eq!(
        r#"{"f3":1,"f4":"snot","f3":"snot","f5":8}"#,
        b.json_string().unwrap()
    )
}

#[test]
fn rename_all() {
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
