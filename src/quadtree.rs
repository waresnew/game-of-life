use std::ops::Index;

use ahash::AHashMap;
use num_bigint::BigUint;

mod convert;
mod manip;

type QuadtreeKey = (usize, usize, usize, usize);
pub const DEAD_CELL_ID: usize = 0;
pub const ALIVE_CELL_ID: usize = 1;
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
    pub fn clear_ans(&mut self) {
        for t in &mut self.pool {
            if let Quadtree::Subtree(subtree) = t {
                subtree.ans = None;
            }
        }
    }
    /// MB
    pub fn estimate_pool_mem(&self) -> usize {
        //exclude Vec overhead
        size_of::<Quadtree>() * self.pool.len() / 1_000_000
    }
    #[must_use]
    pub fn gc_pool_if_needed(&mut self, root: usize) -> Option<(Self, usize)> {
        const GC_THRESHOLD_MB: usize = 512;
        if self.estimate_pool_mem() > GC_THRESHOLD_MB {
            Some(self.gc_pool(root))
        } else {
            None
        }
    }
    #[must_use]
    fn gc_pool(&mut self, root: usize) -> (Self, usize) {
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
                        ALIVE_CELL_ID
                    } else {
                        DEAD_CELL_ID
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
    pub count: BigUint,
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
