use gloo_console::log;
use num_bigint::{BigInt, BigUint};
use num_integer::Integer;

use crate::{
    quadtree_pool::Quadtree,
    renderer::{
        CELL_SIZE_EXP, CellPoint, Renderer, ScreenPoint, WorldPoint,
        image_bitmap::{ImageBitmap, Rgb},
    },
};

impl Renderer {
    pub(super) fn screen_to_cell(&self, point: ScreenPoint) -> CellPoint {
        let world = self.screen_to_world(point);
        CellPoint::new(
            world.x.div_floor(&(BigInt::from(1) << CELL_SIZE_EXP)),
            world.y.div_floor(&(BigInt::from(1) << CELL_SIZE_EXP)),
        )
    }
    pub(super) fn screen_to_world(&self, point: ScreenPoint) -> WorldPoint {
        WorldPoint::new(
            (BigInt::from(point.x) + &self.camera.centre.x - self.viewport_info.canvas_dims.x / 2)
                * (&(BigInt::from(1) << self.camera.zoom_out_exp)),
            (BigInt::from(point.y) + &self.camera.centre.y - self.viewport_info.canvas_dims.y / 2)
                * -(&(BigInt::from(1) << self.camera.zoom_out_exp)),
        )
    }
    //OPTIMIZE:
    //TODO: i rpefer min.x over min_x cloning, also don't recompute 1<<etc
    pub(super) fn cell_to_screen(&self, point: &CellPoint) -> ScreenPoint {
        let x = (&point.x * (BigInt::from(1) << CELL_SIZE_EXP))
            .div_floor(&(BigInt::from(1) << self.camera.zoom_out_exp))
            - &self.camera.centre.x
            + (self.viewport_info.canvas_dims.x / 2);
        let y = (-&point.y * (BigInt::from(1) << CELL_SIZE_EXP))
            .div_floor(&(BigInt::from(1) << self.camera.zoom_out_exp))
            - &self.camera.centre.y
            + (self.viewport_info.canvas_dims.y / 2);
        fn into_clamped_i64(n: BigInt) -> i64 {
            if n < BigInt::from(i64::MIN) {
                i64::MIN
            } else if n > BigInt::from(i64::MAX) {
                i64::MAX
            } else {
                i64::try_from(n).unwrap()
            }
        }
        ScreenPoint::new(into_clamped_i64(x), into_clamped_i64(y))
    }
    pub(super) fn draw_grid(&self, ans: &mut ImageBitmap) {
        const GRID_CUTOFF: u32 = CELL_SIZE_EXP - 3;
        //aka CELL_SIZE_EXP-zoom_out_exp<cutoff
        if CELL_SIZE_EXP < GRID_CUTOFF + self.camera.zoom_out_exp {
            return;
        }
        const GRID_COLOUR: Rgb = Rgb::new(240, 240, 240);
        let min = self.screen_to_cell(ScreenPoint::new(0, self.viewport_info.canvas_dims.y));
        let max = self.screen_to_cell(ScreenPoint::new(self.viewport_info.canvas_dims.x, 0));
        let mut x = min.x;
        while x <= max.x {
            for y in 0..self.viewport_info.canvas_dims.y {
                let transformed_x = self
                    .cell_to_screen(&CellPoint::new(x.clone(), BigInt::from(0)))
                    .x;
                ans.fill_pixel(ScreenPoint::new(transformed_x, y), GRID_COLOUR);
            }
            x += 1;
        }
        let mut y = min.y;
        while y <= max.y {
            for x in 0..self.viewport_info.canvas_dims.x {
                let transformed_y = self
                    .cell_to_screen(&CellPoint::new(BigInt::from(0), y.clone()))
                    .y;
                ans.fill_pixel(ScreenPoint::new(x, transformed_y), GRID_COLOUR);
            }
            y += 1;
        }
    }
    pub(super) fn draw_visible_alives(&self, id: usize, min: &CellPoint, ans: &mut ImageBitmap) {
        //2^x mult/div
        let pixels_exp_tmp = (CELL_SIZE_EXP as i32) - (self.camera.zoom_out_exp as i32);
        match &self.solver.pool[id] {
            Quadtree::Subtree(root) => {
                let (screen_min, screen_max) = self.get_screen_bounding_box(min, root.height);
                if !self.box_intersects_canvas(screen_min, screen_max) {
                    return;
                }
                if root.count == BigUint::ZERO {
                    return;
                }

                let pixels_exp = (pixels_exp_tmp + (root.height as i32)).max(0) as u32;
                if pixels_exp == 0 {
                    if root.count > BigUint::ZERO {
                        ans.fill_cell(screen_min, pixels_exp);
                    }
                    return;
                }
                let mid = BigInt::from(1) << (root.height - 1);
                let CellPoint { x: min_x, y: min_y } = min.clone();
                self.draw_visible_alives(
                    root.tl,
                    &CellPoint::new(min_x.clone(), &min_y + &mid),
                    ans,
                );
                self.draw_visible_alives(
                    root.tr,
                    &CellPoint::new(&min_x + &mid, &min_y + &mid),
                    ans,
                );
                self.draw_visible_alives(root.bl, min, ans);
                self.draw_visible_alives(
                    root.br,
                    &CellPoint::new(&min_x + &mid, min_y.clone()),
                    ans,
                );
            }
            &Quadtree::Cell(alive) => {
                let (screen_min, screen_max) = self.get_screen_bounding_box(min, 0);
                if alive && self.box_intersects_canvas(screen_min, screen_max) {
                    let pixels_exp = pixels_exp_tmp.max(0) as u32;
                    ans.fill_cell(screen_min, pixels_exp);
                }
            }
        }
    }

    fn get_screen_bounding_box(
        &self,
        point: &CellPoint,
        size_exp: u32,
    ) -> (ScreenPoint, ScreenPoint) {
        let point1 = self.cell_to_screen(point);
        let point2 = self.cell_to_screen(&CellPoint::new(
            &point.x + (BigInt::from(1) << size_exp),
            &point.y + (BigInt::from(1) << size_exp),
        ));
        (
            ScreenPoint::new(point1.x.min(point2.x), point1.y.min(point2.y)),
            ScreenPoint::new(point1.x.max(point2.x), point1.y.max(point2.y)),
        )
    }
    fn box_intersects_canvas(&self, min: ScreenPoint, max: ScreenPoint) -> bool {
        !(min.x >= self.viewport_info.canvas_dims.x
            || min.y >= self.viewport_info.canvas_dims.y
            || max.x < 0
            || max.y < 0)
    }

    pub(super) fn point_in_box(
        point: &CellPoint,
        box_min: &CellPoint,
        box_max: &CellPoint,
    ) -> bool {
        !(point.x > box_max.x || point.x < box_min.x || point.y > box_max.y || point.y < box_min.y)
    }
}
