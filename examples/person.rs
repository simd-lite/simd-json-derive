#![allow(warnings)]

use serde::Deserialize;
use serde_json;
use simd_json::Buffers;
use simd_json_derive::Deserialize as SimdDeserialize;
use std::time::Instant;

#[derive(Deserialize, SimdDeserialize)]
struct Person {
    id: String,
    index: i32,
    guid: String,
    isActive: bool,
    picture: String,
    age: u32,
}

#[derive(Deserialize, SimdDeserialize)]
struct PersonBorrowed<'ser> {
    #[serde(borrow)]
    id: &'ser str,
    index: i32,
    #[serde(borrow)]
    guid: &'ser str,
    isActive: bool,
    #[serde(borrow)]
    picture: &'ser str,
    age: u32,
}

const N: usize = 100000;

fn main() {
    let json_bytes = br#"{
        "id": "60a6965e5e47ef8456878326",
        "index": 0,
        "guid": "cfce331d-07f3-40d3-b3d9-0672f651c26d",
        "isActive": true,
        "picture": "http://placehold.it/32x32",
        "age": 22
    }"#
    .to_vec();

    let mut json_bytes_2 = json_bytes.clone();
    let now_2 = Instant::now();
    for _ in 0..N {
        let p2: simd_json::OwnedValue = simd_json::to_owned_value(&mut json_bytes_2).unwrap();
    }
    println!("simd_json {:?}", now_2.elapsed());

    let mut json_bytes_2 = json_bytes.clone();
    let now_2 = Instant::now();
    for _ in 0..N {
        let p2: Person = simd_json::serde::from_slice(&mut json_bytes_2).unwrap();
        criterion::black_box(p2);
    }
    println!("simd_json (struct) {:?}", now_2.elapsed());

    let mut json_bytes_2 = json_bytes.clone();
    let now_2 = Instant::now();
    for _ in 0..N {
        let p2 = Person::from_slice(&mut json_bytes_2).unwrap();
        criterion::black_box(p2);
    }
    println!("simd_json (simd-struct) {:?}", now_2.elapsed());

    let mut json_bytes_2 = json_bytes.clone();
    let now_2 = Instant::now();
    for _ in 0..N {
        let p2 = PersonBorrowed::from_slice(&mut json_bytes_2).unwrap();
        criterion::black_box(p2);
    }
    println!("simd_json (simd-struct borrowed) {:?}", now_2.elapsed());

    let mut json_bytes_2 = json_bytes.clone();
    let now_2 = Instant::now();
    let mut buffers = Buffers::new(2048);
    for _ in 0..N {
        let p2 = PersonBorrowed::from_slice_with_buffers(&mut json_bytes_2, &mut buffers).unwrap();
        criterion::black_box(p2);
    }
    println!(
        "simd_json (simd-struct borrowed buffered) {:?}",
        now_2.elapsed()
    );

    let mut json_bytes_1 = json_bytes.clone();
    let now_1 = Instant::now();
    for _ in 0..N {
        let p: Person = serde_json::from_slice(&json_bytes_1).unwrap();
        criterion::black_box(p);
    }
    println!("serde {:?}", now_1.elapsed());

    let mut json_bytes_1 = json_bytes.clone();
    let now_1 = Instant::now();
    for _ in 0..N {
        let p: PersonBorrowed = serde_json::from_slice(&json_bytes_1).unwrap();
        criterion::black_box(p);
    }
    println!("serde (borrowed) {:?}", now_1.elapsed());
}
