use std::hash::{Hash, Hasher};

use ahash::{AHashMap, AHasher};

use crate::{quadtree::Quadtree, utils::PerfStats};

pub fn next_step(
    cur: Quadtree,
    k: u32,
    dict: &mut AHashMap<u64, Quadtree>,
    dp: &mut AHashMap<u64, Quadtree>,
    stats: &mut PerfStats,
) -> Quadtree {
    fn calc_key(cur: Quadtree, k: u32) -> u64 {
        let mut hasher = AHasher::default();
        cur.hash(&mut hasher);
        k.hash(&mut hasher);
        hasher.finish()
    }
    let key = calc_key(cur, k);
    if !dp.contains_key(&key) {
        stats.cache_misses += 1;
        if cur.height == 2 {
            let ans = solve_4x4(cur, dict);
            dp.insert(key, ans);
            return ans;
        }
        let next_tl = next_step(dict[&cur.tl], k, dict, dp, stats);
        let next_tm = next_step(
            Quadtree::join_with_u64(
                dict[&cur.tl].tr,
                dict[&cur.tr].tl,
                dict[&cur.tl].br,
                dict[&cur.tr].bl,
                dict,
            ),
            k,
            dict,
            dp,
            stats,
        );
        let next_tr = next_step(dict[&cur.tr], k, dict, dp, stats);
        let next_ml = next_step(
            Quadtree::join_with_u64(
                dict[&cur.tl].bl,
                dict[&cur.tl].br,
                dict[&cur.bl].tl,
                dict[&cur.bl].tr,
                dict,
            ),
            k,
            dict,
            dp,
            stats,
        );
        let next_mm = next_step(
            Quadtree::join_with_u64(
                dict[&cur.tl].br,
                dict[&cur.tr].bl,
                dict[&cur.bl].tr,
                dict[&cur.br].tl,
                dict,
            ),
            k,
            dict,
            dp,
            stats,
        );
        let next_mr = next_step(
            Quadtree::join_with_u64(
                dict[&cur.tr].bl,
                dict[&cur.tr].br,
                dict[&cur.br].tl,
                dict[&cur.br].tr,
                dict,
            ),
            k,
            dict,
            dp,
            stats,
        );
        let next_bl = next_step(dict[&cur.bl], k, dict, dp, stats);
        let next_bm = next_step(
            Quadtree::join_with_u64(
                dict[&cur.bl].tr,
                dict[&cur.br].tl,
                dict[&cur.bl].br,
                dict[&cur.br].bl,
                dict,
            ),
            k,
            dict,
            dp,
            stats,
        );
        let next_br = next_step(dict[&cur.br], k, dict, dp, stats);
        let intermediate_tl = Quadtree::join(next_tl, next_tm, next_ml, next_mm, dict);
        let intermediate_tr = Quadtree::join(next_tm, next_tr, next_mm, next_mr, dict);
        let intermediate_bl = Quadtree::join(next_ml, next_mm, next_bl, next_bm, dict);
        let intermediate_br = Quadtree::join(next_mm, next_mr, next_bm, next_br, dict);
        let ans = if cur.height - 2 > k {
            Quadtree::join(
                Quadtree::get_centre(intermediate_tl, dict),
                Quadtree::get_centre(intermediate_tr, dict),
                Quadtree::get_centre(intermediate_bl, dict),
                Quadtree::get_centre(intermediate_br, dict),
                dict,
            )
        } else {
            Quadtree::join(
                next_step(intermediate_tl, k, dict, dp, stats),
                next_step(intermediate_tr, k, dict, dp, stats),
                next_step(intermediate_bl, k, dict, dp, stats),
                next_step(intermediate_br, k, dict, dp, stats),
                dict,
            )
        };
        dp.insert(key, ans);
    } else {
        stats.cache_hits += 1;
    }
    *dp.get(&key).unwrap()
}
fn solve_4x4(cur: Quadtree, dict: &AHashMap<u64, Quadtree>) -> Quadtree {
    fn apply_gol(i: usize, j: usize, grid: &[[u64; 4]; 4]) -> Quadtree {
        let alive = Quadtree::alive_cell();
        let dead = Quadtree::dead_cell();
        let alive_hash = alive.calc_hash();
        let mut alive_neighbours = 0;
        for di in -1..=1 {
            for dj in -1..=1 {
                if di == 0 && dj == 0 {
                    continue;
                }
                if grid[(i as isize + di) as usize][(j as isize + dj) as usize] == alive_hash {
                    alive_neighbours += 1;
                }
            }
        }
        if grid[i][j] == alive_hash {
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
    let tl = dict[&cur.tl];
    let tr = dict[&cur.tr];
    let bl = dict[&cur.bl];
    let br = dict[&cur.br];
    let grid: [[u64; 4]; 4] = [
        [tl.tl, tl.tr, tr.tl, tr.tr],
        [tl.bl, tl.br, tr.bl, tr.br],
        [bl.tl, bl.tr, br.tl, br.tr],
        [bl.bl, bl.br, br.bl, br.br],
    ];
    let ans_tl = apply_gol(1, 1, &grid);
    let ans_tr = apply_gol(1, 2, &grid);
    let ans_bl = apply_gol(2, 1, &grid);
    let ans_br = apply_gol(2, 2, &grid);
    Quadtree {
        tl: ans_tl.calc_hash(),
        tr: ans_tr.calc_hash(),
        bl: ans_bl.calc_hash(),
        br: ans_br.calc_hash(),
        height: 1,
        count: ans_tl.count + ans_tr.count + ans_bl.count + ans_br.count,
    }
}
