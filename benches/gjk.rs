use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use gjk::{colliders::Collider, gjk::GJKNesterov, json_loder::load_test_file};

fn test_gjk(collider1: &Collider, collider2: &Collider) {
    let mut gjk = GJKNesterov::new(None, 1e-6);
    gjk.intersect_nesterov_accelerated(collider1, collider2, 100);
}

fn criterion_benchmark(c: &mut Criterion) {

    let path = "../data/test_data.json";
    let test_data = load_test_file(path);

    let mut group = c.benchmark_group("test_gjk");

    for (i, data) in test_data.iter().enumerate() {
        group.bench_with_input(BenchmarkId::from_parameter(i), data, |b, data| b.iter(|| test_gjk(&data.0, &data.1)));
        
        if i == 10 {
            break;
        }
    
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);