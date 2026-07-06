use num_bigint::BigUint;

use crate::{
    quadtree_pool::Quadtree,
    renderer::{Renderer, RendererOutput, WorldPoint},
};

impl Renderer {
    pub fn to_visible_alives(
        &self,
        id: usize,
        bounds: (WorldPoint, WorldPoint),
        base_cell_size: u32,
        zoom: f64,
        min: WorldPoint,
    ) -> Vec<RendererOutput> {
        match &self.solver.pool[id] {
            Quadtree::Subtree(root) => {
                if Self::boxes_disjoint(
                    bounds,
                    (
                        min,
                        WorldPoint::new(
                            min.x + (1_i64 << root.height),
                            min.y + (1_i64 << root.height),
                        ),
                    ),
                ) {
                    return vec![];
                }
                if root.count == BigUint::ZERO {
                    return vec![];
                }
                if (1_i64 << root.height) as f64 * base_cell_size as f64 * zoom <= 1.0 {
                    if root.count > BigUint::ZERO {
                        return vec![RendererOutput {
                            min,
                            size_exp: root.height,
                        }];
                    } else {
                        return vec![];
                    }
                }
                let mid = 1_i64 << (root.height - 1);
                let tl_ans = self
                    .to_visible_alives(
                        root.tl,
                        bounds,
                        base_cell_size,
                        zoom,
                        WorldPoint::new(min.x, min.y + mid),
                    )
                    .into_iter();
                let tr_ans = self
                    .to_visible_alives(
                        root.tr,
                        bounds,
                        base_cell_size,
                        zoom,
                        WorldPoint::new(min.x + mid, min.y + mid),
                    )
                    .into_iter();
                let bl_ans = self.to_visible_alives(root.bl, bounds, base_cell_size, zoom, min);
                let br_ans = self
                    .to_visible_alives(
                        root.br,
                        bounds,
                        base_cell_size,
                        zoom,
                        WorldPoint::new(min.x + mid, min.y),
                    )
                    .into_iter();
                tl_ans.chain(tr_ans).chain(bl_ans).chain(br_ans).collect()
            }
            &Quadtree::Cell(alive) => {
                if alive {
                    vec![RendererOutput::unit_cell(min)]
                } else {
                    vec![]
                }
            }
        }
    }

    fn boxes_disjoint(
        (first1, first2): (WorldPoint, WorldPoint),
        (second1, second2): (WorldPoint, WorldPoint),
    ) -> bool {
        let (min_x1, max_x1) = (first1.x.min(first2.x), first1.x.max(first2.x));
        let (min_y1, max_y1) = (first1.y.min(first2.y), first1.y.max(first2.y));
        let (min_x2, max_x2) = (second1.x.min(second2.x), second1.x.max(second2.x));
        let (min_y2, max_y2) = (second1.y.min(second2.y), second1.y.max(second2.y));
        min_x1 > max_x2 || min_y1 > max_y2 || max_x1 < min_x2 || max_y1 < min_y2
    }
}
