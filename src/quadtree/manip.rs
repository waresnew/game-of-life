use crate::quadtree::{ALIVE_CELL_ID, DEAD_CELL_ID, Quadtree, QuadtreePool, Subtree};

use ahash::AHashMap;
use num_bigint::BigUint;

use crate::WorldPoint;

impl QuadtreePool {
    pub fn join(&mut self, tl: usize, tr: usize, bl: usize, br: usize, height: u32) -> usize {
        let count = [tl, tr, bl, br]
            .iter()
            .map(|&id| match &self.pool[id] {
                Quadtree::Subtree(subtree) => &subtree.count,
                &Quadtree::Cell(alive) => {
                    if alive {
                        &BigUint::ONE
                    } else {
                        &BigUint::ZERO
                    }
                }
            })
            .sum();
        self.insert_if_new(Quadtree::Subtree(Subtree {
            tl,
            tr,
            bl,
            br,
            height,
            count,
            ans: None,
            _private: (),
        }))
    }

    pub fn zeros(&mut self, height: u32) -> usize {
        self.load_alives(
            &mut Vec::new(),
            WorldPoint::new(0, 0),
            height,
            &mut AHashMap::new(),
        )
    }
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
        match self.pool[root] {
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
                        WorldPoint::new(min.x + (1_i64 << height), min.y + (1_i64 << height)),
                    ),
                ) {
                    return root;
                }
                let mid = 1_i64 << (height - 1);
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
                self.join(tl, tr, bl, br, height)
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
    pub fn add_border(&mut self, t: usize) -> usize {
        let &Subtree {
            tl,
            tr,
            bl,
            br,
            height,
            ..
        } = self[t].as_subtree();

        let zero = self.zeros(height - 1);
        let tl = self.join(zero, zero, zero, tl, height);
        let tr = self.join(zero, zero, tr, zero, height);
        let bl = self.join(zero, bl, zero, zero, height);
        let br = self.join(br, zero, zero, zero, height);
        self.join(tl, tr, bl, br, height + 1)
    }
    pub fn get_centre(&mut self, t: usize) -> usize {
        let &Subtree {
            tl,
            tr,
            bl,
            br,
            height,
            ..
        } = self[t].as_subtree();
        assert!(height >= 2);
        self.join(
            self[tl].as_subtree().br,
            self[tr].as_subtree().bl,
            self[bl].as_subtree().tr,
            self[br].as_subtree().tl,
            height - 1,
        )
    }
}
