use criterion::{criterion_group, criterion_main, Criterion};
use day_04::{part_one, part_two};
use std::fs;

fn bench(c: &mut Criterion) {
    c.bench_function("bench_day_04_part1", |b| {
        b.iter(|| {
            let input = fs::read_to_string("input.txt").unwrap();
            part_one(&input)
        })
    });

    c.bench_function("bench_day_04_part2", |b| {
        b.iter(|| {
            let input = fs::read_to_string("input.txt").unwrap();
            part_two(&input)
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);