use std::fmt;

use wasm_bindgen::prelude::*;

use crate::{app::CELL_SIZE_EXP, input_handler::Viewport};

/*
 * currently these structs purely serve to prevent mixing coordinate spaces
 */

#[derive(Default, Hash, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
#[wasm_bindgen]
pub struct ScreenPoint {
    pub x: i128,
    pub y: i128,
}
impl fmt::Debug for ScreenPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[wasm_bindgen]
impl ScreenPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i128, y: i128) -> Self {
        Self { x, y }
    }
}
impl ScreenPoint {
    pub fn to_cell(&self, viewport: &Viewport) -> CellPoint {
        let world = self.to_world(viewport);
        let cell_size = 1 << CELL_SIZE_EXP;
        CellPoint::new(world.x.div_euclid(cell_size), world.y.div_euclid(cell_size))
    }
    pub fn to_world(&self, viewport: &Viewport) -> WorldPoint {
        let zoom_out = 1 << viewport.camera.zoom_out_exp;
        WorldPoint::new(
            (self.x + viewport.camera.centre.x - viewport.canvas_dims.x / 2) * zoom_out,
            (self.y + viewport.camera.centre.y - viewport.canvas_dims.y / 2) * -zoom_out,
        )
    }
}
#[derive(Default, Hash, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct CellPoint {
    pub x: i128,
    pub y: i128,
}
impl fmt::Debug for CellPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl CellPoint {
    pub fn new(x: i128, y: i128) -> Self {
        Self { x, y }
    }
    pub fn from_tuple((x, y): (i128, i128)) -> Self {
        Self::new(x, y)
    }
    pub fn in_box(&self, box_min: CellPoint, box_max: CellPoint) -> bool {
        !(self.x > box_max.x || self.x < box_min.x || self.y > box_max.y || self.y < box_min.y)
    }
    pub fn to_screen(&self, viewport: &Viewport) -> ScreenPoint {
        let cell_size = 1 << CELL_SIZE_EXP;
        let zoom_out = 1 << viewport.camera.zoom_out_exp;
        let x = (self.x * cell_size).div_euclid(zoom_out) - viewport.camera.centre.x
            + viewport.canvas_dims.x / 2;
        let y = (-self.y * cell_size).div_euclid(zoom_out) - viewport.camera.centre.y
            + viewport.canvas_dims.y / 2;
        ScreenPoint::new(x, y)
    }
}
#[derive(Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct WorldPoint {
    pub x: i128,
    pub y: i128,
}
impl fmt::Debug for WorldPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl WorldPoint {
    pub fn new(x: i128, y: i128) -> Self {
        Self { x, y }
    }
}
