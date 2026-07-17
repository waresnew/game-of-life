use gloo_console::log;
use malachite::Integer;

use crate::{
    app::CELL_SIZE_EXP,
    image_bitmap::{ImageBitmap, Rgb},
    input_handler::Viewport,
    point::{CellPoint, ScreenPoint},
    quadtree_pool::{Quadtree, QuadtreePool},
};

pub fn render_to_image(
    viewport: &Viewport,
    root: usize,
    pool: &QuadtreePool,
    min: &CellPoint,
) -> Vec<u8> {
    let mut image = ImageBitmap::new(viewport.canvas_dims);
    render_alives(viewport, root, pool, min, &mut image);
    render_grid(viewport, &mut image);
    image.into_pixels()
}
fn render_grid(viewport: &Viewport, ans: &mut ImageBitmap) {
    const GRID_CUTOFF: u32 = CELL_SIZE_EXP - 3;
    //aka CELL_SIZE_EXP-zoom_out_exp<cutoff
    if CELL_SIZE_EXP < GRID_CUTOFF + viewport.camera.zoom_out_exp {
        return;
    }
    const GRID_COLOUR: Rgb = Rgb::new(240, 240, 240);
    let min = ScreenPoint::new(0, viewport.canvas_dims.y).to_cell(viewport);
    let max = ScreenPoint::new(viewport.canvas_dims.x, 0).to_cell(viewport);
    let mut x = min.x;
    while x <= max.x {
        let transformed_x = CellPoint::new(x.clone(), Integer::from(0))
            .to_screen(viewport)
            .x;
        for y in 0..viewport.canvas_dims.y {
            ans.fill_pixel(ScreenPoint::new(transformed_x, y), GRID_COLOUR);
        }
        x += Integer::from(1);
    }
    let mut y = min.y;
    while y <= max.y {
        let transformed_y = CellPoint::new(Integer::from(0), y.clone())
            .to_screen(viewport)
            .y;
        for x in 0..viewport.canvas_dims.x {
            ans.fill_pixel(ScreenPoint::new(x, transformed_y), GRID_COLOUR);
        }
        y += Integer::from(1);
    }
}
fn render_alives(
    viewport: &Viewport,
    id: usize,
    pool: &QuadtreePool,
    min: &CellPoint,
    ans: &mut ImageBitmap,
) {
    //2^x mult/div
    let pixels_exp_tmp = (CELL_SIZE_EXP as i32) - (viewport.camera.zoom_out_exp as i32);
    match &pool[id] {
        Quadtree::Subtree(root) => {
            let (screen_min, screen_max) = get_screen_bounding_box(viewport, min, root.height);
            if !box_intersects_canvas(viewport, screen_min, screen_max) {
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
            render_alives(
                viewport,
                root.tl,
                pool,
                &CellPoint::new(min.x.clone(), &min.y + &mid),
                ans,
            );
            render_alives(
                viewport,
                root.tr,
                pool,
                &CellPoint::new(&min.x + &mid, &min.y + &mid),
                ans,
            );
            render_alives(viewport, root.bl, pool, min, ans);
            render_alives(
                viewport,
                root.br,
                pool,
                &CellPoint::new(&min.x + &mid, min.y.clone()),
                ans,
            );
        }
        &Quadtree::Cell(alive) => {
            let (screen_min, screen_max) = get_screen_bounding_box(viewport, min, 0);
            if alive && box_intersects_canvas(viewport, screen_min, screen_max) {
                let pixels_exp = pixels_exp_tmp.max(0) as u32;
                ans.fill_cell(screen_min, pixels_exp);
            }
        }
    }
}

/// (tl,br)
fn get_screen_bounding_box(
    viewport: &Viewport,
    point: &CellPoint,
    height: u32,
) -> (ScreenPoint, ScreenPoint) {
    let cell_size =
        1 << (height as i64 + CELL_SIZE_EXP as i64 - viewport.camera.zoom_out_exp as i64).max(0); //FIXME:this will overflow
    let point1 = point.to_screen(viewport); //still low x, high y in screen space
    let point2 = ScreenPoint::new(point1.x + cell_size - 1, point1.y - (cell_size - 1));
    (
        ScreenPoint::new(point1.x, point2.y),
        ScreenPoint::new(point2.x, point1.y),
    )
}
fn box_intersects_canvas(viewport: &Viewport, min: ScreenPoint, max: ScreenPoint) -> bool {
    !(min.x >= viewport.canvas_dims.x || min.y >= viewport.canvas_dims.y || max.x < 0 || max.y < 0)
}
