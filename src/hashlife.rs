use crate::{
    Solver,
    quadtree::{QuadtreePool, Subtree},
};

pub fn evolve(cur_id: usize, ctx: &mut Solver) -> usize {
    let &Subtree {
        tl,
        tr,
        bl,
        br,
        height: cur_height,
        ans: cur_ans,
        ..
    } = ctx.pool[cur_id].as_subtree();
    if cur_ans.is_none() {
        ctx.perf_stats.cache_misses += 1;
        if cur_height == 2 {
            let ans = solve_4x4(cur_id, ctx);
            ctx.pool.set_ans(cur_id, ans);
            return ans;
        }
        let &Subtree {
            tl: tl_tl,
            tr: tl_tr,
            bl: tl_bl,
            br: tl_br,
            ..
        } = ctx.pool[tl].as_subtree();
        let &Subtree {
            tl: tr_tl,
            tr: tr_tr,
            bl: tr_bl,
            br: tr_br,
            ..
        } = ctx.pool[tr].as_subtree();
        let &Subtree {
            tl: bl_tl,
            tr: bl_tr,
            bl: bl_bl,
            br: bl_br,
            ..
        } = ctx.pool[bl].as_subtree();
        let &Subtree {
            tl: br_tl,
            tr: br_tr,
            bl: br_bl,
            br: br_br,
            ..
        } = ctx.pool[br].as_subtree();
        let next_tl = evolve(tl, ctx);
        let next_tm = evolve(
            ctx.pool.join(tl_tr, tr_tl, tl_br, tr_bl, cur_height - 1),
            ctx,
        );
        let next_tr = evolve(tr, ctx);
        let next_ml = evolve(
            ctx.pool.join(tl_bl, tl_br, bl_tl, bl_tr, cur_height - 1),
            ctx,
        );
        let next_mm = evolve(
            ctx.pool.join(tl_br, tr_bl, bl_tr, br_tl, cur_height - 1),
            ctx,
        );
        let next_mr = evolve(
            ctx.pool.join(tr_bl, tr_br, br_tl, br_tr, cur_height - 1),
            ctx,
        );
        let next_bl = evolve(bl, ctx);
        let next_bm = evolve(
            ctx.pool.join(bl_tr, br_tl, bl_br, br_bl, cur_height - 1),
            ctx,
        );
        let next_br = evolve(br, ctx);

        let intermediate_tl = ctx
            .pool
            .join(next_tl, next_tm, next_ml, next_mm, cur_height - 1);
        let intermediate_tr = ctx
            .pool
            .join(next_tm, next_tr, next_mm, next_mr, cur_height - 1);
        let intermediate_bl = ctx
            .pool
            .join(next_ml, next_mm, next_bl, next_bm, cur_height - 1);
        let intermediate_br = ctx
            .pool
            .join(next_mm, next_mr, next_bm, next_br, cur_height - 1);

        let ans = if cur_height - 2 > ctx.step_exp {
            let new_tl = ctx.pool.get_centre(intermediate_tl);
            let new_tr = ctx.pool.get_centre(intermediate_tr);
            let new_bl = ctx.pool.get_centre(intermediate_bl);
            let new_br = ctx.pool.get_centre(intermediate_br);
            ctx.pool
                .join(new_tl, new_tr, new_bl, new_br, cur_height - 1)
        } else {
            let new_tl = evolve(intermediate_tl, ctx);
            let new_tr = evolve(intermediate_tr, ctx);
            let new_bl = evolve(intermediate_bl, ctx);
            let new_br = evolve(intermediate_br, ctx);
            ctx.pool
                .join(new_tl, new_tr, new_bl, new_br, cur_height - 1)
        };
        ctx.pool.set_ans(cur_id, ans);
    } else {
        ctx.perf_stats.cache_hits += 1;
    }
    ctx.pool[cur_id].as_subtree().ans.unwrap()
}
fn solve_4x4(cur_id: usize, ctx: &mut Solver) -> usize {
    fn apply_gol(i: usize, j: usize, grid: &[[usize; 4]; 4], pool: &mut QuadtreePool) -> usize {
        let alive = pool.alive_cell();
        let dead = pool.dead_cell();
        let mut alive_neighbours = 0;
        for di in -1..=1 {
            for dj in -1..=1 {
                if di == 0 && dj == 0 {
                    continue;
                }
                if grid[(i as isize + di) as usize][(j as isize + dj) as usize] == alive {
                    alive_neighbours += 1;
                }
            }
        }
        if grid[i][j] == alive {
            if !(2..=3).contains(&alive_neighbours) {
                dead
            } else {
                alive
            }
        } else if alive_neighbours == 3 {
            alive
        } else {
            dead
        }
    }
    let cur = ctx.pool[cur_id].as_subtree();
    let tl = ctx.pool[cur.tl].as_subtree();
    let tr = ctx.pool[cur.tr].as_subtree();
    let bl = ctx.pool[cur.bl].as_subtree();
    let br = ctx.pool[cur.br].as_subtree();
    let grid: [[usize; 4]; 4] = [
        [tl.tl, tl.tr, tr.tl, tr.tr],
        [tl.bl, tl.br, tr.bl, tr.br],
        [bl.tl, bl.tr, br.tl, br.tr],
        [bl.bl, bl.br, br.bl, br.br],
    ];
    let ans_tl = apply_gol(1, 1, &grid, &mut ctx.pool);
    let ans_tr = apply_gol(1, 2, &grid, &mut ctx.pool);
    let ans_bl = apply_gol(2, 1, &grid, &mut ctx.pool);
    let ans_br = apply_gol(2, 2, &grid, &mut ctx.pool);
    ctx.pool.join(ans_tl, ans_tr, ans_bl, ans_br, 1)
}
