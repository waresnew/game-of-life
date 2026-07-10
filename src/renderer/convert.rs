use std::fmt;

use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{
    config::CELL_SIZE_EXP,
    quadtree_pool::Quadtree,
    renderer::{Renderer, controls::CellPoint},
};

impl Renderer {
    // fn world_to_screen(&self, point: Point) -> Point {}
    pub fn to_visible_alives(&self, id: usize, min: CellPoint, ans: &mut Vec<i64>) {
        match &self.solver.pool[id] {
            Quadtree::Subtree(root) => {
                if self.box_in_viewport(
                    min,
                    CellPoint::new(min.x + (1 << root.height), min.y + (1 << root.height)),
                ) {
                    return;
                }
                if root.count == BigUint::ZERO {
                    return;
                }

                //2^x mult/div
                if (root.height as i32) + (CELL_SIZE_EXP as i32) + self.viewport_info.zoom_exp <= 0
                {
                    if root.count > BigUint::ZERO {
                        ans.extend([min.x, min.y, root.height as i64]);
                    }
                    return;
                }
                let mid = 1 << (root.height - 1);
                self.to_visible_alives(root.tl, CellPoint::new(min.x, min.y + mid), ans);
                self.to_visible_alives(root.tr, CellPoint::new(min.x + mid, min.y + mid), ans);
                self.to_visible_alives(root.bl, min, ans);
                self.to_visible_alives(root.br, CellPoint::new(min.x + mid, min.y), ans);
            }
            &Quadtree::Cell(alive) => {
                if alive && self.point_in_viewport(min) {
                    ans.extend([min.x, min.y, 0]);
                }
            }
        }
    }

    pub(super) fn box_in_viewport(&self, box_min: CellPoint, box_max: CellPoint) -> bool {
        box_min.x > self.viewport_info.bound_max.x
            || box_min.y > self.viewport_info.bound_max.y
            || box_max.x < self.viewport_info.bound_min.x
            || box_max.y < self.viewport_info.bound_min.y
    }

    pub(super) fn point_in_viewport(&self, point: CellPoint) -> bool {
        Self::point_in_box(
            point,
            self.viewport_info.bound_min,
            self.viewport_info.bound_max,
        )
    }
    pub(super) fn point_in_box(point: CellPoint, box_min: CellPoint, box_max: CellPoint) -> bool {
        !(point.x > box_max.x || point.x < box_min.x || point.y > box_max.y || point.y < box_min.y)
    }
}
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct ScreenPoint {
    pub x: usize,
    pub y: usize,
}
#[wasm_bindgen]
impl ScreenPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
impl fmt::Debug for ScreenPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
