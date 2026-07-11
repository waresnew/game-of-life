use std::fmt;
use web_sys::console;

use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{
    config::CELL_SIZE_EXP,
    quadtree_pool::Quadtree,
    renderer::{Renderer, controls::CellPoint, image_bitmap::ImageBitmap},
};

impl Renderer {
    fn cell_to_screen(&self, point: CellPoint) -> CellPoint {
        //TODO: return type
        let x = (point.x * (1 << CELL_SIZE_EXP)).div_euclid(1 << self.viewport_info.zoom_out_exp)
            - self.viewport_info.centre.x
            + (self.viewport_info.canvas_dims.x / 2) as i64;
        let y = (-point.y * (1 << CELL_SIZE_EXP)).div_euclid(1 << self.viewport_info.zoom_out_exp)
            - self.viewport_info.centre.y
            + (self.viewport_info.canvas_dims.y / 2) as i64;
        CellPoint::new(x, y)
    }
    pub fn to_visible_alives(&self, id: usize, min: CellPoint, ans: &mut ImageBitmap) {
        let screen_point = self.cell_to_screen(min);
        //2^x mult/div
        let pixels_exp_tmp = (CELL_SIZE_EXP as i32) - (self.viewport_info.zoom_out_exp as i32);
        match &self.solver.pool[id] {
            Quadtree::Subtree(root) => {
                if !self.box_intersects_viewport(
                    min,
                    CellPoint::new(
                        min.x + (1 << root.height) - 1,
                        min.y + (1 << root.height) - 1,
                    ),
                ) {
                    return;
                }
                if root.count == BigUint::ZERO {
                    return;
                }

                let pixels_exp = (pixels_exp_tmp + (root.height as i32)).max(0) as u32;
                if pixels_exp == 0 {
                    if root.count > BigUint::ZERO {
                        ans.fill(screen_point, pixels_exp);
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
                    let pixels_exp = pixels_exp_tmp.max(0) as u32;
                    ans.fill(screen_point, pixels_exp);
                }
            }
        }
    }

    fn box_intersects_viewport(&self, box_min: CellPoint, box_max: CellPoint) -> bool {
        !(box_min.x > self.viewport_info.bound_max.x
            || box_min.y > self.viewport_info.bound_max.y
            || box_max.x < self.viewport_info.bound_min.x
            || box_max.y < self.viewport_info.bound_min.y)
    }

    fn point_in_viewport(&self, point: CellPoint) -> bool {
        Self::point_in_box(
            point,
            self.viewport_info.bound_min,
            self.viewport_info.bound_max,
        )
    }
    pub(super) fn point_in_box(point: CellPoint, box_min: CellPoint, box_max: CellPoint) -> bool {
        !(point.x > box_max.x || point.x < box_min.x || point.y > box_max.y || point.y < box_min.y)
    }
    /// for tests only
    #[cfg(not(target_arch = "wasm32"))]
    pub fn query_cell(&self, point: CellPoint) -> bool {
        use crate::{config::MIN_POINT, quadtree_pool::QuadtreePool};

        fn traverse(point: CellPoint, root: usize, min: CellPoint, pool: &QuadtreePool) -> bool {
            match &pool[root] {
                Quadtree::Subtree(subtree) => {
                    if !Renderer::point_in_box(
                        point,
                        min,
                        CellPoint::new(
                            min.x + (1 << subtree.height) - 1,
                            min.y + (1 << subtree.height) - 1,
                        ),
                    ) {
                        return false;
                    }
                    if subtree.count == BigUint::ZERO {
                        return false;
                    }
                    let mid = 1 << (subtree.height - 1);
                    traverse(point, subtree.tl, CellPoint::new(min.x, min.y + mid), pool)
                        || traverse(
                            point,
                            subtree.tr,
                            CellPoint::new(min.x + mid, min.y + mid),
                            pool,
                        )
                        || traverse(point, subtree.bl, min, pool)
                        || traverse(point, subtree.br, CellPoint::new(min.x + mid, min.y), pool)
                }
                &Quadtree::Cell(alive) => {
                    if min == point {
                        alive
                    } else {
                        false
                    }
                }
            }
        }
        traverse(point, self.solver.root, MIN_POINT, &self.solver.pool)
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
