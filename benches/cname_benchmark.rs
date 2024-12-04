#![allow(missing_docs)]

use criterion::{criterion_group, criterion_main, Criterion};
use staticdatagen::generators::cname::{CnameConfig, CnameGenerator};

fn benchmark_parallel_batch(c: &mut Criterion) {
    let configs = (0..1_000_000)
        .map(|i| {
            CnameConfig::new(
                format!("example{}.com", i),
                Some(3600),
                None,
            )
            .unwrap()
        })
        .collect::<Vec<_>>();

    _ = c.bench_function("parallel_batch_generate", |b| {
        b.iter(|| CnameGenerator::batch_generate(configs.clone()))
    });
}

criterion_group!(benches, benchmark_parallel_batch);
criterion_main!(benches);
