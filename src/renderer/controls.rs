use std::{cell::Cell, fmt};

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{
    quadtree_pool::{ALIVE_CELL_ID, DEAD_CELL_ID, Quadtree, Subtree},
    renderer::Renderer,
};

impl Renderer {
    #[must_use]
    pub(super) fn toggle_cell_and_return_root(
        &mut self,
        point: CellPoint,
        root: usize,
        min: CellPoint,
    ) -> usize {
        match self.solver.pool[root] {
            Quadtree::Subtree(Subtree {
                tl,
                tr,
                bl,
                br,
                height,
                ..
            }) => {
                if !Self::point_in_box(
                    point,
                    min,
                    CellPoint::new(min.x + (1 << height) - 1, min.y + (1 << height) - 1),
                ) {
                    return root;
                }
                let mid = 1 << (height - 1);
                let tl =
                    self.toggle_cell_and_return_root(point, tl, CellPoint::new(min.x, min.y + mid));
                let tr = self.toggle_cell_and_return_root(
                    point,
                    tr,
                    CellPoint::new(min.x + mid, min.y + mid),
                );
                let bl = self.toggle_cell_and_return_root(point, bl, min);
                let br =
                    self.toggle_cell_and_return_root(point, br, CellPoint::new(min.x + mid, min.y));
                self.solver.pool.join(tl, tr, bl, br, height)
            }
            Quadtree::Cell(alive) => {
                if min == point {
                    if alive { DEAD_CELL_ID } else { ALIVE_CELL_ID }
                } else {
                    root
                }
            }
        }
    }
}
#[derive(Serialize, Deserialize, Default, Hash, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
#[wasm_bindgen]
pub struct CellPoint {
    pub x: i64,
    pub y: i64,
}
impl fmt::Debug for CellPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[wasm_bindgen]
impl CellPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}
impl CellPoint {
    pub fn from_tuple((x, y): (i64, i64)) -> Self {
        CellPoint::new(x, y)
    }
    pub fn negate(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
