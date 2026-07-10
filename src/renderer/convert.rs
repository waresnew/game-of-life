use num_bigint::BigUint;

use crate::{
    config::CELL_SIZE,
    quadtree_pool::Quadtree,
    renderer::{Renderer, WorldPoint},
};

impl Renderer {
    pub fn to_visible_alives(
        &self,
        id: usize,
        bounds: (WorldPoint, WorldPoint),
        zoom: f64,
        min: WorldPoint,
        ans: &mut Vec<i64>,
    ) {
        match &self.solver.pool[id] {
            Quadtree::Subtree(root) => {
                if Self::boxes_disjoint(
                    bounds,
                    (
                        min,
                        WorldPoint::new(min.x + (1 << root.height), min.y + (1 << root.height)),
                    ),
                ) {
                    return;
                }
                if root.count == BigUint::ZERO {
                    return;
                }
                if (1_i64 << root.height) as f64 * CELL_SIZE as f64 * zoom <= 1.0 {
                    if root.count > BigUint::ZERO {
                        ans.extend([min.x, min.y, root.height as i64]);
                    }
                    return;
                }
                let mid = 1 << (root.height - 1);
                self.to_visible_alives(
                    root.tl,
                    bounds,
                    zoom,
                    WorldPoint::new(min.x, min.y + mid),
                    ans,
                );
                self.to_visible_alives(
                    root.tr,
                    bounds,
                    zoom,
                    WorldPoint::new(min.x + mid, min.y + mid),
                    ans,
                );
                self.to_visible_alives(root.bl, bounds, zoom, min, ans);
                self.to_visible_alives(
                    root.br,
                    bounds,
                    zoom,
                    WorldPoint::new(min.x + mid, min.y),
                    ans,
                );
            }
            &Quadtree::Cell(alive) => {
                if alive && Self::point_in_box(min, bounds) {
                    ans.extend([min.x, min.y, 0]);
                }
            }
        }
    }

    pub(super) fn boxes_disjoint(
        (first1, first2): (WorldPoint, WorldPoint),
        (second1, second2): (WorldPoint, WorldPoint),
    ) -> bool {
        let (min_x1, max_x1) = (first1.x.min(first2.x), first1.x.max(first2.x));
        let (min_y1, max_y1) = (first1.y.min(first2.y), first1.y.max(first2.y));
        let (min_x2, max_x2) = (second1.x.min(second2.x), second1.x.max(second2.x));
        let (min_y2, max_y2) = (second1.y.min(second2.y), second1.y.max(second2.y));
        min_x1 > max_x2 || min_y1 > max_y2 || max_x1 < min_x2 || max_y1 < min_y2
    }

    pub(super) fn point_in_box(
        point: WorldPoint,
        (bounds1, bounds2): (WorldPoint, WorldPoint),
    ) -> bool {
        let (min_x, max_x) = (bounds1.x.min(bounds2.x), bounds1.x.max(bounds2.x));
        let (min_y, max_y) = (bounds1.y.min(bounds2.y), bounds1.y.max(bounds2.y));
        !(point.x > max_x || point.x < min_x || point.y > max_y || point.y < min_y)
    }
}
