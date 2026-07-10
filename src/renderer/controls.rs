use crate::{
    quadtree_pool::{ALIVE_CELL_ID, DEAD_CELL_ID, Quadtree, Subtree},
    renderer::{Renderer, WorldPoint},
};

impl Renderer {
    fn point_in_box(point: WorldPoint, (bounds1, bounds2): (WorldPoint, WorldPoint)) -> bool {
        let (min_x, max_x) = (bounds1.x.min(bounds2.x), bounds1.x.max(bounds2.x));
        let (min_y, max_y) = (bounds1.y.min(bounds2.y), bounds1.y.max(bounds2.y));
        !(point.x > max_x || point.x < min_x || point.y > max_y || point.y < min_y)
    }
    #[must_use]
    pub fn toggle_cell_and_return_root(
        &mut self,
        point: WorldPoint,
        root: usize,
        min: WorldPoint,
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
                    (
                        min,
                        WorldPoint::new(min.x + (1 << height), min.y + (1 << height)),
                    ),
                ) {
                    return root;
                }
                let mid = 1 << (height - 1);
                let tl = self.toggle_cell_and_return_root(
                    point,
                    tl,
                    WorldPoint::new(min.x, min.y + mid),
                );
                let tr = self.toggle_cell_and_return_root(
                    point,
                    tr,
                    WorldPoint::new(min.x + mid, min.y + mid),
                );
                let bl = self.toggle_cell_and_return_root(point, bl, min);
                let br = self.toggle_cell_and_return_root(
                    point,
                    br,
                    WorldPoint::new(min.x + mid, min.y),
                );
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
