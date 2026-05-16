use std::hash::{Hash, Hasher};

use ahash::{AHashMap, AHasher};

use crate::Point;

mod convert;
mod manip;

#[derive(Default, Debug)]
pub struct Quadtree {
    pub tl: u64,
    pub tr: u64,
    pub bl: u64,
    pub br: u64,
    pub height: u32,
    pub hash: u64,
    pub ans: Option<u64>,
    pub keep: bool,
    _private: (),
}
impl PartialEq for Quadtree {
    fn eq(&self, other: &Self) -> bool {
        self.tl == other.tl && self.tr == other.tr && self.bl == other.bl && self.br == other.br
    }
}
impl Eq for Quadtree {}

impl Quadtree {
    pub fn join(
        tl: u64,
        tr: u64,
        bl: u64,
        br: u64,
        height: u32,
        dict: &mut AHashMap<u64, Quadtree>,
    ) -> u64 {
        let hash = Self::calc_hash(tl, tr, bl, br);
        if !dict.contains_key(&hash) {
            let ans = Self {
                tl,
                tr,
                bl,
                br,
                height,
                hash,
                ans: None,
                keep: false,
                _private: (),
            };
            dict.insert(ans.hash, ans);
        }
        hash
    }
    fn calc_hash(tl: u64, tr: u64, bl: u64, br: u64) -> u64 {
        let mut hasher = AHasher::default();
        tl.hash(&mut hasher);
        tr.hash(&mut hasher);
        bl.hash(&mut hasher);
        br.hash(&mut hasher);
        hasher.finish()
    }
    pub fn zeros(height: u32, dict: &mut AHashMap<u64, Quadtree>) -> u64 {
        Self::from_alive(
            &mut Vec::new(),
            Point::new(0, 0),
            height,
            dict,
            &mut AHashMap::new(),
        )
    }
    pub fn alive_cell(dict: &mut AHashMap<u64, Quadtree>) -> u64 {
        Self::join(1, 1, 1, 1, 0, dict)
    }
    pub fn dead_cell(dict: &mut AHashMap<u64, Quadtree>) -> u64 {
        Self::join(0, 0, 0, 0, 0, dict)
    }
}
