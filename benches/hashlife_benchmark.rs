use std::collections::HashSet;

use criterion::{Criterion, criterion_group, criterion_main};
use game_of_life::{Point, Solver};
use rand::{distr::Uniform, prelude::*};

fn random_rect(c: &mut Criterion) {
    let seed = 1234;
    let mut alive: HashSet<(i64, i64)> = HashSet::new();
    let mut rng = SmallRng::seed_from_u64(seed);
    let distr = Uniform::new(-32, 32).unwrap();
    while alive.len() < 2048 {
        let x = rng.sample(distr);
        let y = rng.sample(distr);
        alive.insert((x, y));
    }
    let input: Vec<Point> = alive.into_iter().map(Point::from_tuple).collect();
    c.bench_function("random 64x64", |b| {
        b.iter(|| {
            let mut solver = Solver::new(input.clone(), 12);
            solver.next_step();
        })
    });
}

criterion_group!(benches, random_rect);
criterion_main!(benches);
