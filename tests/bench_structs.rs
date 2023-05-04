use simd_json_derive::{Deserialize, Serialize};

#[test]
fn mesh() {
    #[derive(Serialize, Deserialize, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
    pub struct Vector3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    impl Default for Vector3 {
        fn default() -> Self {
            Self {
                x: rand::random(),
                y: rand::random(),
                z: rand::random(),
            }
        }
    }
    #[derive(
        Serialize, Deserialize, PartialEq, Debug, serde::Serialize, serde::Deserialize, Default,
    )]
    pub struct Triangle {
        pub v0: Vector3,
        pub v1: Vector3,
        pub v2: Vector3,
        pub normal: Vector3,
    }
    #[derive(
        Serialize, Deserialize, PartialEq, Debug, serde::Serialize, serde::Deserialize, Default,
    )]
    pub struct Mesh {
        pub triangles: Vec<Triangle>,
    }

    let m = Mesh {
        triangles: (0..128).map(|_| Triangle::default()).collect(),
    };

    let simd = m.json_string().unwrap();
    let ser = serde_json::to_string(&m).unwrap();
    println!("{simd} == {ser}");
    assert_eq!(simd, ser);
}
