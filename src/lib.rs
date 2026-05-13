use std::fmt;

use crate::{
    hashlife::next_step,
    quadtree::Quadtree,
    utils::{PerfStats, decompose_bits},
};
use ahash::AHashMap;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::utils::web::*;

mod hashlife;
mod quadtree;
mod utils;

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
fn calc_start_pos(alive: &Vec<Point>) -> Point {
    if alive.is_empty() {
        return Point::new(0, 0);
    }
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    for &Point { x, y } in alive {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
    }
    Point::new(min_x, min_y)
}
const MAX_HEIGHT: u32 = 53;
const MIN_POINT: Point = Point {
    x: -1_000_000_000_000_000,
    y: -1_000_000_000_000_000,
};
#[wasm_bindgen]
#[derive(Default)]
pub struct Solver {
    pub perf_stats: PerfStats,
    dict: AHashMap<u64, Quadtree>,
    next_step_dp: AHashMap<u64, Quadtree>,
    quadtree: Option<Quadtree>,
}

impl Solver {
    pub fn load_alive(&mut self, alive: &mut Vec<Point>) {
        self.quadtree = Some(Quadtree::from_alive(
            alive,
            MIN_POINT,
            MAX_HEIGHT,
            &mut self.dict,
            &mut AHashMap::new(),
        ));
    }
}
#[wasm_bindgen]
impl Solver {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
    pub fn reset(&mut self, mut alive: Vec<Point>) {
        self.perf_stats = PerfStats::default();
        self.dict = AHashMap::new();
        self.next_step_dp = AHashMap::new();
        self.load_alive(&mut alive);
    }
    pub fn solve(&mut self, n: u64) -> Vec<Point> {
        self.perf_stats.cache_hits = 0;
        self.perf_stats.cache_misses = 0;
        let mut cur = self
            .quadtree
            .expect("call load_alive() at least once before solve()");
        let bits = decompose_bits(n);
        for k in bits {
            cur = next_step(
                Quadtree::add_border(cur, &mut self.dict),
                k,
                &mut self.dict,
                &mut self.next_step_dp,
                &mut self.perf_stats,
            );
        }
        let new_alive = cur
            .to_alive(&self.dict, &mut AHashMap::new())
            .into_iter()
            .map(|Point { x, y }| Point::new(x + MIN_POINT.x, y + MIN_POINT.y))
            .collect();
        self.quadtree = Some(cur);
        new_alive
    }
}

#[wasm_bindgen(start, private)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
