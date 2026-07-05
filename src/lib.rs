use std::fmt;

use crate::{hashlife::evolve, quadtree::QuadtreePool};
use num_bigint::BigUint;
use wasm_bindgen::prelude::*;

mod hashlife;
mod quadtree;

#[wasm_bindgen]
#[derive(Default, Hash, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct WorldPoint {
    pub x: i64,
    pub y: i64,
}
impl fmt::Debug for WorldPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct RendererOutput {
    pub min: WorldPoint,
    pub size_exp: u32,
}
impl RendererOutput {
    pub fn unit_cell(min: WorldPoint) -> Self {
        Self { min, size_exp: 0 }
    }
}

#[wasm_bindgen]
impl WorldPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}
impl WorldPoint {
    pub fn from_tuple((x, y): (i64, i64)) -> Self {
        WorldPoint::new(x, y)
    }
    pub fn negate(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

const MAX_HEIGHT: u32 = 50;
pub const MIN_POINT: WorldPoint = WorldPoint {
    x: -1_000_000_000_000_000,
    y: -1_000_000_000_000_000,
};
#[wasm_bindgen]
pub struct Renderer {
    solver: Solver,
    pub base_cell_size: u32,
}
impl Renderer {
    pub fn set_alives(&mut self, alives: BigUint) {
        self.solver.perf_stats.alives = alives.to_str_radix(10);
    }
}
#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(getter=perf_stats)]
    pub fn get_perf_stats_copy(&self) -> PerfStats {
        self.solver.perf_stats.clone()
    }
    #[wasm_bindgen(constructor)]
    pub fn new(step_exp: u32, base_cell_size: u32) -> Self {
        Self {
            solver: Solver::new(step_exp),
            base_cell_size,
        }
    }
    pub fn next_step(&mut self) {
        self.solver.next_step();
    }
    pub fn render(
        &self,
        zoom: f64,
        bound_min: WorldPoint,
        bound_max: WorldPoint,
    ) -> Vec<RendererOutput> {
        self.solver.pool.to_visible_alives(
            self.solver.quadtree,
            (bound_min, bound_max),
            self.base_cell_size,
            zoom,
            MIN_POINT,
        )
    }
    pub fn toggle_cell(&mut self, point: WorldPoint) {
        self.solver.quadtree =
            self.solver
                .pool
                .toggle_cell_and_return_root(point, self.solver.quadtree, MIN_POINT);
        let alives = self.solver.pool[self.solver.quadtree]
            .as_subtree()
            .count
            .clone();
        self.set_alives(alives);
    }
    pub fn change_step_exp(&mut self, step_exp: u32) {
        self.solver.step_exp = step_exp;
        self.solver.pool.reset_ans();
    }
    pub fn clear_grid(&mut self) {
        self.solver = Solver::new(self.solver.step_exp);
    }
}
pub struct Solver {
    pub perf_stats: PerfStats,
    pool: QuadtreePool,
    quadtree: usize,
    step_exp: u32,
}

impl Solver {
    pub fn new(step_exp: u32) -> Self {
        let mut pool = QuadtreePool::new();
        let quadtree = pool.zeros(MAX_HEIGHT);
        Self {
            perf_stats: PerfStats::default(),
            pool,
            quadtree,
            step_exp,
        }
    }
    pub fn next_step(&mut self) {
        self.perf_stats.cache_hits = 0;
        self.perf_stats.cache_misses = 0;
        self.quadtree = evolve(self.pool.add_border(self.quadtree), self);
        (self.pool, self.quadtree) = self.pool.gc_pool(self.quadtree);
        self.perf_stats.alives = self.pool[self.quadtree].as_subtree().count.to_str_radix(10);
        self.perf_stats.pool_size = self.pool.size();
    }
}

#[wasm_bindgen(start, private)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Debug)]
pub struct PerfStats {
    pub alives: String,
    pub pool_size: usize,
    pub cache_hits: u64,
    pub cache_misses: u64,
}
impl Default for PerfStats {
    fn default() -> Self {
        Self {
            alives: String::from("0"),
            pool_size: 0,
            cache_hits: 0,
            cache_misses: 0,
        }
    }
}
