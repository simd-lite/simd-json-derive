use simd_json_derive::Deserialize;

#[test]
fn opt() {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Ranme {
        logo_name: Option<String>,
        #[serde(rename = "Name")]
        name: String,
    }
}
