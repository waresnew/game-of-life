use std::hash::{Hash, Hasher};

use ahash::{AHashMap, AHasher};

use crate::{Point, quadtree::Quadtree};

impl Quadtree {
    pub fn from_alive(
        alive: &mut Vec<Point>,
        start_pos: Point,
        height: u32,
        dict: &mut AHashMap<u64, Quadtree>,
        dp: &mut AHashMap<u64, u64>,
    ) -> u64 {
        if height == 0 {
            assert!(alive.len() <= 1, "alive len: {}", alive.len());
            if alive.len() == 1 {
                return Self::alive_cell(dict);
            } else {
                return Self::dead_cell(dict);
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
            let tl = Self::from_alive(
                &mut tl_alive,
                Point::new(start_pos.x, mid_y),
                height - 1,
                dict,
                dp,
            );
            let tr = Self::from_alive(
                &mut tr_alive,
                Point::new(mid_x, mid_y),
                height - 1,
                dict,
                dp,
            );
            let bl = Self::from_alive(&mut bl_alive, start_pos, height - 1, dict, dp);
            let br = Self::from_alive(
                &mut br_alive,
                Point::new(mid_x, start_pos.y),
                height - 1,
                dict,
                dp,
            );
            dp.insert(key, Self::join(tl, tr, bl, br, height, dict));
        }
        dp[&key]
    }
    pub fn to_string(&self, dict: &AHashMap<u64, Quadtree>) -> String {
        let grid = self.to_array(dict);
        grid.iter()
            .map(|row| {
                row.iter()
                    .map(|x| if *x == 1 { "*" } else { "." })
                    .collect()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
    fn to_array(&self, dict: &AHashMap<u64, Quadtree>) -> Vec<Vec<u8>> {
        if self.height == 0 {
            return vec![vec![self.tl as u8]];
        }
        let tl = dict[&self.tl].to_array(dict);
        let tr = dict[&self.tr].to_array(dict);
        let bl = dict[&self.bl].to_array(dict);
        let br = dict[&self.br].to_array(dict);
        fn block_concat(left: Vec<Vec<u8>>, right: Vec<Vec<u8>>) -> impl Iterator<Item = Vec<u8>> {
            left.into_iter().zip(right).map(|(x, y)| [x, y].concat())
        }
        let top = block_concat(tl, tr);
        let bottom = block_concat(bl, br);
        top.chain(bottom).collect()
    }
    pub fn to_alive(
        &self,
        dict: &AHashMap<u64, Quadtree>,
        dp: &mut AHashMap<u64, Vec<Point>>,
    ) -> Vec<Point> {
        if self.height == 0 {
            assert!(
                self.tl == self.tr
                    && self.tr == self.bl
                    && self.bl == self.br
                    && (self.tl == 1 || self.tl == 0)
            );
            if self.tl == 1 {
                return vec![Point::new(0, 0)];
            } else {
                return vec![];
            }
        }
        if !dp.contains_key(&self.hash) {
            let mid = 1 << (self.height - 1);
            let tl_ans = dict[&self.tl]
                .to_alive(dict, dp)
                .into_iter()
                .map(|Point { x, y }| Point::new(x, y + mid));
            let tr_ans = dict[&self.tr]
                .to_alive(dict, dp)
                .into_iter()
                .map(|Point { x, y }| Point::new(x + mid, y + mid));
            let bl_ans = dict[&self.bl].to_alive(dict, dp);
            let br_ans = dict[&self.br]
                .to_alive(dict, dp)
                .into_iter()
                .map(|Point { x, y }| Point::new(x + mid, y));
            let ans = tl_ans.chain(tr_ans).chain(bl_ans).chain(br_ans).collect();
            dp.insert(self.hash, ans);
        }
        dp[&self.hash].clone()
    }
}
