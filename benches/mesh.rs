use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use simd_json::AlignedBuf;
use simd_json_derive::{Deserialize, Serialize};

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

const BUFFER_LEN: usize = 50_000_000;

fn deserialize_mesh(c: &mut Criterion) {
    let mut group = c.benchmark_group("deserialize");
    for i in [2_usize, 32, 128, 1024] {
        let mesh = Mesh {
            triangles: (0..i).map(|_| Default::default()).collect(),
        };
        let input = mesh.json_vec().unwrap();
        group.throughput(Throughput::Bytes(input.len() as u64));

        let mut input_buffer = AlignedBuf::with_capacity(BUFFER_LEN);
        let mut string_buffer = vec![0; BUFFER_LEN];

        group.bench_function(format!("simd({i})"), |b| {
            b.iter_batched_ref(
                || input.clone(),
                |deserialize_buffer| {
                    black_box(
                        Mesh::from_slice_with_buffers(
                            deserialize_buffer.as_mut_slice(),
                            &mut input_buffer,
                            string_buffer.as_mut_slice(),
                        )
                        .unwrap(),
                    );
                },
                BatchSize::SmallInput,
            )
        });
        group.bench_function(format!("serde({i})"), |b| {
            b.iter_batched_ref(
                || input.clone(),
                |deserialize_buffer| {
                    black_box(
                        simd_json::serde::from_slice::<'_, Mesh>(deserialize_buffer.as_mut_slice())
                            .unwrap(),
                    );
                },
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}
fn serialize_mesh(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialize");
    for i in [2_usize, 32, 128, 1024] {
        let mut serialize_buffer = vec![0; BUFFER_LEN];
        let mesh = Mesh {
            triangles: (0..i).map(|_| Default::default()).collect(),
        };
        let input = mesh.json_vec().unwrap();
        group.throughput(Throughput::Bytes(input.len() as u64));

        group.bench_function(format!("simd({i})"), |b| {
            b.iter(|| {
                black_box(&mesh)
                    .json_write(&mut black_box(serialize_buffer.as_mut_slice()))
                    .unwrap();
            })
        });

        group.bench_function(format!("serde({i})"), |b| {
            b.iter(|| {
                simd_json::serde::to_writer(
                    black_box(serialize_buffer.as_mut_slice()),
                    black_box(&mesh),
                )
                .unwrap();
                black_box(());
            })
        });
    }
    group.finish();
}

criterion_group!(benches, serialize_mesh, deserialize_mesh);
criterion_main!(benches);
