use std::hash::{Hash, Hasher};

use ahash::{AHashMap, AHasher};

use crate::{Point, utils::update_dict};

mod convert;
mod manip;

#[derive(Default, Copy, Clone, Debug)]
pub struct Quadtree {
    pub tl: u64,
    pub tr: u64,
    pub bl: u64,
    pub br: u64,
    pub height: u32,
    pub hash: u64,
    _private: (),
}
impl Hash for Quadtree {
    //TODO: not needed?
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}
impl Quadtree {
    pub fn join(
        tl: u64,
        tr: u64,
        bl: u64,
        br: u64,
        height: u32,
        dict: &mut AHashMap<u64, Quadtree>,
    ) -> Self {
        let ret = Self {
            tl,
            tr,
            bl,
            br,
            height,
            hash: Self::calc_hash(tl, tr, bl, br),
            _private: (),
        };
        update_dict(ret, dict);
        ret
    }
    fn calc_hash(tl: u64, tr: u64, bl: u64, br: u64) -> u64 {
        let mut hasher = AHasher::default();
        tl.hash(&mut hasher);
        tr.hash(&mut hasher);
        bl.hash(&mut hasher);
        br.hash(&mut hasher);
        hasher.finish()
    }
    pub fn zeros(height: u32, dict: &mut AHashMap<u64, Quadtree>) -> Self {
        let ret = Self::from_alive(
            &mut Vec::new(),
            Point::new(0, 0),
            height,
            dict,
            &mut AHashMap::new(),
        );
        update_dict(ret, dict);
        ret
    }
    pub fn alive_cell(dict: &mut AHashMap<u64, Quadtree>) -> Self {
        Self::join(1, 1, 1, 1, 0, dict)
    }
    pub fn dead_cell(dict: &mut AHashMap<u64, Quadtree>) -> Self {
        Self::join(0, 0, 0, 0, 0, dict)
    }
}
