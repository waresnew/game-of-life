use wasm_bindgen::prelude::*;

use crate::quadtree_pool::QuadtreePool;

mod hashlife;
const MAX_HEIGHT: u32 = 50;
pub struct Solver {
    pub perf_stats: PerfStats,
    pub pool: QuadtreePool,
    pub root: usize,
    step_exp: u32,
}
impl Solver {
    pub fn new(step_exp: u32) -> Self {
        let mut pool = QuadtreePool::new();
        let root = pool.zeros(MAX_HEIGHT);
        Self {
            perf_stats: PerfStats::default(),
            pool,
            root,
            step_exp,
        }
    }
    pub fn update_stats(&mut self) {
        self.perf_stats.alives = self.pool[self.root].as_subtree().count.to_str_radix(10);
        self.perf_stats.pool_mem = self.pool.estimate_pool_mem();
    }
    pub fn next_step(&mut self) {
        self.perf_stats.cache_hits = 0;
        self.perf_stats.cache_misses = 0;
        let input = self.pool.add_border(self.root);
        self.root = self.evolve(input);
        if let Some((new_pool, new_root)) = self.pool.gc_pool_if_needed(self.root) {
            self.pool = new_pool;
            self.root = new_root;
        }
        self.update_stats();
    }
    pub fn set_step_exp(&mut self, step_exp: u32) {
        self.step_exp = step_exp;
        self.pool.clear_ans();
    }
    pub fn step_exp(&self) -> u32 {
        self.step_exp
    }
}
#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug)]
pub struct PerfStats {
    pub alives: String,
    pub pool_mem: usize,
    pub cache_hits: u64,
    pub cache_misses: u64,
}
impl Default for PerfStats {
    fn default() -> Self {
        Self {
            alives: String::from("0"),
            pool_mem: 0,
            cache_hits: 0,
            cache_misses: 0,
        }
    }
}
