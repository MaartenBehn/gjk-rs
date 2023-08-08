use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use gjk::{colliders::Collider, gjk::GJKNesterov, json_loder::load_test_file};

fn test_gjk(collider1: &Collider, collider2: &Collider) {
    let mut gjk = GJKNesterov::new(None, 1e-6);
    gjk.distance_nesterov_accelerated(collider1, collider2, 100);
}

fn criterion_benchmark(c: &mut Criterion) {

    let path = "../data/nao_test_cases.json";
    let test_data = load_test_file(path);

    c.bench_function("gjk", |b| b.iter(|| 
        for (i, data) in test_data.iter().enumerate() {
            test_gjk(&data.0, &data.1);
        }
    ));

    print!("Cases: {:?}", test_data.len());
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);