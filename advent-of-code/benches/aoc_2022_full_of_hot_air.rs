use advent_of_code::{
    aoc_2022::full_of_hot_air::{OptimizedDecryption, Snafu},
    Naive,
};
use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

pub fn benchmark_aoc_2022_full_of_hot_air(c: &mut Criterion) {
    let mut group = c.benchmark_group("aoc_2022_full_of_hot_air - Profiling Decryption");

    let mut rng = rand::thread_rng();

    group.bench_with_input("Naive Flag", &(rng.gen::<u32>() as i64), |b, &message| {
        b.iter(|| i64::from(Snafu::<Naive>::from(message)))
    });

    group.bench_with_input(
        "Optimized Flag",
        &(rng.gen::<u32>() as i64),
        |b, &message| b.iter(|| i64::from(Snafu::<OptimizedDecryption>::from(message))),
    );

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .significance_level(0.1)
        .sample_size(500)
        .confidence_level(0.85);
    targets = benchmark_aoc_2022_full_of_hot_air
}
criterion_main!(benches);
