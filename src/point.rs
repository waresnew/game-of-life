use std::fmt;

use malachite::{
    Integer,
    base::{num::arithmetic::traits::DivRound, rounding_modes::RoundingMode},
};
use wasm_bindgen::prelude::*;

use crate::{app::CELL_SIZE_EXP, input_handler::Viewport};

#[derive(Default, Hash, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
#[wasm_bindgen]
pub struct ScreenPoint {
    // can be negative for out of bounds pixels
    pub x: i64,
    pub y: i64,
}
impl fmt::Debug for ScreenPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[wasm_bindgen]
impl ScreenPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}
impl ScreenPoint {
    pub fn to_cell(&self, viewport: &Viewport) -> CellPoint {
        let world = self.to_world(viewport);
        let cell_size = Integer::from(1) << CELL_SIZE_EXP;
        CellPoint::new(
            world.x.div_round(&cell_size, RoundingMode::Floor).0,
            world.y.div_round(&cell_size, RoundingMode::Floor).0,
        )
    }
    pub fn to_world(&self, viewport: &Viewport) -> WorldPoint {
        let zoom_out = Integer::from(1) << viewport.camera.zoom_out_exp;
        WorldPoint::new(
            (Integer::from(self.x) + &viewport.camera.centre.x
                - Integer::from(viewport.canvas_dims.x / 2))
                * &zoom_out,
            (Integer::from(self.y) + &viewport.camera.centre.y
                - Integer::from(viewport.canvas_dims.y / 2))
                * -&zoom_out,
        )
    }
}
#[derive(Default, Hash, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct CellPoint {
    pub x: Integer,
    pub y: Integer,
}
impl fmt::Debug for CellPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl CellPoint {
    pub fn new(x: Integer, y: Integer) -> Self {
        Self { x, y }
    }
    pub fn from_tuple((x, y): (Integer, Integer)) -> Self {
        Self::new(x, y)
    }
    pub fn in_box(&self, box_min: &CellPoint, box_max: &CellPoint) -> bool {
        !(self.x > box_max.x || self.x < box_min.x || self.y > box_max.y || self.y < box_min.y)
    }
    pub fn to_screen(&self, viewport: &Viewport) -> ScreenPoint {
        let cell_size = Integer::from(1) << CELL_SIZE_EXP;
        let zoom_out = Integer::from(1) << viewport.camera.zoom_out_exp;
        let x = (&self.x * &cell_size)
            .div_round(&zoom_out, RoundingMode::Floor)
            .0
            - &viewport.camera.centre.x
            + Integer::from(viewport.canvas_dims.x / 2);
        let y = (-&self.y * &cell_size)
            .div_round(&zoom_out, RoundingMode::Floor)
            .0
            - &viewport.camera.centre.y
            + Integer::from(viewport.canvas_dims.y / 2);
        fn into_clamped_i64(n: Integer) -> i64 {
            if n < i64::MIN {
                i64::MIN
            } else if n > i64::MAX {
                i64::MAX
            } else {
                i64::try_from(&n).unwrap()
            }
        }
        ScreenPoint::new(into_clamped_i64(x), into_clamped_i64(y))
    }
}
#[derive(Default, PartialEq, PartialOrd, Clone)]
pub struct WorldPoint {
    pub x: Integer,
    pub y: Integer,
}
impl fmt::Debug for WorldPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl WorldPoint {
    pub fn new(x: Integer, y: Integer) -> Self {
        Self { x, y }
    }
}
