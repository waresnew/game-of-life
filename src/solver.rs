use wasm_bindgen::prelude::*;

use crate::{config::MAX_HEIGHT, quadtree_pool::QuadtreePool};

mod hashlife;
#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct LifeRule {
    born: [bool; 9],
    survive: [bool; 9],
}
#[wasm_bindgen]
impl LifeRule {
    #[wasm_bindgen(getter)]
    pub fn born(&self) -> Vec<usize> {
        self.born
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| if x { Some(i) } else { None })
            .collect::<Vec<usize>>()
    }
    #[wasm_bindgen(getter)]
    pub fn survive(&self) -> Vec<usize> {
        self.survive
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| if x { Some(i) } else { None })
            .collect::<Vec<usize>>()
    }
}
impl LifeRule {
    pub fn is_born(&self, num: usize) -> bool {
        self.born[num]
    }
    pub fn survives(&self, num: usize) -> bool {
        self.survive[num]
    }
}
pub const GOL_RULES: LifeRule = LifeRule {
    born: [false, false, false, true, false, false, false, false, false],
    survive: [false, false, true, true, false, false, false, false, false],
};
pub struct Solver {
    pub perf_stats: PerfStats,
    pub pool: QuadtreePool,
    pub root: usize,
    step_exp: u32,
    rule: LifeRule,
}
impl Solver {
    pub fn new(step_exp: u32, rules: LifeRule) -> Self {
        let mut pool = QuadtreePool::new();
        let root = pool.zeros(MAX_HEIGHT);
        Self {
            perf_stats: PerfStats::default(),
            pool,
            root,
            step_exp,
            rule: rules,
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

    pub fn rule(&self) -> LifeRule {
        self.rule
    }
    pub fn set_rule(&mut self, b: Vec<usize>, s: Vec<usize>) {
        self.rule.born = [false; 9];
        self.rule.survive = [false; 9];
        for x in b {
            self.rule.born[x] = true;
        }
        for x in s {
            self.rule.survive[x] = true;
        }
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
