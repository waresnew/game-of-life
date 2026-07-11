use num_bigint::BigUint;

use crate::{
    config::CELL_SIZE_EXP,
    quadtree_pool::Quadtree,
    renderer::{Renderer, controls::Point, image_bitmap::ImageBitmap},
};

impl Renderer {
    fn cell_to_screen(&self, point: Point) -> Point {
        let x = (point.x * (1 << CELL_SIZE_EXP)).div_euclid(1 << self.viewport_info.zoom_out_exp)
            - self.viewport_info.centre.x
            + (self.viewport_info.canvas_dims.x / 2);
        let y = (-point.y * (1 << CELL_SIZE_EXP)).div_euclid(1 << self.viewport_info.zoom_out_exp)
            - self.viewport_info.centre.y
            + (self.viewport_info.canvas_dims.y / 2);
        Point::new(x, y)
    }
    pub fn to_visible_alives(&self, id: usize, min: Point, ans: &mut ImageBitmap) {
        let screen_point = self.cell_to_screen(min);
        //2^x mult/div
        let pixels_exp_tmp = (CELL_SIZE_EXP as i32) - (self.viewport_info.zoom_out_exp as i32);
        match &self.solver.pool[id] {
            Quadtree::Subtree(root) => {
                if !self.box_intersects_viewport(
                    min,
                    Point::new(
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
                self.to_visible_alives(root.tl, Point::new(min.x, min.y + mid), ans);
                self.to_visible_alives(root.tr, Point::new(min.x + mid, min.y + mid), ans);
                self.to_visible_alives(root.bl, min, ans);
                self.to_visible_alives(root.br, Point::new(min.x + mid, min.y), ans);
            }
            &Quadtree::Cell(alive) => {
                if alive && self.point_in_viewport(min) {
                    let pixels_exp = pixels_exp_tmp.max(0) as u32;
                    ans.fill(screen_point, pixels_exp);
                }
            }
        }
    }

    fn box_intersects_viewport(&self, box_min: Point, box_max: Point) -> bool {
        !(box_min.x > self.viewport_info.bound_max.x
            || box_min.y > self.viewport_info.bound_max.y
            || box_max.x < self.viewport_info.bound_min.x
            || box_max.y < self.viewport_info.bound_min.y)
    }

    fn point_in_viewport(&self, point: Point) -> bool {
        Self::point_in_box(
            point,
            self.viewport_info.bound_min,
            self.viewport_info.bound_max,
        )
    }
    pub(super) fn point_in_box(point: Point, box_min: Point, box_max: Point) -> bool {
        !(point.x > box_max.x || point.x < box_min.x || point.y > box_max.y || point.y < box_min.y)
    }
    /// for tests only
    #[cfg(not(target_arch = "wasm32"))]
    pub fn query_cell(&self, point: Point) -> bool {
        use crate::{config::MIN_POINT, quadtree_pool::QuadtreePool};

        fn traverse(point: Point, root: usize, min: Point, pool: &QuadtreePool) -> bool {
            match &pool[root] {
                Quadtree::Subtree(subtree) => {
                    if !Renderer::point_in_box(
                        point,
                        min,
                        Point::new(
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
                    traverse(point, subtree.tl, Point::new(min.x, min.y + mid), pool)
                        || traverse(
                            point,
                            subtree.tr,
                            Point::new(min.x + mid, min.y + mid),
                            pool,
                        )
                        || traverse(point, subtree.bl, min, pool)
                        || traverse(point, subtree.br, Point::new(min.x + mid, min.y), pool)
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
