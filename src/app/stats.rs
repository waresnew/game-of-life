use wasm_bindgen::prelude::*;

use crate::{
    point::CellPoint,
    solver::{GOL_RULES, LifeRule, SolverStats},
};

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Stats {
    #[wasm_bindgen(skip)]
    pub cell_cursor: CellPoint,

    pub zoom_out_exp: u32,

    #[wasm_bindgen(skip)]
    pub rule: LifeRule,

    #[wasm_bindgen(skip)]
    pub solver_stats: SolverStats,
}
#[wasm_bindgen]
impl Stats {
    #[wasm_bindgen]
    pub fn default() -> Self {
        Self {
            cell_cursor: CellPoint::new(0, 0),
            zoom_out_exp: 0,
            rule: GOL_RULES,
            solver_stats: SolverStats::default(),
        }
    }
    #[wasm_bindgen(getter)]
    pub fn rule_b(&self) -> Vec<usize> {
        let mut ans = Vec::new();
        for i in 0..9 {
            if self.rule.is_born(i) {
                ans.push(i);
            }
        }
        ans
    }
    #[wasm_bindgen(getter)]
    pub fn rule_s(&self) -> Vec<usize> {
        let mut ans = Vec::new();
        for i in 0..9 {
            if self.rule.survives(i) {
                ans.push(i);
            }
        }
        ans
    }
    #[wasm_bindgen(getter)]
    pub fn cell_cursor_x(&self) -> String {
        self.cell_cursor.x.to_string()
    }
    #[wasm_bindgen(getter)]
    pub fn cell_cursor_y(&self) -> String {
        self.cell_cursor.y.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn alives(&self) -> String {
        self.solver_stats.alives.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn pool_mem(&self) -> usize {
        self.solver_stats.pool_mem
    }
    #[wasm_bindgen(getter)]
    pub fn cache_hit_rate(&self) -> u64 {
        let total = self.solver_stats.cache_hits + self.solver_stats.cache_misses;
        (self.solver_stats.cache_hits * 100)
            .checked_div(total)
            .unwrap_or_default()
    }
}
