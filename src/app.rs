use gloo_console::log;
use wasm_bindgen::prelude::*;

use crate::{
    app::stats::Stats,
    input_handler::InputHandler,
    point::ScreenPoint,
    renderer,
    solver::{GOL_RULES, Solver},
};
mod stats;
pub const CELL_SIZE_EXP: u32 = 5;
#[wasm_bindgen]
pub struct App {
    solver: Solver,
    input_handler: InputHandler,
    stats: Stats,
}
#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new(step_exp: u32) -> Self {
        Self {
            solver: Solver::new(step_exp, GOL_RULES),
            input_handler: InputHandler::new(),
            stats: Stats::default(),
        }
    }
    pub fn next_step(&mut self) {
        self.solver.next_step();
    }
    pub fn set_step_exp(&mut self, step_exp: u32) {
        self.solver.set_step_exp(step_exp);
    }
    pub fn update_canvas_dims(&mut self, canvas_dims: ScreenPoint) {
        self.input_handler.update_canvas_dims(canvas_dims);
    }
    pub fn get_stats(&mut self, cursor: ScreenPoint) -> Stats {
        self.stats.cell_cursor = cursor.to_cell(self.input_handler.viewport());
        self.stats.zoom_out_exp = self.input_handler.viewport().camera.zoom_out_exp;
        self.stats.rule = self.solver.rule();
        self.stats.solver_stats = self.solver.stats();
        self.stats.clone()
    }
    pub fn clear_grid(&mut self) {
        self.solver.clear_grid();
    }
    pub fn render(&self) -> Vec<u8> {
        renderer::render_to_image(
            self.input_handler.viewport(),
            self.solver.root,
            &self.solver.pool,
            &self.solver.get_min_point(),
        )
    }
    pub fn end_draw_session(&mut self) {
        self.input_handler.end_draw_session()
    }
    pub fn load_pattern(&mut self, pattern: String) {
        self.input_handler
            .load_rle_pattern(pattern, &mut self.solver);
    }
    pub fn set_rule(&mut self, b: Vec<usize>, s: Vec<usize>) {
        self.solver.set_rule(b, s);
    }
    pub fn handle_draw(&mut self, cursor: ScreenPoint) {
        self.input_handler.handle_draw(cursor, &mut self.solver);
    }
    pub fn handle_pan(&mut self, delta: ScreenPoint) {
        self.input_handler.handle_pan(delta);
    }
    pub fn handle_zoom(&mut self, delta: i32, cursor: ScreenPoint) {
        self.input_handler.handle_zoom(delta, cursor);
    }
}

#[wasm_bindgen(start, private)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
