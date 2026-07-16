use malachite::{Integer, base::num::arithmetic::traits::Abs};

use crate::{
    point::CellPoint,
    quadtree_pool::{ALIVE_CELL_ID, DEAD_CELL_ID, Quadtree, QuadtreePool, Subtree},
    solver::Solver,
};

impl Solver {
    pub fn toggle_cell(&mut self, point: &CellPoint) {
        while Integer::from(1) << (self.pool[self.root].as_subtree().height - 1)
            <= (&point.x).abs().max((&point.y).abs())
        {
            self.root = self.pool.add_border(self.root);
        }
        self.root = self.toggle_cell_and_return_root(point, self.root, self.get_min_point());
    }
    #[must_use]
    fn toggle_cell_and_return_root(
        &mut self,
        point: &CellPoint,
        root: usize,
        min: CellPoint,
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
                let cell_size = Integer::from(1) << height;
                if !point.in_box(
                    &min,
                    &CellPoint::new(
                        &min.x + &cell_size - Integer::from(1),
                        &min.y + &cell_size - Integer::from(1),
                    ),
                ) {
                    return root;
                }
                let mid = Integer::from(1) << (height - 1);
                let tl = self.toggle_cell_and_return_root(
                    point,
                    tl,
                    CellPoint::new(min.x.clone(), &min.y + &mid),
                );
                let tr = self.toggle_cell_and_return_root(
                    point,
                    tr,
                    CellPoint::new(&min.x + &mid, &min.y + &mid),
                );
                let bl = self.toggle_cell_and_return_root(point, bl, min.clone());
                let br = self.toggle_cell_and_return_root(
                    point,
                    br,
                    CellPoint::new(&min.x + &mid, min.y.clone()),
                );
                self.pool.join(tl, tr, bl, br, height)
            }
            Quadtree::Cell(alive) => {
                if &min == point {
                    if alive { DEAD_CELL_ID } else { ALIVE_CELL_ID }
                } else {
                    root
                }
            }
        }
    }
    /// test/benches only
    #[cfg(not(target_arch = "wasm32"))]
    pub fn query_cell(&self, point: &CellPoint) -> bool {
        fn traverse(point: &CellPoint, root: usize, min: &CellPoint, pool: &QuadtreePool) -> bool {
            match &pool[root] {
                Quadtree::Subtree(subtree) => {
                    let cell_size = Integer::from(1) << subtree.height;
                    if !point.in_box(
                        min,
                        &CellPoint::new(
                            &min.x + &cell_size - Integer::from(1),
                            &min.y + &cell_size - Integer::from(1),
                        ),
                    ) {
                        return false;
                    }
                    if subtree.count == 0 {
                        return false;
                    }
                    let mid = Integer::from(1) << (subtree.height - 1);
                    traverse(
                        point,
                        subtree.tl,
                        &CellPoint::new(min.x.clone(), &min.y + &mid),
                        pool,
                    ) || traverse(
                        point,
                        subtree.tr,
                        &CellPoint::new(&min.x + &mid, &min.y + &mid),
                        pool,
                    ) || traverse(point, subtree.bl, min, pool)
                        || traverse(
                            point,
                            subtree.br,
                            &CellPoint::new(&min.x + &mid, min.y.clone()),
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
        traverse(point, self.root, &self.get_min_point(), &self.pool)
    }
}
