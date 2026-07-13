use num_bigint::{BigInt, BigUint};

use crate::{
    quadtree_pool::Quadtree,
    renderer::{CellPoint, Renderer, image_bitmap::ImageBitmap},
};

/// test/benches only
impl Renderer {
    pub fn render_visible(&self) -> Vec<u8> {
        let mut ans = ImageBitmap::new(self.viewport_info.canvas_dims);
        self.draw_visible_alives(self.solver.root, &self.solver.get_min_point(), &mut ans);
        ans.into_pixels()
    }
    pub fn query_cell(&self, point: &CellPoint) -> bool {
        use crate::quadtree_pool::QuadtreePool;

        fn traverse(point: &CellPoint, root: usize, min: &CellPoint, pool: &QuadtreePool) -> bool {
            match &pool[root] {
                Quadtree::Subtree(subtree) => {
                    if !Renderer::point_in_box(
                        point,
                        min,
                        &CellPoint::new(
                            &min.x + (BigInt::from(1) << subtree.height) - 1,
                            &min.y + (BigInt::from(1) << subtree.height) - 1,
                        ),
                    ) {
                        return false;
                    }
                    if subtree.count == BigUint::ZERO {
                        return false;
                    }
                    let mid = BigInt::from(1) << (subtree.height - 1);
                    let CellPoint { x: min_x, y: min_y } = min.clone();
                    traverse(
                        point,
                        subtree.tl,
                        &CellPoint::new(min_x.clone(), &min_y + &mid),
                        pool,
                    ) || traverse(
                        point,
                        subtree.tr,
                        &CellPoint::new(&min_x + &mid, &min_y + &mid),
                        pool,
                    ) || traverse(point, subtree.bl, min, pool)
                        || traverse(
                            point,
                            subtree.br,
                            &CellPoint::new(&min_x + &mid, min_y.clone()),
                            pool,
                        )
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
        traverse(
            point,
            self.solver.root,
            &self.solver.get_min_point(),
            &self.solver.pool,
        )
    }
}
