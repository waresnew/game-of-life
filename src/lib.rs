use std::collections::HashMap;

use crate::quadtree::Quadtree;

mod quadtree;

fn join(
    tl: Quadtree,
    tr: Quadtree,
    bl: Quadtree,
    br: Quadtree,
    dict: &mut HashMap<u64, Quadtree>,
) -> Quadtree {
    assert!(tl.height == tr.height && tr.height == bl.height && bl.height == br.height);
    update_dict(tl, dict);
    update_dict(tr, dict);
    update_dict(bl, dict);
    update_dict(br, dict);
    Quadtree {
        tl: tl.calc_hash(),
        tr: tr.calc_hash(),
        bl: bl.calc_hash(),
        br: br.calc_hash(),
        height: tl.height + 1,
        count: tl.count + tr.count + bl.count + br.count,
    }
}
fn join_with_u64(
    tl: u64,
    tr: u64,
    bl: u64,
    br: u64,
    dict: &mut HashMap<u64, Quadtree>,
) -> Quadtree {
    join(dict[&tl], dict[&tr], dict[&bl], dict[&br], dict)
}
fn next_step(cur: Quadtree, dict: &mut HashMap<u64, Quadtree>) -> Quadtree {
    if cur.height == 2 {
        return solve_4x4(cur, dict);
    }
    let next_tl = next_step(dict[&cur.tl], dict);
    let next_tm = next_step(
        join_with_u64(
            dict[&cur.tl].tr,
            dict[&cur.tr].tl,
            dict[&cur.tl].br,
            dict[&cur.tr].bl,
            dict,
        ),
        dict,
    );
    let next_tr = next_step(dict[&cur.tr], dict);
    let next_ml = next_step(
        join_with_u64(
            dict[&cur.tl].bl,
            dict[&cur.tl].br,
            dict[&cur.bl].tl,
            dict[&cur.bl].tr,
            dict,
        ),
        dict,
    );
    let next_mm = next_step(
        join_with_u64(
            dict[&cur.tl].br,
            dict[&cur.tr].bl,
            dict[&cur.bl].tr,
            dict[&cur.br].tl,
            dict,
        ),
        dict,
    );
    let next_mr = next_step(
        join_with_u64(
            dict[&cur.tr].bl,
            dict[&cur.tr].br,
            dict[&cur.br].tl,
            dict[&cur.br].tr,
            dict,
        ),
        dict,
    );
    let next_bl = next_step(dict[&cur.bl], dict);
    let next_bm = next_step(
        join_with_u64(
            dict[&cur.bl].tr,
            dict[&cur.br].tl,
            dict[&cur.bl].br,
            dict[&cur.br].bl,
            dict,
        ),
        dict,
    );
    let next_br = next_step(dict[&cur.br], dict);
    let ans_tl = join_with_u64(next_tl.br, next_tm.bl, next_ml.tr, next_mm.tl, dict);
    let ans_tr = join_with_u64(next_tm.br, next_tr.bl, next_mm.tr, next_mr.tl, dict);
    let ans_bl = join_with_u64(next_ml.br, next_mm.bl, next_bl.tr, next_bm.tl, dict);
    let ans_br = join_with_u64(next_mm.br, next_mr.bl, next_bm.tr, next_br.tl, dict);
    join(ans_tl, ans_tr, ans_bl, ans_br, dict)
}
fn update_dict(t: Quadtree, dict: &mut HashMap<u64, Quadtree>) -> u64 {
    let hash = t.calc_hash();
    dict.insert(hash, t);
    hash
}
fn add_border(t: Quadtree, dict: &mut HashMap<u64, Quadtree>) -> Quadtree {
    let zero = Quadtree::zeros(t.height - 1, dict);
    Quadtree {
        tl: update_dict(join(zero, zero, zero, dict[&t.tl], dict), dict),
        tr: update_dict(join(zero, zero, dict[&t.tr], zero, dict), dict),
        bl: update_dict(join(zero, dict[&t.bl], zero, zero, dict), dict),
        br: update_dict(join(dict[&t.br], zero, zero, zero, dict), dict),
        height: t.height + 1,
        count: t.count,
    }
}
fn solve_4x4(cur: Quadtree, dict: &HashMap<u64, Quadtree>) -> Quadtree {
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
fn calc_start_pos(alive: &Vec<(i64, i64)>) -> (i64, i64) {
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    for &(x, y) in alive {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
    }
    (min_x - 1, min_y - 1) //bc of edge cells
}
pub fn solve(alive: Vec<(i64, i64)>, height: u32) -> Vec<(i64, i64)> {
    if height < 1 {
        panic!("height must be >=1");
    }
    let mut dict = HashMap::new();
    let start_pos = calc_start_pos(&alive);
    let qt = Quadtree::from_alive(&alive, start_pos, height, &mut dict);
    dbg!(&qt, &dict);
    let res = next_step(add_border(qt, &mut dict), &mut dict);
    res.to_alive(start_pos, &dict)
}
