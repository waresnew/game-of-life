use std::collections::HashSet;

use gloo_console::log;

use crate::{
    point::{CellPoint, ScreenPoint, WorldPoint},
    solver::Solver,
};

mod pattern_file;
pub struct InputHandler {
    viewport: Viewport,
    draw_session: HashSet<CellPoint>,
}
pub struct Viewport {
    pub canvas_dims: ScreenPoint,
    pub camera: Camera,
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
impl Default for Viewport {
    fn default() -> Self {
        Self {
            canvas_dims: ScreenPoint::new(0, 0),
            camera: Camera::default(),
        }
    }
}
impl InputHandler {
    pub fn new() -> Self {
        Self {
            viewport: Viewport::default(),
            draw_session: HashSet::new(),
        }
    }
    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }
    pub fn end_draw_session(&mut self) {
        self.draw_session.clear();
    }
    pub fn update_canvas_dims(&mut self, canvas_dims: ScreenPoint) {
        self.viewport.canvas_dims = canvas_dims;
    }
    pub fn handle_zoom(&mut self, delta: i32, cursor: ScreenPoint) {
        let new_zoom_out_exp = (self.viewport.camera.zoom_out_exp as i32 + delta).max(0) as u32;
        let world_cursor = cursor.to_world(&self.viewport);
        let new_zoom_out = 1 << new_zoom_out_exp;
        let old_zoom_out = 1 << self.viewport.camera.zoom_out_exp;
        self.viewport.camera.centre = WorldPoint::new(
            world_cursor.x.div_euclid(new_zoom_out) - world_cursor.x.div_euclid(old_zoom_out)
                + self.viewport.camera.centre.x,
            -world_cursor.y.div_euclid(new_zoom_out)
                + world_cursor.y.div_euclid(old_zoom_out)
                + self.viewport.camera.centre.y,
        );
        self.viewport.camera.zoom_out_exp = new_zoom_out_exp;
    }
    pub fn handle_pan(&mut self, delta: ScreenPoint) {
        self.viewport.camera.centre = WorldPoint::new(
            delta.x + self.viewport.camera.centre.x,
            delta.y + self.viewport.camera.centre.y,
        );
    }
    pub fn handle_draw(&mut self, cursor: ScreenPoint, solver: &mut Solver) {
        let cell_cursor = cursor.to_cell(&self.viewport);
        if !self.draw_session.contains(&cell_cursor) {
            self.draw_session.insert(cell_cursor);
            solver.toggle_cell(cell_cursor);
        }
    }
}
