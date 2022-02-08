use criterion::{criterion_group, criterion_main, Criterion};
use idgenerator::*;

fn id_generator(c: &mut Criterion) {
    let options = IdGeneratorOptions::new()
        .worker_id(1)
        .worker_id_bit_len(6)
        .seq_bit_len(12);
    let _ = IdInstance::init(options).unwrap();
    let mut group = c.benchmark_group("id-generator");
    group.bench_function("id", |b| b.iter(|| IdInstance::next_id()));
    group.finish();
}

criterion_group!(benches, id_generator);
criterion_main!(benches);
