use num_bigint::BigUint;

use crate::{
    config::CELL_SIZE_EXP,
    quadtree_pool::Quadtree,
    renderer::{
        Renderer,
        controls::Point,
        image_bitmap::{ImageBitmap, Rgb},
    },
};

impl Renderer {
    fn screen_to_cell(&self, point: Point) -> Point {
        Point::new(
            ((point.x + self.viewport_info.centre.x - self.viewport_info.canvas_dims.x / 2)
                * (1 << self.viewport_info.zoom_out_exp))
                .div_euclid(1 << CELL_SIZE_EXP),
            ((point.y + self.viewport_info.centre.y - self.viewport_info.canvas_dims.y / 2)
                * -(1 << self.viewport_info.zoom_out_exp))
                .div_euclid(1 << CELL_SIZE_EXP),
        )
    }
    fn cell_to_screen(&self, point: Point) -> Point {
        let x = (point.x * (1 << CELL_SIZE_EXP)).div_euclid(1 << self.viewport_info.zoom_out_exp)
            - self.viewport_info.centre.x
            + (self.viewport_info.canvas_dims.x / 2);
        let y = (-point.y * (1 << CELL_SIZE_EXP)).div_euclid(1 << self.viewport_info.zoom_out_exp)
            - self.viewport_info.centre.y
            + (self.viewport_info.canvas_dims.y / 2);
        Point::new(x, y)
    }
    pub(super) fn draw_gridlines(&self, ans: &mut ImageBitmap) {
        const GRID_CUTOFF: u32 = 2;
        //aka CELL_SIZE_EXP-zoom_out_exp<cutoff
        if CELL_SIZE_EXP < GRID_CUTOFF + self.viewport_info.zoom_out_exp {
            return;
        }
        const GRID_COLOUR: Rgb = Rgb::new(240, 240, 240);
        let min = self.screen_to_cell(Point::new(0, self.viewport_info.canvas_dims.y));
        let max = self.screen_to_cell(Point::new(self.viewport_info.canvas_dims.x, 0));
        for x in min.x..=max.x {
            for y in 0..self.viewport_info.canvas_dims.y {
                let transformed_x = self.cell_to_screen(Point::new(x, 0)).x;
                ans.fill_pixel(Point::new(transformed_x, y), GRID_COLOUR);
            }
        }
        for y in min.y..=max.y {
            for x in 0..self.viewport_info.canvas_dims.x {
                let transformed_y = self.cell_to_screen(Point::new(0, y)).y;
                ans.fill_pixel(Point::new(x, transformed_y), GRID_COLOUR);
            }
        }
    }
    pub(super) fn render_alives(&self, id: usize, min: Point, ans: &mut ImageBitmap) {
        let screen_point = self.cell_to_screen(min);
        //2^x mult/div
        let pixels_exp_tmp = (CELL_SIZE_EXP as i32) - (self.viewport_info.zoom_out_exp as i32);
        match &self.solver.pool[id] {
            Quadtree::Subtree(root) => {
                if !self.box_intersects_canvas(
                    screen_point,
                    self.cell_to_screen(Point::new(
                        min.x + (1 << root.height) - 1,
                        min.y + (1 << root.height) - 1,
                    )),
                ) {
                    return;
                }
                if root.count == BigUint::ZERO {
                    return;
                }

                let pixels_exp = (pixels_exp_tmp + (root.height as i32)).max(0) as u32;
                if pixels_exp == 0 {
                    if root.count > BigUint::ZERO {
                        ans.fill_cell(screen_point, pixels_exp);
                    }
                    return;
                }
                let mid = 1 << (root.height - 1);
                self.render_alives(root.tl, Point::new(min.x, min.y + mid), ans);
                self.render_alives(root.tr, Point::new(min.x + mid, min.y + mid), ans);
                self.render_alives(root.bl, min, ans);
                self.render_alives(root.br, Point::new(min.x + mid, min.y), ans);
            }
            &Quadtree::Cell(alive) => {
                if alive && self.point_in_canvas(screen_point) {
                    let pixels_exp = pixels_exp_tmp.max(0) as u32;
                    ans.fill_cell(screen_point, pixels_exp);
                }
            }
        }
    }

    fn box_intersects_canvas(&self, corner1: Point, corner2: Point) -> bool {
        let min_x = corner1.x.min(corner2.x);
        let max_x = corner1.x.max(corner2.x);
        let min_y = corner1.y.min(corner2.y);
        let max_y = corner1.y.max(corner2.y);
        !(min_x >= self.viewport_info.canvas_dims.x
            || min_y >= self.viewport_info.canvas_dims.y
            || max_x < 0
            || max_y < 0)
    }

    fn point_in_canvas(&self, point: Point) -> bool {
        self.box_intersects_canvas(point, point)
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
