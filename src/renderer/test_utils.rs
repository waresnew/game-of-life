use num_bigint::BigUint;
use tsify::Tsify;

use crate::{
    quadtree_pool::{Quadtree, QuadtreePool},
    renderer::{
        CellPoint, MIN_POINT, Renderer, ScreenPoint, ViewportInfo, image_bitmap::ImageBitmap,
    },
};

/// test/benches only
impl Renderer {
    /// ignores size_exp
    pub fn render_all(&mut self) -> Vec<u8> {
        self.update_viewport(
            ViewportInfo {
                canvas_dims: ScreenPoint::new(MIN_POINT.x, MIN_POINT.y),
                ..Default::default()
            }
            .into_ts()
            .unwrap(),
        );
        self.render_visible()
    }
    pub fn render_visible(&self) -> Vec<u8> {
        let mut ans = ImageBitmap::new(self.viewport_info.canvas_dims);
        self.draw_visible_alives(self.solver.root, MIN_POINT, &mut ans);
        ans.into_pixels()
    }
    pub fn query_cell(&self, point: CellPoint) -> bool {
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
