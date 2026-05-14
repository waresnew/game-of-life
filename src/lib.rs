use std::fmt;

use crate::{hashlife::next_step, quadtree::Quadtree};
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
    dict: AHashMap<u64, Quadtree>,
    quadtree: Option<u64>,
    step_exp: u32,
}

impl Default for Solver {
    fn default() -> Self {
        Self {
            perf_stats: Default::default(),
            dict: Default::default(),
            step_exp: 0,
            quadtree: Default::default(),
        }
    }
}
#[wasm_bindgen]
impl Solver {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
    pub fn init(&mut self, mut alive: Vec<Point>, step_exp: u32) {
        self.perf_stats = PerfStats::default();
        self.dict = AHashMap::new();
        self.quadtree = Some(Quadtree::from_alive(
            &mut alive,
            MIN_POINT,
            MAX_HEIGHT,
            &mut self.dict,
            &mut AHashMap::new(),
        ));
        self.step_exp = step_exp;
    }
    pub fn solve(&mut self) -> Vec<Point> {
        self.perf_stats.cache_hits = 0;
        self.perf_stats.cache_misses = 0;
        let mut cur = self
            .quadtree
            .expect("call init() at least once before solve()");
        cur = next_step(Quadtree::add_border(cur, &mut self.dict), self);
        let new_alive = (&self.dict[&cur])
            .to_alive(&self.dict, &mut AHashMap::new())
            .into_iter()
            .map(|Point { x, y }| Point::new(x + MIN_POINT.x, y + MIN_POINT.y))
            .collect();
        self.quadtree = Some(cur);
        self.gc_dict();
        new_alive
    }
    fn gc_dict(&mut self) {
        fn mark_gc(cur: u64, dict: &mut AHashMap<u64, Quadtree>) {
            let &Quadtree {
                tl,
                tr,
                bl,
                br,
                height,
                ans,
                ..
            } = &dict[&cur];
            if height == 0 {
                return;
            }
            for maybe_next in [Some(tl), Some(tr), Some(bl), Some(br), ans] {
                let Some(next) = maybe_next else {
                    continue;
                };
                let tree = dict.get_mut(&next).unwrap();
                if !tree.keep {
                    tree.keep = true;
                    mark_gc(next, dict);
                }
            }
        }
        self.dict.get_mut(&self.quadtree.unwrap()).unwrap().keep = true;
        mark_gc(self.quadtree.unwrap(), &mut self.dict);
        self.dict.retain(|_, v| v.keep);
        for (_, v) in &mut self.dict {
            v.keep = false;
        }
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
