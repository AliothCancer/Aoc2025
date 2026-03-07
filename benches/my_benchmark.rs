use std::fs::read_to_string;

use aoc2025::{get_max_part2, get_max_part2_usize};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

criterion_group!(benches, aoc2025_d3_part1, aoc2025_d3_part2);
criterion_main!(benches);
fn aoc2025_d3_part2(c: &mut Criterion) {
    let mut lines = Vec::with_capacity(1000);
    let file = read_to_string("./src/input.txt").unwrap();
    file.lines()
        .enumerate()
        .for_each(|(n, s)| lines.insert(n, s));

    c.bench_with_input(
        BenchmarkId::new("get_max_part2_usize", 100),
        &lines,
        |b, i| {
            b.iter(|| {
                i.par_iter()
                    .map(|line| unsafe { get_max_part2_usize::<12>(*line) })
                    .sum::<usize>()
            });
        },
    );
}

fn aoc2025_d3_part1(c: &mut Criterion) {
    let mut lines = Vec::with_capacity(1000);
    let file = read_to_string("./src/input.txt").unwrap();
    file.lines()
        .enumerate()
        .for_each(|(n, s)| lines.insert(n, s));

    c.bench_with_input(
        BenchmarkId::new("get_max_part1_usize", 100),
        &lines,
        |b, i| {
            b.iter(|| {
                i.par_iter()
                    .map(|line| unsafe { get_max_part2_usize::<2>(*line) })
                    .sum::<usize>()
            });
        },
    );
}
