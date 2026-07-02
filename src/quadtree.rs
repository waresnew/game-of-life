use std::ops::Index;

use ahash::AHashMap;

use crate::Point;

mod convert;

type QuadtreeKey = (usize, usize, usize, usize);
#[derive(Debug)]
pub struct QuadtreePool {
    dict: AHashMap<QuadtreeKey, usize>,
    pool: Vec<Quadtree>,
}
impl QuadtreePool {
    pub fn new() -> Self {
        let mut ret = Self {
            dict: AHashMap::new(),
            pool: Vec::new(),
        };
        ret.pool.push(Quadtree::Cell(false));
        ret.pool.push(Quadtree::Cell(true));
        ret
    }
    pub fn insert_if_new(&mut self, t: Quadtree) -> usize {
        let key = t.as_subtree().get_key();
        if !self.dict.contains_key(&key) {
            self.pool.push(t);
            self.dict.insert(key, self.pool.len() - 1);
        }
        self.dict[&key]
    }
    pub fn set_ans(&mut self, id: usize, ans: usize) {
        self.pool[id].as_subtree_mut().ans = Some(ans);
    }
    pub fn alive_cell(&mut self) -> usize {
        1
    }
    pub fn dead_cell(&mut self) -> usize {
        0
    }
    pub fn join(&mut self, tl: usize, tr: usize, bl: usize, br: usize, height: u32) -> usize {
        self.insert_if_new(Quadtree::Subtree(Subtree {
            tl,
            tr,
            bl,
            br,
            height,
            ans: None,
            _private: (),
        }))
    }

    pub fn zeros(&mut self, height: u32) -> usize {
        self.load_alives(
            &mut Vec::new(),
            Point::new(0, 0),
            height,
            &mut AHashMap::new(),
        )
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
    pub fn gc_pool(&mut self, root: usize) -> (Self, usize) {
        fn copy_to_new_pool(
            cur: usize,
            pool: &mut QuadtreePool,
            new_pool: &mut QuadtreePool,
            dp: &mut AHashMap<usize, usize>,
        ) -> usize {
            match pool[cur] {
                Quadtree::Subtree(Subtree {
                    tl,
                    tr,
                    bl,
                    br,
                    height,
                    ans,
                    ..
                }) => {
                    if !dp.contains_key(&cur) {
                        let new_tl = copy_to_new_pool(tl, pool, new_pool, dp);
                        let new_tr = copy_to_new_pool(tr, pool, new_pool, dp);
                        let new_bl = copy_to_new_pool(bl, pool, new_pool, dp);
                        let new_br = copy_to_new_pool(br, pool, new_pool, dp);
                        let new = new_pool.join(new_tl, new_tr, new_bl, new_br, height);
                        if let Some(ans) = ans {
                            let new_ans = copy_to_new_pool(ans, pool, new_pool, dp);
                            new_pool.set_ans(new, new_ans);
                        }
                        dp.insert(cur, new);
                    }
                    dp[&cur]
                }
                Quadtree::Cell(alive) => {
                    if alive {
                        new_pool.alive_cell()
                    } else {
                        new_pool.dead_cell()
                    }
                }
            }
        }
        let mut new_pool = QuadtreePool::new();
        let mut dp = AHashMap::new();
        let new_root = copy_to_new_pool(root, self, &mut new_pool, &mut dp);
        (new_pool, new_root)
    }
}

impl Index<usize> for QuadtreePool {
    type Output = Quadtree;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pool[index]
    }
}
// disable accidental Copy
#[derive(Debug)]
pub enum Quadtree {
    Subtree(Subtree),
    Cell(bool),
}
#[derive(Debug)]
pub struct Subtree {
    pub tl: usize,
    pub tr: usize,
    pub bl: usize,
    pub br: usize,
    pub height: u32,
    pub ans: Option<usize>,
    _private: (),
}
impl Subtree {
    pub fn get_key(&self) -> QuadtreeKey {
        (self.tl, self.tr, self.bl, self.br)
    }
}
impl Quadtree {
    pub fn as_subtree(&self) -> &Subtree {
        match self {
            Quadtree::Subtree(subtree) => subtree,
            _ => panic!("called assert_subtree on {:?} which is not a subtree", self),
        }
    }
    pub fn as_subtree_mut(&mut self) -> &mut Subtree {
        match self {
            Quadtree::Subtree(subtree) => subtree,
            _ => panic!("called assert_subtree on {:?} which is not a subtree", self),
        }
    }
}
