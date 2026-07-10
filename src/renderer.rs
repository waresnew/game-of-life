use std::fmt;

use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::{
    config::{MIN_POINT, RENDER_OUTPUT_SIZE},
    solver::{PerfStats, Solver},
};
mod controls;
mod convert;
#[wasm_bindgen]
pub struct Renderer {
    solver: Solver,
}
#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(getter)]
    pub fn perf_stats(&self) -> PerfStats {
        self.solver.perf_stats.clone()
    }
    #[wasm_bindgen(constructor)]
    pub fn new(step_exp: u32) -> Self {
        Self {
            solver: Solver::new(step_exp),
        }
    }
    pub fn next_step(&mut self) {
        self.solver.next_step();
    }
    pub fn set_step_exp(&mut self, step_exp: u32) {
        self.solver.set_step_exp(step_exp);
    }
    #[cfg(target_arch = "wasm32")]
    /// only to be used for wasm bc the function signature is unergonomic
    pub fn render(
        &self,
        zoom: f64,
        bound_min_x: i64,
        bound_min_y: i64,
        bound_max_x: i64,
        bound_max_y: i64,
    ) -> Vec<i64> {
        let mut ans = Vec::new();
        self.to_visible_alives(
            self.solver.root,
            (
                WorldPoint::new(bound_min_x, bound_min_y),
                WorldPoint::new(bound_max_x, bound_max_y),
            ),
            zoom,
            MIN_POINT,
            &mut ans,
        );
        ans
    }
    pub fn toggle_cell(&mut self, x: i64, y: i64) {
        self.solver.root =
            self.toggle_cell_and_return_root(WorldPoint::new(x, y), self.solver.root, MIN_POINT);
        self.solver.update_stats();
    }
    pub fn clear_grid(&mut self) {
        self.solver = Solver::new(self.solver.step_exp());
    }
}
impl Renderer {
    /// tests/benches only, ignores size_exp
    pub fn render_all(&self) -> Vec<WorldPoint> {
        self.render_visible(MIN_POINT, WorldPoint::negate(MIN_POINT), 1.0)
    }
    /// tests/benches only
    pub fn render_visible(&self, min: WorldPoint, max: WorldPoint, zoom: f64) -> Vec<WorldPoint> {
        let mut ans = Vec::new();
        self.to_visible_alives(self.solver.root, (min, max), zoom, MIN_POINT, &mut ans);
        ans.chunks_exact(RENDER_OUTPUT_SIZE)
            .map(|chunk| {
                let &[x, y, _size_exp] = chunk else {
                    unreachable!();
                };
                WorldPoint::new(x, y)
            })
            .collect()
    }
}
#[derive(Serialize, Default, Hash, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct WorldPoint {
    pub x: i64,
    pub y: i64,
}
impl fmt::Debug for WorldPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl WorldPoint {
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
