use criterion::{criterion_group, criterion_main, Criterion};
use logsmith::pipeline::rayon::parse_parallel;

fn bench_parallel_parse(c: &mut Criterion) {
    let lines: Vec<String> = (0..10_000)
        .map(|_| {
            r#"{"timestamp":"2025-11-08T10:00:00Z","level":"INFO","latency_ms":120}"#
                .to_string()
        }).collect();
    c.bench_function("rayon_parse_10k", |b|{
        b.iter(|| parse_parallel(lines.clone()))
    });

    c.bench_function("channel_pipeline_10k", |b| {
        b.iter(|| run_channel_pipeline(&lines))
    });

}

criterion_group(benches, bench_parallel_parse);
criterion_main(benches);
