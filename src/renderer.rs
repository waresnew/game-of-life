use std::fmt;

use wasm_bindgen::prelude::*;

use crate::solver::{PerfStats, Solver};
mod controls;
mod convert;
pub const MIN_POINT: WorldPoint = WorldPoint {
    x: -1_000_000_000_000_000,
    y: -1_000_000_000_000_000,
};
#[wasm_bindgen]
pub struct Renderer {
    solver: Solver,
    pub base_cell_size: u32,
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
impl Renderer {
    #[wasm_bindgen(getter)]
    pub fn perf_stats(&self) -> PerfStats {
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
    pub fn set_step_exp(&mut self, step_exp: u32) {
        self.solver.set_step_exp(step_exp);
    }
    pub fn render(
        &self,
        zoom: f64,
        bound_min: WorldPoint,
        bound_max: WorldPoint,
    ) -> Vec<RendererOutput> {
        self.to_visible_alives(
            self.solver.root,
            (bound_min, bound_max),
            self.base_cell_size,
            zoom,
            MIN_POINT,
        )
    }
    pub fn toggle_cell(&mut self, point: WorldPoint) {
        self.solver.root = self.toggle_cell_and_return_root(point, self.solver.root, MIN_POINT);
        self.solver.update_stats();
    }
    pub fn clear_grid(&mut self) {
        self.solver = Solver::new(self.solver.step_exp());
    }
}
impl Renderer {
    /// tests/benches only
    pub fn render_all(&self) -> Vec<RendererOutput> {
        self.render(1.0, MIN_POINT, WorldPoint::negate(MIN_POINT))
    }
}
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
