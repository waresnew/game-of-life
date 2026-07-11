use serde::{Deserialize, Serialize};
use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

use crate::{
    config::MIN_POINT,
    solver::{PerfStats, Solver},
};
mod controls;
mod convert;
mod image_bitmap;
pub use controls::Point;
use image_bitmap::*;
#[derive(Tsify, Serialize, Deserialize, Copy, Clone)]
pub struct ViewportInfo {
    pub bound_min: Point,
    pub bound_max: Point,
    pub zoom_out_exp: u32,
    pub canvas_dims: Point,
    pub centre: Point,
}
impl Default for ViewportInfo {
    fn default() -> Self {
        Self {
            bound_min: Point::new(0, 0),
            bound_max: Point::new(0, 0),
            zoom_out_exp: 0,
            canvas_dims: Point::new(0, 0),
            centre: Point::new(0, 0),
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
    pub fn render(&self) -> Vec<u8> {
        let mut ans = ImageBitmap::new(self.viewport_info.canvas_dims);
        self.to_visible_alives(self.solver.root, MIN_POINT, &mut ans);
        ans.into_pixels()
    }
    pub fn toggle_cell(&mut self, x: i64, y: i64) {
        self.solver.root =
            self.toggle_cell_and_return_root(Point::new(x, y), self.solver.root, MIN_POINT);
        self.solver.update_stats();
    }
    pub fn clear_grid(&mut self) {
        self.solver = Solver::new(self.solver.step_exp());
    }
}
impl Renderer {
    /// tests/benches only, ignores size_exp
    pub fn render_all(&mut self) -> Vec<u8> {
        self.update_viewport(
            ViewportInfo {
                bound_min: MIN_POINT,
                bound_max: Point::negate(MIN_POINT),
                zoom_out_exp: 0,
                centre: Point::new(0, 0),
                canvas_dims: Point::new(MIN_POINT.x, MIN_POINT.y),
            }
            .into_ts()
            .unwrap(),
        );
        self.render_visible()
    }
    /// tests/benches only
    pub fn render_visible(&self) -> Vec<u8> {
        let mut ans = ImageBitmap::new(self.viewport_info.canvas_dims);
        self.to_visible_alives(self.solver.root, MIN_POINT, &mut ans);
        ans.into_pixels()
    }
}
