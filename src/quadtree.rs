use std::ops::Index;

use ahash::AHashMap;

use crate::Point;

mod convert;

type QuadtreeKey = (usize, usize, usize, usize, u32); //HACK:a height 1 quadtree containing 4 dead cells is mixed upw ith the dead cell itself
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
        ret.dead_cell();
        ret.alive_cell();
        ret
    }
    pub fn insert_if_new(&mut self, t: Quadtree) -> usize {
        let key = t.get_key();
        if !self.dict.contains_key(&key) {
            self.pool.push(t);
            self.dict.insert(key, self.pool.len() - 1);
        }
        self.dict[&key]
    }
    pub fn set_ans(&mut self, id: usize, ans: usize) {
        self.pool[id].ans = Some(ans);
    }
    pub fn alive_cell(&mut self) -> usize {
        let ret = self.join(1, 1, 1, 1, 0);
        assert_eq!(ret, 1);
        ret
    }
    pub fn dead_cell(&mut self) -> usize {
        let ret = self.join(0, 0, 0, 0, 0);
        assert_eq!(ret, 0);
        ret
    }
    pub fn join(&mut self, tl: usize, tr: usize, bl: usize, br: usize, height: u32) -> usize {
        self.insert_if_new(Quadtree {
            tl,
            tr,
            bl,
            br,
            height,
            ans: None,
            _private: (),
        })
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
        let Quadtree {
            tl,
            tr,
            bl,
            br,
            height,
            ..
        } = self[t];
        let zero = self.zeros(height - 1);
        let tl = self.join(zero, zero, zero, tl, height);
        let tr = self.join(zero, zero, tr, zero, height);
        let bl = self.join(zero, bl, zero, zero, height);
        let br = self.join(br, zero, zero, zero, height);
        self.join(tl, tr, bl, br, height + 1)
    }
    pub fn get_centre(&mut self, t: usize) -> usize {
        let t = &self[t];
        self.join(
            self[t.tl].br,
            self[t.tr].bl,
            self[t.bl].tr,
            self[t.br].tl,
            t.height - 1,
        )
    }
    pub fn gc_pool(&mut self, root: usize) -> (Self, usize) {
        fn copy_to_new_pool(
            cur: usize,
            pool: &mut QuadtreePool,
            new_pool: &mut QuadtreePool,
            dp: &mut AHashMap<usize, usize>,
        ) -> usize {
            let Quadtree {
                tl,
                tr,
                bl,
                br,
                height,
                ans,
                ..
            } = pool[cur];
            if !dp.contains_key(&cur) {
                if height == 0 {
                    let alive = pool.alive_cell();
                    if cur == alive {
                        return new_pool.alive_cell();
                    } else {
                        return new_pool.dead_cell();
                    }
                }
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
#[derive(Debug, Default)]
pub struct Quadtree {
    pub tl: usize,
    pub tr: usize,
    pub bl: usize,
    pub br: usize,
    pub height: u32,
    pub ans: Option<usize>,
    _private: (),
}
impl Quadtree {
    pub fn get_key(&self) -> QuadtreeKey {
        (self.tl, self.tr, self.bl, self.br, self.height)
    }
}
