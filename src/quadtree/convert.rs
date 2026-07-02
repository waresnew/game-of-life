use std::hash::{Hash, Hasher};

use ahash::{AHashMap, AHasher};

use crate::{
    Point,
    quadtree::{QuadtreeKey, QuadtreePool},
};

impl QuadtreePool {
    pub fn load_alives(
        &mut self,
        alive: &mut Vec<Point>,
        start_pos: Point,
        height: u32,
        dp: &mut AHashMap<u64, usize>,
    ) -> usize {
        if height == 0 {
            assert!(alive.len() <= 1, "alive len: {}", alive.len());
            if alive.len() == 1 {
                return self.alive_cell();
            } else {
                return self.dead_cell();
            }
        }
        /// will mutate the arg to avoid a clone
        fn calc_key(alive: &mut Vec<Point>, height: u32) -> u64 {
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
            let mid_x = start_pos.x + (1 << (height - 1));
            let mid_y = start_pos.y + (1 << (height - 1));
            let mut tl_alive = Vec::new();
            let mut tr_alive = Vec::new();
            let mut bl_alive = Vec::new();
            let mut br_alive = Vec::new();
            for &mut Point { x, y } in alive {
                if x < mid_x && y >= mid_y {
                    tl_alive.push(Point::new(x, y));
                } else if x >= mid_x && y >= mid_y {
                    tr_alive.push(Point::new(x, y));
                } else if x < mid_x && y < mid_y {
                    bl_alive.push(Point::new(x, y));
                } else if x >= mid_x && y < mid_y {
                    br_alive.push(Point::new(x, y));
                } else {
                    unreachable!("cell not placed in quadrant");
                }
            }
            let tl = self.load_alives(
                &mut tl_alive,
                Point::new(start_pos.x, mid_y),
                height - 1,
                dp,
            );
            let tr = self.load_alives(&mut tr_alive, Point::new(mid_x, mid_y), height - 1, dp);
            let bl = self.load_alives(&mut bl_alive, start_pos, height - 1, dp);
            let br = self.load_alives(
                &mut br_alive,
                Point::new(mid_x, start_pos.y),
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
        let root = &self[id];
        if root.height == 0 {
            return vec![vec![root.tl as u8]];
        }
        let tl = self.to_array(root.tl);
        let tr = self.to_array(root.tr);
        let bl = self.to_array(root.bl);
        let br = self.to_array(root.br);
        fn block_concat(left: Vec<Vec<u8>>, right: Vec<Vec<u8>>) -> impl Iterator<Item = Vec<u8>> {
            left.into_iter().zip(right).map(|(x, y)| [x, y].concat())
        }
        let top = block_concat(tl, tr);
        let bottom = block_concat(bl, br);
        top.chain(bottom).collect()
    }
    pub fn to_alive(&self, id: usize, dp: &mut AHashMap<QuadtreeKey, Vec<Point>>) -> Vec<Point> {
        let root = &self[id];
        if root.height == 0 {
            assert!(
                root.tl == root.tr
                    && root.tr == root.bl
                    && root.bl == root.br
                    && (root.tl == 1 || root.tl == 0)
            );
            if root.tl == 1 {
                return vec![Point::new(0, 0)];
            } else {
                return vec![];
            }
        }
        if !dp.contains_key(&root.get_key()) {
            let mid = 1 << (root.height - 1);
            let tl_ans = self
                .to_alive(root.tl, dp)
                .into_iter()
                .map(|Point { x, y }| Point::new(x, y + mid));
            let tr_ans = self
                .to_alive(root.tr, dp)
                .into_iter()
                .map(|Point { x, y }| Point::new(x + mid, y + mid));
            let bl_ans = self.to_alive(root.bl, dp);
            let br_ans = self
                .to_alive(root.br, dp)
                .into_iter()
                .map(|Point { x, y }| Point::new(x + mid, y));
            let ans = tl_ans.chain(tr_ans).chain(bl_ans).chain(br_ans).collect();
            dp.insert(root.get_key(), ans);
        }
        dp[&root.get_key()].clone()
    }
}
