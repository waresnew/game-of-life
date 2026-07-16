use gloo_console::log;
use malachite::{Integer, Natural};

use crate::{point::CellPoint, quadtree_pool::QuadtreePool};

mod cell_ops;
mod hashlife;
mod liferule;
pub use liferule::*;
pub struct Solver {
    stats: SolverStats,
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
            stats: SolverStats::default(),
            pool,
            root,
            step_exp,
            rule: rules,
        }
    }
    pub fn clear_grid(&mut self) {
        self.root = self.pool.zeros(INITIAL_HEIGHT);
    }
    pub fn get_min_point(&self) -> CellPoint {
        //TODO: reutrn borrowed value?
        let offset = Integer::from(1) << (self.pool[self.root].as_subtree().height - 1);
        CellPoint::new(-offset.clone(), -offset.clone())
    }
    pub fn stats(&mut self) -> SolverStats {
        self.stats.alives = self.pool[self.root].as_subtree().count.clone();
        self.stats.pool_mem = self.pool.estimate_pool_mem();
        self.stats.height = self.pool[self.root].as_subtree().height;
        self.stats.clone()
    }
    pub fn next_step(&mut self) {
        self.stats.cache_hits = 0;
        self.stats.cache_misses = 0;
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
            let needed = root_count - centre_count > 0;
            if needed {
                root = pool.add_border(root);
            }
            root
        }
        self.root = try_grow_quadtree(&mut self.pool, self.root);
    }
    pub fn set_step_exp(&mut self, step_exp: u32) {
        self.step_exp = step_exp;
        self.pool.clear_ans();
    }

    pub fn rule(&self) -> LifeRule {
        self.rule
    }
    pub fn set_rule(&mut self, b: Vec<usize>, s: Vec<usize>) {
        self.rule = LifeRule::from_dense(b, s);
        self.pool.clear_ans();
    }
}
#[derive(Clone, Debug)]
pub struct SolverStats {
    pub alives: Natural,
    pub pool_mem: usize,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub height: u32,
}
impl Default for SolverStats {
    fn default() -> Self {
        Self {
            alives: Natural::from(0_u32),
            pool_mem: 0,
            cache_hits: 0,
            cache_misses: 0,
            height: INITIAL_HEIGHT,
        }
    }
}
