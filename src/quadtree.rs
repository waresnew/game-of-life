use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::Point;

mod convert;
mod manip;

#[derive(Default, Copy, Clone, Debug)]
pub struct Quadtree {
    pub tl: u64,
    pub tr: u64,
    pub bl: u64,
    pub br: u64,
    pub height: u32,
    pub count: usize,
}
impl Hash for Quadtree {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tl.hash(state);
        self.tr.hash(state);
        self.bl.hash(state);
        self.br.hash(state);
        self.count.hash(state);
    }
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
        Self::from_alive(
            &mut Vec::new(),
            Point::new(0, 0),
            height,
            dict,
            &mut HashMap::new(),
        )
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
}
