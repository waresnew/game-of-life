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
        // k.hash(&mut hasher);
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
    let grid = cur.to_array(dict);
    assert!(grid.len() == 16);
    fn apply_gol(cur_i: usize, cur_j: usize, grid: &[u8]) -> Quadtree {
        let cur = grid[4 * cur_i + cur_j];
        let mut alive_neighbours = 0;
        for di in -1_isize..=1 {
            for dj in -1_isize..=1 {
                if di == 0 && dj == 0 {
                    continue;
                }
                let ni = (cur_i as isize + di) as usize;
                let nj = (cur_j as isize + dj) as usize;
                if grid[4 * ni + nj] == 1 {
                    alive_neighbours += 1;
                }
            }
        }
        if cur == 1 {
            if !(2..=3).contains(&alive_neighbours) {
                Quadtree::dead_cell()
            } else {
                Quadtree::alive_cell()
            }
        } else if alive_neighbours == 3 {
            Quadtree::alive_cell()
        } else {
            Quadtree::dead_cell()
        }
    }
    let tl = apply_gol(1, 1, &grid);
    let tr = apply_gol(1, 2, &grid);
    let bl = apply_gol(2, 1, &grid);
    let br = apply_gol(2, 2, &grid);
    Quadtree {
        tl: tl.calc_hash(),
        tr: tr.calc_hash(),
        bl: bl.calc_hash(),
        br: br.calc_hash(),
        height: 1,
        count: tl.count + tr.count + bl.count + br.count,
    }
}
