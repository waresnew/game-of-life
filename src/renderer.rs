use serde::{Deserialize, Serialize};
use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

use crate::{
    config::{MIN_POINT, RENDER_OUTPUT_SIZE},
    solver::{PerfStats, Solver},
};
mod controls;
mod convert;
pub use controls::CellPoint;
pub use convert::ScreenPoint;
#[derive(Tsify, Serialize, Deserialize, Copy, Clone)]
pub struct ViewportInfo {
    pub bound_min: CellPoint,
    pub bound_max: CellPoint,
    pub zoom_out_exp: u32,
    pub canvas_dims: ScreenPoint,
    pub centre: ScreenPoint,
}
impl Default for ViewportInfo {
    fn default() -> Self {
        Self {
            bound_min: CellPoint::new(0, 0),
            bound_max: CellPoint::new(0, 0),
            zoom_out_exp: 0,
            canvas_dims: ScreenPoint::new(0, 0),
            centre: ScreenPoint::new(0, 0),
        }
    }
}
#[wasm_bindgen]
pub struct Renderer {
    solver: Solver,
    viewport_info: ViewportInfo,
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
            viewport_info: ViewportInfo::default(),
        }
    }
    pub fn next_step(&mut self) {
        self.solver.next_step();
    }
    pub fn set_step_exp(&mut self, step_exp: u32) {
        self.solver.set_step_exp(step_exp);
    }
    pub fn update_viewport(&mut self, viewport_info: Ts<ViewportInfo>) {
        self.viewport_info = viewport_info.to_rust().unwrap();
    }
    #[cfg(target_arch = "wasm32")]
    /// only to be used for wasm bc the function signature is unergonomic
    pub fn render(&self) -> Vec<i64> {
        let mut ans = Vec::new();
        self.to_visible_alives(self.solver.root, MIN_POINT, &mut ans);
        ans
    }
    pub fn toggle_cell(&mut self, x: i64, y: i64) {
        self.solver.root =
            self.toggle_cell_and_return_root(CellPoint::new(x, y), self.solver.root, MIN_POINT);
        self.solver.update_stats();
    }
    pub fn clear_grid(&mut self) {
        self.solver = Solver::new(self.solver.step_exp());
    }
}
impl Renderer {
    /// tests/benches only, ignores size_exp
    pub fn render_all(&mut self) -> Vec<CellPoint> {
        self.update_viewport(
            ViewportInfo {
                bound_min: MIN_POINT,
                bound_max: CellPoint::negate(MIN_POINT),
                zoom_out_exp: 0,
                centre: ScreenPoint::new(0, 0),
                canvas_dims: ScreenPoint::new(
                    MIN_POINT.x.unsigned_abs() as usize,
                    MIN_POINT.y.unsigned_abs() as usize,
                ),
            }
            .into_ts()
            .unwrap(),
        );
        self.render_visible()
    }
    /// tests/benches only
    pub fn render_visible(&self) -> Vec<CellPoint> {
        let mut ans = Vec::new();
        self.to_visible_alives(self.solver.root, MIN_POINT, &mut ans);
        ans.chunks_exact(RENDER_OUTPUT_SIZE)
            .map(|chunk| {
                let &[x, y, _size_exp] = chunk else {
                    unreachable!();
                };
                CellPoint::new(x, y)
            })
            .collect()
    }
}
