use criterion::{criterion_group, criterion_main, Criterion};
use idgenerator::{IdGeneratorOptions, IdHelper};

fn id_generator(c: &mut Criterion) {
    let worker_id: u32 = 1;
    IdHelper::init();
    let mut options: IdGeneratorOptions = IdGeneratorOptions::new(worker_id);
    options.worker_id_bit_len = 8;
    IdHelper::set_id_generator(options);
    let mut group = c.benchmark_group("id_generator");
    group.bench_function("id", |b| b.iter(|| idgenerator::IdHelper::next_id()));
    group.finish();
}

criterion_group!(benches, id_generator);
criterion_main!(benches);
