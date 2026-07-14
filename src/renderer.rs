use ahash::HashSet;
use gloo_console::log;
use num_bigint::BigInt;
use num_integer::Integer;
use wasm_bindgen::prelude::*;

use crate::solver::{GOL_RULES, LifeRule, PerfStats, Solver};
mod controls;
mod drawing;
mod image_bitmap;
mod point;
#[cfg(not(target_arch = "wasm32"))]
mod test_utils;
use image_bitmap::*;
pub use point::*;
pub const CELL_SIZE_EXP: u32 = 5;
#[derive(Copy, Clone)]
#[wasm_bindgen]
pub struct ViewportInfo {
    pub canvas_dims: ScreenPoint,
}
#[wasm_bindgen]
impl ViewportInfo {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_dims: ScreenPoint) -> Self {
        Self { canvas_dims }
    }
}
pub struct Camera {
    pub centre: WorldPoint,
    pub zoom_out_exp: u32,
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            centre: WorldPoint::new(BigInt::from(0), BigInt::from(0)),
            zoom_out_exp: 0,
        }
    }
}
impl Default for ViewportInfo {
    fn default() -> Self {
        Self {
            canvas_dims: ScreenPoint::new(0, 0),
        }
    }
}
#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct RenderStatsDisplay {
    cell_cursor: CellPoint,
    pub zoom_out_exp: u32,
    rule: LifeRule,
}
#[wasm_bindgen]
impl RenderStatsDisplay {
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
}
impl Default for RenderStatsDisplay {
    fn default() -> Self {
        Self {
            cell_cursor: CellPoint::new(BigInt::from(0), BigInt::from(0)),
            zoom_out_exp: 0,
            rule: GOL_RULES,
        }
    }
}
#[wasm_bindgen]
pub struct Renderer {
    solver: Solver,
    viewport_info: ViewportInfo,
    render_stats: RenderStatsDisplay,
    camera: Camera,
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
            render_stats: RenderStatsDisplay::default(),
            camera: Camera::default(),
            draw_session: HashSet::default(),
        }
    }
    #[wasm_bindgen(getter)]
    pub fn render_stats(&self) -> RenderStatsDisplay {
        self.render_stats.clone()
    }
    pub fn next_step(&mut self) {
        self.solver.next_step();
    }
    pub fn set_step_exp(&mut self, step_exp: u32) {
        self.solver.set_step_exp(step_exp);
    }
    pub fn update_viewport(&mut self, viewport_info: ViewportInfo) {
        self.viewport_info = viewport_info;
    }
    pub fn update_render_stats(&mut self, cursor: ScreenPoint) {
        self.render_stats.cell_cursor = self.screen_to_cell(cursor);
        self.render_stats.zoom_out_exp = self.camera.zoom_out_exp;
        self.render_stats.rule = self.solver.rule()
    }
    pub fn render(&self) -> Vec<u8> {
        let mut ans = ImageBitmap::new(self.viewport_info.canvas_dims);
        self.draw_visible_alives(self.solver.root, &self.solver.get_min_point(), &mut ans);
        self.draw_grid(&mut ans);
        ans.into_pixels()
    }
    pub fn end_draw_session(&mut self) {
        self.draw_session.clear();
    }
    pub fn clear_grid(&mut self) {
        self.solver = Solver::new(self.solver.step_exp(), self.solver.rule());
    }
    pub fn load_pattern(&mut self, pattern: String) {
        self.load_rle_pattern(pattern);
    }
    pub fn set_rule(&mut self, b: Vec<usize>, s: Vec<usize>) {
        self.solver.set_rule(b, s);
    }
    pub fn handle_zoom(&mut self, delta: i32, cursor: ScreenPoint) {
        let new_zoom_out_exp = (self.camera.zoom_out_exp as i32 + delta).max(0) as u32;
        let world_cursor = self.screen_to_world(cursor);
        let new_zoom_out = BigInt::from(1) << new_zoom_out_exp;
        let old_zoom_out = BigInt::from(1) << self.camera.zoom_out_exp;
        self.camera.centre = WorldPoint::new(
            world_cursor.x.div_floor(&new_zoom_out) - world_cursor.x.div_floor(&old_zoom_out)
                + &self.camera.centre.x,
            -world_cursor.y.div_floor(&new_zoom_out)
                + world_cursor.y.div_floor(&old_zoom_out)
                + &self.camera.centre.y,
        );
        self.camera.zoom_out_exp = new_zoom_out_exp;
    }
    pub fn handle_pan(&mut self, delta: ScreenPoint) {
        self.camera.centre = WorldPoint::new(
            delta.x + &self.camera.centre.x,
            delta.y + &self.camera.centre.y,
        );
    }
    pub fn handle_draw(&mut self, cursor: ScreenPoint) {
        let cell_cursor = self.screen_to_cell(cursor);
        if !self.draw_session.contains(&cell_cursor) {
            self.draw_session.insert(cell_cursor.clone());
            self.toggle_cell(&cell_cursor.clone()); //TODO: is a clone needed here conceptaully
        }
    }
}
