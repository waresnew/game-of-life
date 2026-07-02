use std::fmt;

use crate::{hashlife::evolve, quadtree::QuadtreePool};
use ahash::AHashMap;
use wasm_bindgen::prelude::*;

mod hashlife;
mod quadtree;

#[wasm_bindgen]
#[derive(Default, Hash, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}
impl Point {
    pub fn from_tuple((x, y): (i64, i64)) -> Self {
        Point::new(x, y)
    }
}

const MAX_HEIGHT: u32 = 50;
const MIN_POINT: Point = Point {
    x: -1_000_000_000_000_000,
    y: -1_000_000_000_000_000,
};
#[wasm_bindgen]
pub struct Solver {
    pub perf_stats: PerfStats,
    pool: QuadtreePool,
    quadtree: usize,
    step_exp: u32,
}

#[wasm_bindgen]
impl Solver {
    #[wasm_bindgen(constructor)]
    pub fn new(mut alive: Vec<Point>, step_exp: u32) -> Self {
        let mut pool = QuadtreePool::new();
        let quadtree = pool.load_alives(&mut alive, MIN_POINT, MAX_HEIGHT, &mut AHashMap::new());
        Self {
            perf_stats: PerfStats::default(),
            pool,
            quadtree,
            step_exp,
        }
    }
    pub fn next_step(&mut self) -> Vec<Point> {
        self.perf_stats.cache_hits = 0;
        self.perf_stats.cache_misses = 0;
        self.quadtree = evolve(self.pool.add_border(self.quadtree), self);
        let new_alive = self
            .pool
            .to_alive(self.quadtree, &mut AHashMap::new())
            .into_iter()
            .map(|Point { x, y }| Point::new(x + MIN_POINT.x, y + MIN_POINT.y))
            .collect();
        (self.pool, self.quadtree) = self.pool.gc_pool(self.quadtree);
        new_alive
    }
}

#[wasm_bindgen(start, private)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Default)]
pub struct PerfStats {
    pub cache_hits: u64,
    pub cache_misses: u64,
}
