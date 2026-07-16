use std::ops::Index;

use ahash::AHashMap;

mod convert;
mod manip;
pub mod quadtree;

pub use quadtree::*;

type QuadtreeKey = (usize, usize, usize, usize);

pub const DEAD_CELL_ID: usize = 0;
pub const ALIVE_CELL_ID: usize = 1;

#[derive(Debug)]
pub struct QuadtreePool {
    dict: AHashMap<QuadtreeKey, usize>,
    pool: Vec<Quadtree>,
    ans: Vec<Option<usize>>,
}

impl QuadtreePool {
    pub fn new() -> Self {
        Self {
            dict: AHashMap::new(),
            pool: vec![Quadtree::Cell(false), Quadtree::Cell(true)],
            ans: vec![None, None],
        }
    }

    fn insert_if_new(&mut self, t: Quadtree) -> usize {
        let subtree = t.as_subtree();
        let key = (subtree.tl, subtree.tr, subtree.bl, subtree.br);
        if !self.dict.contains_key(&key) {
            self.pool.push(t);
            self.ans.push(None);
            self.dict.insert(key, self.pool.len() - 1);
        }
        self.dict[&key]
    }
    pub fn clear_ans(&mut self) {
        for x in &mut self.ans {
            *x = None;
        }
    }
    pub fn get_ans(&self, id: usize) -> Option<usize> {
        self.ans[id]
    }
    pub fn set_ans(&mut self, id: usize, val: usize) {
        self.ans[id] = Some(val);
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
                    ..
                }) => {
                    if !dp.contains_key(&cur) {
                        let new_tl = copy_to_new_pool(tl, pool, new_pool, dp);
                        let new_tr = copy_to_new_pool(tr, pool, new_pool, dp);
                        let new_bl = copy_to_new_pool(bl, pool, new_pool, dp);
                        let new_br = copy_to_new_pool(br, pool, new_pool, dp);
                        let new = new_pool.join(new_tl, new_tr, new_bl, new_br, height);
                        if let Some(ans) = pool.ans[cur] {
                            let new_ans = copy_to_new_pool(ans, pool, new_pool, dp);
                            new_pool.ans[new] = Some(new_ans);
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

impl Default for QuadtreePool {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<usize> for QuadtreePool {
    type Output = Quadtree;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pool[index]
    }
}
