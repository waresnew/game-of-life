use std::hash::{Hash, Hasher};

use ahash::{AHashMap, AHasher};

use crate::{Point, quadtree::Quadtree, utils::update_dict};

impl Quadtree {
    pub fn from_alive(
        alive: &mut Vec<Point>,
        start_pos: Point,
        height: u32,
        dict: &mut AHashMap<u64, Quadtree>,
        dp: &mut AHashMap<u64, Quadtree>,
    ) -> Quadtree {
        if height == 0 {
            assert!(alive.len() <= 1, "alive len: {}", alive.len());
            if alive.len() == 1 {
                return Self::alive_cell();
            } else {
                return Self::dead_cell();
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
            let mid_x = start_pos.x + 2_i64.pow(height - 1);
            let mid_y = start_pos.y + 2_i64.pow(height - 1);
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
            dp.insert(
                key,
                Self {
                    tl: update_dict(tl, dict),
                    tr: update_dict(tr, dict),
                    bl: update_dict(bl, dict),
                    br: update_dict(br, dict),
                    count: tl.count + tr.count + bl.count + br.count,
                    height,
                },
            );
        }
        dp[&key]
    }
    pub fn to_array(self, dict: &AHashMap<u64, Quadtree>) -> Vec<u8> {
        if self.height == 0 {
            return vec![self.count as u8];
        }
        let tl = dict[&self.tl].to_array(dict);
        let tr = dict[&self.tr].to_array(dict);
        let bl = dict[&self.bl].to_array(dict);
        let br = dict[&self.br].to_array(dict);
        fn block_concat(left: &[u8], right: &[u8], width: usize) -> impl Iterator<Item = u8> {
            left.chunks_exact(width)
                .zip(right.chunks_exact(width))
                .flat_map(|(x, y)| [x, y].concat())
        }
        let width = 2_usize.pow(self.height - 1);
        let top = block_concat(&tl, &tr, width);
        let bottom = block_concat(&bl, &br, width);
        top.chain(bottom).collect()
    }
    pub fn to_alive(
        self,
        dict: &AHashMap<u64, Quadtree>,
        dp: &mut AHashMap<u64, Vec<Point>>,
    ) -> Vec<Point> {
        if self.height == 0 {
            assert!(self.count <= 1);
            if self.count == 1 {
                return vec![Point::new(0, 0)];
            } else {
                return vec![];
            }
        }
        if !dp.contains_key(&self.calc_hash()) {
            let mid = 2_i64.pow(self.height - 1);
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
            dp.insert(self.calc_hash(), ans);
        }
        dp[&self.calc_hash()].clone()
    }
}
