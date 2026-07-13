use gloo_console::log;
use num_bigint::{BigInt, BigUint};
use serde::{Deserialize, Serialize, ser::SerializeStruct};
use wasm_bindgen::prelude::*;

use crate::{
    quadtree_pool::{self, QuadtreePool},
    renderer::CellPoint,
};

mod hashlife;
mod liferule;
pub use liferule::*;
pub struct Solver {
    pub perf_stats: PerfStats,
    pub pool: QuadtreePool,
    pub root: usize,
    step_exp: u32,
    rule: LifeRule,
}
pub const INITIAL_HEIGHT: u32 = 32;
impl Solver {
    pub fn new(step_exp: u32, rules: LifeRule) -> Self {
        let mut pool = QuadtreePool::new();
        let root = pool.zeros(INITIAL_HEIGHT);
        Self {
            perf_stats: PerfStats::default(),
            pool,
            root,
            step_exp,
            rule: rules,
        }
    }
    pub fn get_min_point(&self) -> CellPoint {
        //TODO: reutrn borrowed value?
        let offset = BigInt::from(1) << (self.pool[self.root].as_subtree().height - 1);
        CellPoint::new(-offset.clone(), -offset.clone())
    }
    pub fn update_stats(&mut self) {
        self.perf_stats.alives = self.pool[self.root].as_subtree().count.to_str_radix(10);
        self.perf_stats.pool_mem = self.pool.estimate_pool_mem();
        self.perf_stats.height = self.pool[self.root].as_subtree().height;
    }
    pub fn next_step(&mut self) {
        self.perf_stats.cache_hits = 0;
        self.perf_stats.cache_misses = 0;
        let mut input = self.pool.add_border(self.root);
        while self.pool[input].as_subtree().height - 2 < self.step_exp {
            input = self.pool.add_border(input);
        }
        self.root = self.evolve(input);
        if let Some((new_pool, new_root)) = self.pool.gc_pool_if_needed(self.root) {
            self.pool = new_pool;
            self.root = new_root;
        }
        #[must_use]
        fn try_grow_quadtree(pool: &mut QuadtreePool, mut root: usize) -> usize {
            let centre = pool.get_centre(root);
            let root_count = &pool[root].as_subtree().count;
            let centre_count = &pool[centre].as_subtree().count;
            let needed = root_count - centre_count > BigUint::ZERO;
            if needed {
                root = pool.add_border(root);
            }
            root
        }
        self.root = try_grow_quadtree(&mut self.pool, self.root);
        self.update_stats();
    }
    pub fn set_step_exp(&mut self, step_exp: u32) {
        self.step_exp = step_exp;
        self.pool.clear_ans();
    }
    pub fn step_exp(&self) -> u32 {
        self.step_exp
    }

    pub fn rule(&self) -> LifeRule {
        self.rule
    }
    pub fn set_rule(&mut self, b: Vec<usize>, s: Vec<usize>) {
        self.rule = LifeRule::from_dense(b, s);
        self.pool.clear_ans();
    }
}
#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug)]
pub struct PerfStats {
    pub alives: String,
    pub pool_mem: usize,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub height: u32,
}
impl Default for PerfStats {
    fn default() -> Self {
        Self {
            alives: String::from("0"),
            pool_mem: 0,
            cache_hits: 0,
            cache_misses: 0,
            height: INITIAL_HEIGHT,
        }
    }
}
