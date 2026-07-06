use num_bigint::BigUint;

use crate::quadtree_pool::{DEAD_CELL_ID, Quadtree, QuadtreePool, Subtree};

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
        }))
    }

    pub fn zeros(&mut self, height: u32) -> usize {
        if height == 0 {
            DEAD_CELL_ID
        } else {
            let child = self.zeros(height - 1);
            self.join(child, child, child, child, height)
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
