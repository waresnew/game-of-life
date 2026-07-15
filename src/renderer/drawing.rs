use gloo_console::log;
use malachite::{
    Integer,
    base::{num::arithmetic::traits::DivRound, rounding_modes::RoundingMode},
};

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
        let cell_size = Integer::from(1) << CELL_SIZE_EXP;
        CellPoint::new(
            world.x.div_round(&cell_size, RoundingMode::Floor).0,
            world.y.div_round(&cell_size, RoundingMode::Floor).0,
        )
    }
    pub(super) fn screen_to_world(&self, point: ScreenPoint) -> WorldPoint {
        let zoom_out = Integer::from(1) << self.camera.zoom_out_exp;
        WorldPoint::new(
            (Integer::from(point.x) + &self.camera.centre.x
                - Integer::from(self.viewport_info.canvas_dims.x / 2))
                * &zoom_out,
            (Integer::from(point.y) + &self.camera.centre.y
                - Integer::from(self.viewport_info.canvas_dims.y / 2))
                * -&zoom_out,
        )
    }
    pub(super) fn cell_to_screen(&self, point: &CellPoint) -> ScreenPoint {
        let cell_size = Integer::from(1) << CELL_SIZE_EXP;
        let zoom_out = Integer::from(1) << self.camera.zoom_out_exp;
        let x = (&point.x * &cell_size)
            .div_round(&zoom_out, RoundingMode::Floor)
            .0
            - &self.camera.centre.x
            + Integer::from(self.viewport_info.canvas_dims.x / 2);
        let y = (-&point.y * &cell_size)
            .div_round(&zoom_out, RoundingMode::Floor)
            .0
            - &self.camera.centre.y
            + Integer::from(self.viewport_info.canvas_dims.y / 2);
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
            let transformed_x = self
                .cell_to_screen(&CellPoint::new(x.clone(), Integer::from(0)))
                .x;
            for y in 0..self.viewport_info.canvas_dims.y {
                ans.fill_pixel(ScreenPoint::new(transformed_x, y), GRID_COLOUR);
            }
            x += Integer::from(1);
        }
        let mut y = min.y;
        while y <= max.y {
            let transformed_y = self
                .cell_to_screen(&CellPoint::new(Integer::from(0), y.clone()))
                .y;
            for x in 0..self.viewport_info.canvas_dims.x {
                ans.fill_pixel(ScreenPoint::new(x, transformed_y), GRID_COLOUR);
            }
            y += Integer::from(1);
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
                if root.count == 0 {
                    return;
                }

                let pixels_exp = (pixels_exp_tmp + (root.height as i32)).max(0) as u32;
                if pixels_exp == 0 {
                    if root.count > 0 {
                        ans.fill_cell(screen_min, pixels_exp);
                    }
                    return;
                }
                let mid = Integer::from(1) << (root.height - 1);
                self.draw_visible_alives(
                    root.tl,
                    &CellPoint::new(min.x.clone(), &min.y + &mid),
                    ans,
                );
                self.draw_visible_alives(
                    root.tr,
                    &CellPoint::new(&min.x + &mid, &min.y + &mid),
                    ans,
                );
                self.draw_visible_alives(root.bl, min, ans);
                self.draw_visible_alives(
                    root.br,
                    &CellPoint::new(&min.x + &mid, min.y.clone()),
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
        let cell_size = Integer::from(1) << size_exp;
        let point1 = self.cell_to_screen(point);
        let point2 = self.cell_to_screen(&CellPoint::new(
            &point.x + &cell_size,
            &point.y + &cell_size,
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
