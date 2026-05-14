use ahash::AHashMap;

use crate::{Solver, quadtree::Quadtree};

pub fn next_step(cur_hash: u64, ctx: &mut Solver) -> u64 {
    let &Quadtree {
        tl,
        tr,
        bl,
        br,
        height: cur_height,
        ans: cur_ans,
        ..
    } = &ctx.dict[&cur_hash];
    if cur_ans.is_none() {
        ctx.perf_stats.cache_misses += 1;
        if cur_height == 2 {
            let ans = solve_4x4(cur_hash, ctx);
            ctx.dict.get_mut(&cur_hash).unwrap().ans = Some(ans);
            return ans;
        }
        let &Quadtree {
            tl: tl_tl,
            tr: tl_tr,
            bl: tl_bl,
            br: tl_br,
            ..
        } = &ctx.dict[&tl];
        let &Quadtree {
            tl: tr_tl,
            tr: tr_tr,
            bl: tr_bl,
            br: tr_br,
            ..
        } = &ctx.dict[&tr];
        let &Quadtree {
            tl: bl_tl,
            tr: bl_tr,
            bl: bl_bl,
            br: bl_br,
            ..
        } = &ctx.dict[&bl];
        let &Quadtree {
            tl: br_tl,
            tr: br_tr,
            bl: br_bl,
            br: br_br,
            ..
        } = &ctx.dict[&br];
        let next_tl = next_step(tl, ctx);
        let next_tm = next_step(
            Quadtree::join(tl_tr, tr_tl, tl_br, tr_bl, cur_height - 1, &mut ctx.dict),
            ctx,
        );
        let next_tr = next_step(tr, ctx);
        let next_ml = next_step(
            Quadtree::join(tl_bl, tl_br, bl_tl, bl_tr, cur_height - 1, &mut ctx.dict),
            ctx,
        );
        let next_mm = next_step(
            Quadtree::join(tl_br, tr_bl, bl_tr, br_tl, cur_height - 1, &mut ctx.dict),
            ctx,
        );
        let next_mr = next_step(
            Quadtree::join(tr_bl, tr_br, br_tl, br_tr, cur_height - 1, &mut ctx.dict),
            ctx,
        );
        let next_bl = next_step(bl, ctx);
        let next_bm = next_step(
            Quadtree::join(bl_tr, br_tl, bl_br, br_bl, cur_height - 1, &mut ctx.dict),
            ctx,
        );
        let next_br = next_step(br, ctx);
        let intermediate_tl = Quadtree::join(
            next_tl,
            next_tm,
            next_ml,
            next_mm,
            cur_height - 1,
            &mut ctx.dict,
        );
        let intermediate_tr = Quadtree::join(
            next_tm,
            next_tr,
            next_mm,
            next_mr,
            cur_height - 1,
            &mut ctx.dict,
        );
        let intermediate_bl = Quadtree::join(
            next_ml,
            next_mm,
            next_bl,
            next_bm,
            cur_height - 1,
            &mut ctx.dict,
        );
        let intermediate_br = Quadtree::join(
            next_mm,
            next_mr,
            next_bm,
            next_br,
            cur_height - 1,
            &mut ctx.dict,
        );
        let ans = if cur_height - 2 > ctx.step_exp {
            Quadtree::join(
                Quadtree::get_centre(intermediate_tl, &mut ctx.dict),
                Quadtree::get_centre(intermediate_tr, &mut ctx.dict),
                Quadtree::get_centre(intermediate_bl, &mut ctx.dict),
                Quadtree::get_centre(intermediate_br, &mut ctx.dict),
                cur_height - 1,
                &mut ctx.dict,
            )
        } else {
            Quadtree::join(
                next_step(intermediate_tl, ctx),
                next_step(intermediate_tr, ctx),
                next_step(intermediate_bl, ctx),
                next_step(intermediate_br, ctx),
                cur_height - 1,
                &mut ctx.dict,
            )
        };
        ctx.dict.get_mut(&cur_hash).unwrap().ans = Some(ans);
    } else {
        ctx.perf_stats.cache_hits += 1;
    }
    ctx.dict[&cur_hash].ans.unwrap()
}
fn solve_4x4<'a>(cur_hash: u64, ctx: &'a mut Solver) -> u64 {
    fn apply_gol(
        i: usize,
        j: usize,
        grid: &[[u64; 4]; 4],
        dict: &mut AHashMap<u64, Quadtree>,
    ) -> u64 {
        let alive = Quadtree::alive_cell(dict);
        let dead = Quadtree::dead_cell(dict);
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
    let cur = &ctx.dict[&cur_hash];
    let tl = &ctx.dict[&cur.tl];
    let tr = &ctx.dict[&cur.tr];
    let bl = &ctx.dict[&cur.bl];
    let br = &ctx.dict[&cur.br];
    let grid: [[u64; 4]; 4] = [
        [tl.tl, tl.tr, tr.tl, tr.tr],
        [tl.bl, tl.br, tr.bl, tr.br],
        [bl.tl, bl.tr, br.tl, br.tr],
        [bl.bl, bl.br, br.bl, br.br],
    ];
    let ans_tl = apply_gol(1, 1, &grid, &mut ctx.dict);
    let ans_tr = apply_gol(1, 2, &grid, &mut ctx.dict);
    let ans_bl = apply_gol(2, 1, &grid, &mut ctx.dict);
    let ans_br = apply_gol(2, 2, &grid, &mut ctx.dict);
    Quadtree::join(ans_tl, ans_tr, ans_bl, ans_br, 1, &mut ctx.dict)
}
