use std::{collections::HashSet, hint::black_box};

use criterion::{Criterion, criterion_group, criterion_main};
use game_of_life::{
    CellPoint, {GOL_RULES, Solver},
};
use rand::{distr::Uniform, prelude::*};

fn random_rect(c: &mut Criterion) {
    let seed = 1234;
    let mut alive: HashSet<(i128, i128)> = HashSet::new();
    let mut rng = SmallRng::seed_from_u64(seed);
    let distr = Uniform::new(-32, 32).unwrap();
    while alive.len() < 2048 {
        let x = rng.sample(distr);
        let y = rng.sample(distr);
        alive.insert((x, y));
    }
    let input: Vec<CellPoint> = alive.into_iter().map(CellPoint::from_tuple).collect();
    let mut solver = Solver::new(12, GOL_RULES);
    for p in input.clone() {
        solver.toggle_cell(p);
    }
    c.bench_function("random 64x64 next_step", |b| {
        b.iter(|| {
            solver.next_step();
            solver.set_step_exp(black_box(12)); //reset ans
        })
    });
}

criterion_group!(benches, random_rect);
criterion_main!(benches);
