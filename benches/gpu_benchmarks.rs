
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_gpu_matmul(c: &mut Criterion) {
    c.bench_function("gpu_matmul_placeholder", |b| {
        b.iter(|| {

        })
    });
}

criterion_group!(benches, benchmark_gpu_matmul);
criterion_main!(benches);
