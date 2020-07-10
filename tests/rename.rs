use simd_json_derive::{Deserialize, Serialize};

#[test]
fn opt() {
    #[derive(simd_json_derive::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Ranme {
        logo_name: Option<String>,
        #[serde(rename = "Name")]
        name: String,
    };
}