use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::update_dict;

#[derive(Default, Copy, Clone, Debug)]
pub struct Quadtree {
    pub tl: u64,
    pub tr: u64,
    pub bl: u64,
    pub br: u64,
    pub height: u32,
    pub count: usize,
}
impl Quadtree {
    pub fn calc_hash(self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.tl.hash(&mut hasher);
        self.tr.hash(&mut hasher);
        self.bl.hash(&mut hasher);
        self.br.hash(&mut hasher);
        self.count.hash(&mut hasher); //to distinguish dead/alive
        hasher.finish()
    }
    pub fn zeros(height: u32, dict: &mut HashMap<u64, Quadtree>) -> Self {
        Self::from_alive(&mut Vec::new(), (0, 0), height, dict, &mut HashMap::new())
    }
    pub fn alive_cell() -> Self {
        Self {
            count: 1,
            ..Default::default()
        }
    }
    pub fn dead_cell() -> Self {
        Self::default()
    }
    pub fn from_alive(
        alive: &mut Vec<(i64, i64)>,
        start_pos: (i64, i64),
        height: u32,
        dict: &mut HashMap<u64, Quadtree>,
        dp: &mut HashMap<u64, Quadtree>,
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
        fn calc_hash(alive: &mut Vec<(i64, i64)>, height: u32) -> u64 {
            alive.sort_unstable();
            let mut hasher = DefaultHasher::new();
            for x in alive {
                x.hash(&mut hasher);
            }
            height.hash(&mut hasher);
            hasher.finish()
        }
        let hash = calc_hash(alive, height);
        if !dp.contains_key(&hash) {
            let mid_x = start_pos.0 + 2_i64.pow(height - 1);
            let mid_y = start_pos.1 + 2_i64.pow(height - 1);
            let mut tl_alive = Vec::new();
            let mut tr_alive = Vec::new();
            let mut bl_alive = Vec::new();
            let mut br_alive = Vec::new();
            for &mut (x, y) in alive {
                if x < mid_x && y >= mid_y {
                    tl_alive.push((x, y));
                } else if x >= mid_x && y >= mid_y {
                    tr_alive.push((x, y));
                } else if x < mid_x && y < mid_y {
                    bl_alive.push((x, y));
                } else if x >= mid_x && y < mid_y {
                    br_alive.push((x, y));
                } else {
                    unreachable!("cell not placed in quadrant");
                }
            }
            let tl = Self::from_alive(&mut tl_alive, (start_pos.0, mid_y), height - 1, dict, dp);
            let tr = Self::from_alive(&mut tr_alive, (mid_x, mid_y), height - 1, dict, dp);
            let bl = Self::from_alive(&mut bl_alive, start_pos, height - 1, dict, dp);
            let br = Self::from_alive(&mut br_alive, (mid_x, start_pos.1), height - 1, dict, dp);
            dp.insert(
                hash,
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
        dp[&hash]
    }
    pub fn to_array(self, dict: &HashMap<u64, Quadtree>) -> Vec<u8> {
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
        dict: &HashMap<u64, Quadtree>,
        dp: &mut HashMap<u64, Vec<(i64, i64)>>,
    ) -> Vec<(i64, i64)> {
        if self.height == 0 {
            assert!(self.count <= 1);
            if self.count == 1 {
                return vec![(0, 0)];
            } else {
                return vec![];
            }
        }
        if !dp.contains_key(&self.calc_hash()) {
            let mid = 2_i64.pow(self.height - 1);
            let tl_ans = dict[&self.tl]
                .to_alive(dict, dp)
                .into_iter()
                .map(|(x, y)| (x, y + mid));
            let tr_ans = dict[&self.tr]
                .to_alive(dict, dp)
                .into_iter()
                .map(|(x, y)| (x + mid, y + mid));
            let bl_ans = dict[&self.bl].to_alive(dict, dp);
            let br_ans = dict[&self.br]
                .to_alive(dict, dp)
                .into_iter()
                .map(|(x, y)| (x + mid, y));
            let ans = tl_ans.chain(tr_ans).chain(bl_ans).chain(br_ans).collect();
            dp.insert(self.calc_hash(), ans);
        }
        dp[&self.calc_hash()].clone()
    }
}
