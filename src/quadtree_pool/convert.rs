use std::hash::{Hash, Hasher};

use ahash::{AHashMap, AHasher};

use crate::{
    quadtree_pool::{ALIVE_CELL_ID, DEAD_CELL_ID, Quadtree, QuadtreePool},
    renderer::WorldPoint,
};

#[allow(dead_code)]
impl QuadtreePool {
    #[deprecated = "use toggle_cell instead"]
    pub fn load_alives(
        &mut self,
        alive: &mut Vec<WorldPoint>,
        start_pos: WorldPoint,
        height: u32,
        dp: &mut AHashMap<u64, usize>,
    ) -> usize {
        if height == 0 {
            assert!(alive.len() <= 1, "alive len: {}", alive.len());
            if alive.len() == 1 {
                return ALIVE_CELL_ID;
            } else {
                return DEAD_CELL_ID;
            }
        }
        /// will mutate the arg to avoid a clone
        fn calc_key(alive: &mut Vec<WorldPoint>, height: u32) -> u64 {
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
            let mid_x = start_pos.x + (1_i64 << (height - 1));
            let mid_y = start_pos.y + (1_i64 << (height - 1));
            let mut tl_alive = Vec::new();
            let mut tr_alive = Vec::new();
            let mut bl_alive = Vec::new();
            let mut br_alive = Vec::new();
            for &mut WorldPoint { x, y } in alive {
                if x < mid_x && y >= mid_y {
                    tl_alive.push(WorldPoint::new(x, y));
                } else if x >= mid_x && y >= mid_y {
                    tr_alive.push(WorldPoint::new(x, y));
                } else if x < mid_x && y < mid_y {
                    bl_alive.push(WorldPoint::new(x, y));
                } else if x >= mid_x && y < mid_y {
                    br_alive.push(WorldPoint::new(x, y));
                } else {
                    unreachable!("cell not placed in quadrant");
                }
            }
            let tl = self.load_alives(
                &mut tl_alive,
                WorldPoint::new(start_pos.x, mid_y),
                height - 1,
                dp,
            );
            let tr = self.load_alives(&mut tr_alive, WorldPoint::new(mid_x, mid_y), height - 1, dp);
            let bl = self.load_alives(&mut bl_alive, start_pos, height - 1, dp);
            let br = self.load_alives(
                &mut br_alive,
                WorldPoint::new(mid_x, start_pos.y),
                height - 1,
                dp,
            );
            dp.insert(key, self.join(tl, tr, bl, br, height));
        }
        dp[&key]
    }
    #[deprecated = "only use this for debugging"]
    pub fn to_string(&self, id: usize) -> String {
        let grid = self.to_array(id);
        grid.iter()
            .map(|row| {
                row.iter()
                    .map(|x| if *x == 1 { "*" } else { "." })
                    .collect()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
    fn to_array(&self, id: usize) -> Vec<Vec<u8>> {
        match &self[id] {
            Quadtree::Subtree(root) => {
                let tl = self.to_array(root.tl);
                let tr = self.to_array(root.tr);
                let bl = self.to_array(root.bl);
                let br = self.to_array(root.br);
                fn block_concat(
                    left: Vec<Vec<u8>>,
                    right: Vec<Vec<u8>>,
                ) -> impl Iterator<Item = Vec<u8>> {
                    left.into_iter().zip(right).map(|(x, y)| [x, y].concat())
                }
                let top = block_concat(tl, tr);
                let bottom = block_concat(bl, br);
                top.chain(bottom).collect()
            }
            &Quadtree::Cell(alive) => vec![vec![if alive { 1 } else { 0 }]],
        }
    }
}
