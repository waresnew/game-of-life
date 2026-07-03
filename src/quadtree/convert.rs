use std::hash::{Hash, Hasher};

use ahash::{AHashMap, AHasher};
use num_bigint::BigUint;

use crate::{
    WorldPoint,
    quadtree::{ALIVE_CELL_ID, DEAD_CELL_ID, Quadtree, QuadtreePool},
};

impl QuadtreePool {
    pub fn load_alives(
        &mut self,
        alive: &mut Vec<WorldPoint>,
        start_pos: WorldPoint,
        height: u32,
        dp: &mut AHashMap<u64, usize>,
    ) -> usize {
        if height == 0 {
            assert!(alive.len() <= 1, "alive len: {}", alive.len());
            if alive.len() == 1 {
                return ALIVE_CELL_ID;
            } else {
                return DEAD_CELL_ID;
            }
        }
        /// will mutate the arg to avoid a clone
        fn calc_key(alive: &mut Vec<WorldPoint>, height: u32) -> u64 {
            alive.sort_unstable();
            let mut hasher = AHasher::default();
            for x in alive {
                x.hash(&mut hasher);
            }
            height.hash(&mut hasher);
            hasher.finish()
        }
        let key = calc_key(alive, height);
        if !dp.contains_key(&key) {
            let mid_x = start_pos.x + (1_i64 << (height - 1));
            let mid_y = start_pos.y + (1_i64 << (height - 1));
            let mut tl_alive = Vec::new();
            let mut tr_alive = Vec::new();
            let mut bl_alive = Vec::new();
            let mut br_alive = Vec::new();
            for &mut WorldPoint { x, y } in alive {
                if x < mid_x && y >= mid_y {
                    tl_alive.push(WorldPoint::new(x, y));
                } else if x >= mid_x && y >= mid_y {
                    tr_alive.push(WorldPoint::new(x, y));
                } else if x < mid_x && y < mid_y {
                    bl_alive.push(WorldPoint::new(x, y));
                } else if x >= mid_x && y < mid_y {
                    br_alive.push(WorldPoint::new(x, y));
                } else {
                    unreachable!("cell not placed in quadrant");
                }
            }
            let tl = self.load_alives(
                &mut tl_alive,
                WorldPoint::new(start_pos.x, mid_y),
                height - 1,
                dp,
            );
            let tr = self.load_alives(&mut tr_alive, WorldPoint::new(mid_x, mid_y), height - 1, dp);
            let bl = self.load_alives(&mut bl_alive, start_pos, height - 1, dp);
            let br = self.load_alives(
                &mut br_alive,
                WorldPoint::new(mid_x, start_pos.y),
                height - 1,
                dp,
            );
            dp.insert(key, self.join(tl, tr, bl, br, height));
        }
        dp[&key]
    }
    pub fn to_string(&self, id: usize) -> String {
        let grid = self.to_array(id);
        grid.iter()
            .map(|row| {
                row.iter()
                    .map(|x| if *x == 1 { "*" } else { "." })
                    .collect()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
    fn to_array(&self, id: usize) -> Vec<Vec<u8>> {
        match &self[id] {
            Quadtree::Subtree(root) => {
                let tl = self.to_array(root.tl);
                let tr = self.to_array(root.tr);
                let bl = self.to_array(root.bl);
                let br = self.to_array(root.br);
                fn block_concat(
                    left: Vec<Vec<u8>>,
                    right: Vec<Vec<u8>>,
                ) -> impl Iterator<Item = Vec<u8>> {
                    left.into_iter().zip(right).map(|(x, y)| [x, y].concat())
                }
                let top = block_concat(tl, tr);
                let bottom = block_concat(bl, br);
                top.chain(bottom).collect()
            }
            &Quadtree::Cell(alive) => vec![vec![if alive { 1 } else { 0 }]],
        }
    }
    pub fn to_alive(
        &self,
        id: usize,
        bounds: (WorldPoint, WorldPoint),
        base_cell_size: u32,
        zoom: f64,
        min: WorldPoint,
    ) -> Vec<WorldPoint> {
        match &self[id] {
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
                if (1_i64 << root.height) as f64 * base_cell_size as f64 * zoom <= 1.0 {
                    if root.count > BigUint::ZERO {
                        return vec![min];
                    } else {
                        return vec![];
                    }
                }
                let mid = 1_i64 << (root.height - 1);
                let tl_ans = self
                    .to_alive(
                        root.tl,
                        bounds,
                        base_cell_size,
                        zoom,
                        WorldPoint::new(min.x, min.y + mid),
                    )
                    .into_iter();
                let tr_ans = self
                    .to_alive(
                        root.tr,
                        bounds,
                        base_cell_size,
                        zoom,
                        WorldPoint::new(min.x + mid, min.y + mid),
                    )
                    .into_iter();
                let bl_ans = self.to_alive(root.bl, bounds, base_cell_size, zoom, min);
                let br_ans = self
                    .to_alive(
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
                    vec![min]
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
