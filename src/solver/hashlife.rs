use crate::{
    quadtree_pool::{ALIVE_CELL_ID, DEAD_CELL_ID, Subtree},
    solver::{LifeRule, Solver},
};

impl Solver {
    pub fn evolve(&mut self, cur_id: usize) -> usize {
        let &Subtree {
            tl,
            tr,
            bl,
            br,
            height: cur_height,
            ..
        } = self.pool[cur_id].as_subtree();
        let cur_ans = self.pool.get_ans(cur_id);
        if cur_ans.is_none() {
            self.perf_stats.cache_misses += 1;
            if cur_height == 2 {
                let ans = self.solve_4x4(cur_id);
                self.pool.set_ans(cur_id, ans);
                return ans;
            }
            let &Subtree {
                tl: _tl_tl,
                tr: tl_tr,
                bl: tl_bl,
                br: tl_br,
                ..
            } = self.pool[tl].as_subtree();
            let &Subtree {
                tl: tr_tl,
                tr: _tr_tr,
                bl: tr_bl,
                br: tr_br,
                ..
            } = self.pool[tr].as_subtree();
            let &Subtree {
                tl: bl_tl,
                tr: bl_tr,
                bl: _bl_bl,
                br: bl_br,
                ..
            } = self.pool[bl].as_subtree();
            let &Subtree {
                tl: br_tl,
                tr: br_tr,
                bl: br_bl,
                br: _br_br,
                ..
            } = self.pool[br].as_subtree();
            let next_tl = self.evolve(tl);
            let tm = self.pool.join(tl_tr, tr_tl, tl_br, tr_bl, cur_height - 1);
            let next_tm = self.evolve(tm);
            let next_tr = self.evolve(tr);
            let ml = self.pool.join(tl_bl, tl_br, bl_tl, bl_tr, cur_height - 1);
            let next_ml = self.evolve(ml);
            let mm = self.pool.join(tl_br, tr_bl, bl_tr, br_tl, cur_height - 1);
            let next_mm = self.evolve(mm);
            let mr = self.pool.join(tr_bl, tr_br, br_tl, br_tr, cur_height - 1);
            let next_mr = self.evolve(mr);
            let next_bl = self.evolve(bl);
            let bm = self.pool.join(bl_tr, br_tl, bl_br, br_bl, cur_height - 1);
            let next_bm = self.evolve(bm);
            let next_br = self.evolve(br);

            let intermediate_tl =
                self.pool
                    .join(next_tl, next_tm, next_ml, next_mm, cur_height - 1);
            let intermediate_tr =
                self.pool
                    .join(next_tm, next_tr, next_mm, next_mr, cur_height - 1);
            let intermediate_bl =
                self.pool
                    .join(next_ml, next_mm, next_bl, next_bm, cur_height - 1);
            let intermediate_br =
                self.pool
                    .join(next_mm, next_mr, next_bm, next_br, cur_height - 1);

            let ans = if cur_height - 2 > self.step_exp {
                let new_tl = self.pool.get_centre(intermediate_tl);
                let new_tr = self.pool.get_centre(intermediate_tr);
                let new_bl = self.pool.get_centre(intermediate_bl);
                let new_br = self.pool.get_centre(intermediate_br);
                self.pool
                    .join(new_tl, new_tr, new_bl, new_br, cur_height - 1)
            } else {
                let new_tl = self.evolve(intermediate_tl);
                let new_tr = self.evolve(intermediate_tr);
                let new_bl = self.evolve(intermediate_bl);
                let new_br = self.evolve(intermediate_br);
                self.pool
                    .join(new_tl, new_tr, new_bl, new_br, cur_height - 1)
            };
            self.pool.set_ans(cur_id, ans);
        } else {
            self.perf_stats.cache_hits += 1;
        }
        self.pool.get_ans(cur_id).unwrap()
    }
    fn solve_4x4(&mut self, cur_id: usize) -> usize {
        fn apply_gol(i: usize, j: usize, grid: &[[usize; 4]; 4], rules: LifeRule) -> usize {
            let mut alive_neighbours = 0;
            for di in -1..=1 {
                for dj in -1..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    if grid[(i as isize + di) as usize][(j as isize + dj) as usize] == ALIVE_CELL_ID
                    {
                        alive_neighbours += 1;
                    }
                }
            }
            if grid[i][j] == ALIVE_CELL_ID {
                if rules.survives(alive_neighbours) {
                    ALIVE_CELL_ID
                } else {
                    DEAD_CELL_ID
                }
            } else {
                if rules.is_born(alive_neighbours) {
                    ALIVE_CELL_ID
                } else {
                    DEAD_CELL_ID
                }
            }
        }
        let cur = self.pool[cur_id].as_subtree();
        let tl = self.pool[cur.tl].as_subtree();
        let tr = self.pool[cur.tr].as_subtree();
        let bl = self.pool[cur.bl].as_subtree();
        let br = self.pool[cur.br].as_subtree();
        let grid: [[usize; 4]; 4] = [
            [tl.tl, tl.tr, tr.tl, tr.tr],
            [tl.bl, tl.br, tr.bl, tr.br],
            [bl.tl, bl.tr, br.tl, br.tr],
            [bl.bl, bl.br, br.bl, br.br],
        ];
        let ans_tl = apply_gol(1, 1, &grid, self.rule());
        let ans_tr = apply_gol(1, 2, &grid, self.rule());
        let ans_bl = apply_gol(2, 1, &grid, self.rule());
        let ans_br = apply_gol(2, 2, &grid, self.rule());
        self.pool.join(ans_tl, ans_tr, ans_bl, ans_br, 1)
    }
}
