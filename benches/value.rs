use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use simd_json::borrowed::Value as BorrowedValue;
use simd_json::owned::Value as OwnedValue;
use simd_json_derive::Deserialize;

fn json_array_string(length: usize) -> String {
    format!(
        "[{}]",
        (0..length)
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn deserialize_owned_value_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("owned-deserialize-array");
    for i in [2_usize, 32, 128, 1024, 4096] {
        let input = json_array_string(i);
        group.throughput(Throughput::Bytes(input.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(i), &input, |b, input| {
            let mut my_input = input.clone();
            b.iter(|| unsafe { OwnedValue::from_str(my_input.as_mut_str()).expect("shizzle") })
        });
    }
    group.finish();
}

fn deserialize_borrowed_value_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("borrowed-deserialize-array");
    for i in [2_usize, 32, 128, 1024, 4096] {
        let input = json_array_string(i);
        group.throughput(Throughput::Bytes(input.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(i), &input, |b, input| {
            let mut my_input = input.clone();
            b.iter(|| {
                let _x =
                    unsafe { BorrowedValue::from_str(my_input.as_mut_str()).expect("shizzle") };
            })
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    deserialize_owned_value_benchmark,
    deserialize_borrowed_value_benchmark
);
criterion_main!(benches);
