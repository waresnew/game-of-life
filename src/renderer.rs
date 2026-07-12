use ahash::HashSet;
use serde::{Deserialize, Serialize};
use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

use crate::{
    config::{MAX_HEIGHT, MIN_POINT},
    solver::{GOL_RULES, PerfStats, Solver},
};
mod controls;
mod drawing;
mod image_bitmap;
mod point;
#[cfg(not(target_arch = "wasm32"))]
mod test_utils;
use image_bitmap::*;
pub use point::*;
#[derive(Tsify, Serialize, Deserialize, Copy, Clone)]
pub struct ViewportInfo {
    pub canvas_dims: ScreenPoint,
    pub cursor: ScreenPoint,
}
pub struct Camera {
    pub centre: WorldPoint,
    pub zoom_out_exp: u32,
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            centre: WorldPoint::new(0, 0),
            zoom_out_exp: 0,
        }
    }
}
impl Default for ViewportInfo {
    fn default() -> Self {
        Self {
            canvas_dims: ScreenPoint::new(0, 0),
            cursor: ScreenPoint::new(0, 0),
        }
    }
}
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct RenderStats {
    pub cell_cursor: CellPoint,
    pub zoom_out_exp: u32,
}
impl Default for RenderStats {
    fn default() -> Self {
        Self {
            cell_cursor: CellPoint::new(0, 0),
            zoom_out_exp: 0,
        }
    }
}
#[wasm_bindgen]
pub struct Renderer {
    solver: Solver,
    viewport_info: ViewportInfo,
    pub render_stats: RenderStats,
    camera: Camera,
    cell_cursor: CellPoint,
    draw_session: HashSet<CellPoint>,
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
            solver: Solver::new(step_exp, GOL_RULES),
            viewport_info: ViewportInfo::default(),
            render_stats: RenderStats::default(),
            camera: Camera::default(),
            cell_cursor: CellPoint::new(0, 0),
            draw_session: HashSet::default(),
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
        self.cell_cursor = self.screen_to_cell(self.viewport_info.cursor);
    }
    pub fn update_render_stats(&mut self) {
        self.render_stats.cell_cursor = self.cell_cursor;
        self.render_stats.zoom_out_exp = self.camera.zoom_out_exp;
    }
    pub fn render(&self) -> Vec<u8> {
        let mut ans = ImageBitmap::new(self.viewport_info.canvas_dims);
        self.draw_visible_alives(self.solver.root, MIN_POINT, &mut ans);
        self.draw_grid(&mut ans);
        ans.into_pixels()
    }
    pub fn end_draw_session(&mut self) {
        self.draw_session.clear();
    }
    pub fn clear_grid(&mut self) {
        self.solver = Solver::new(self.solver.step_exp(), self.solver.rules());
    }
    pub fn load_pattern(&mut self, pattern: String) {
        self.load_rle_pattern(pattern);
    }
    pub fn set_rules(&mut self, b: Vec<usize>, s: Vec<usize>) {
        self.solver.set_rules(b, s);
    }
    pub fn handle_zoom(&mut self, delta: i32) {
        let new_zoom_out_exp =
            ((self.camera.zoom_out_exp as i32 + delta).max(0) as u32).min(MAX_HEIGHT);
        self.camera.centre = WorldPoint::new(
            self.cell_cursor.x.div_euclid(1 << new_zoom_out_exp)
                - self.cell_cursor.x.div_euclid(1 << self.camera.zoom_out_exp)
                + self.camera.centre.x,
            self.cell_cursor.y.div_euclid(1 << new_zoom_out_exp)
                - self.cell_cursor.y.div_euclid(1 << self.camera.zoom_out_exp)
                + self.camera.centre.y,
        );
        self.camera.zoom_out_exp = new_zoom_out_exp;
    }
    pub fn handle_pan(&mut self, delta: ScreenPoint) {
        self.camera.centre = WorldPoint::new(
            delta.x + self.camera.centre.x,
            delta.y + self.camera.centre.y,
        );
    }
    pub fn handle_draw(&mut self) {
        if !self.draw_session.contains(&self.cell_cursor) {
            self.draw_session.insert(self.cell_cursor);
            self.toggle_cell(self.cell_cursor);
        }
    }
}
