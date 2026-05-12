use std::fmt;

use crate::{
    quadtree::Quadtree,
    solver::next_step,
    utils::{PerfStats, decompose_bits, update_dict},
};
use ahash::AHashMap;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::utils::web::*;

mod quadtree;
mod solver;
mod utils;

#[wasm_bindgen]
#[derive(Hash, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}
impl Point {
    pub fn from_tuple((x, y): (i64, i64)) -> Self {
        Point::new(x, y)
    }
}
fn calc_start_pos(alive: &Vec<Point>) -> Point {
    if alive.is_empty() {
        return Point::new(0, 0);
    }
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    for &Point { x, y } in alive {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
    }
    Point::new(min_x, min_y)
}
fn calc_height(alive: &Vec<Point>) -> u32 {
    if alive.is_empty() {
        return 1;
    }
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;
    for &Point { x, y } in alive {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    let dim = (max_x - min_x).max(max_y - min_y) + 1;
    if dim == 0 { 1 } else { dim.ilog2() + 1 }
}
#[wasm_bindgen(getter_with_clone)]
pub struct SolveOutput {
    pub alive: Vec<Point>,
    pub stats: PerfStats,
}
#[wasm_bindgen]
pub fn solve(mut alive: Vec<Point>, n: u64) -> SolveOutput {
    /* #[cfg(target_arch = "wasm32")]
    let _timer = Timer::start("solve"); */
    let mut dict = AHashMap::new();
    let Point {
        x: mut start_x,
        y: mut start_y,
    } = calc_start_pos(&alive);
    start_x -= n as i64;
    start_y -= n as i64;
    let height = calc_height(&alive) + (n.ilog2() + 1);
    let mut cur = Quadtree::from_alive(
        &mut alive,
        Point::new(start_x, start_y),
        height,
        &mut dict,
        &mut AHashMap::new(),
    );
    let mut next_step_dp = AHashMap::new();
    let mut stats = PerfStats::default();
    let bits = decompose_bits(n);
    for k in bits {
        cur = next_step(
            Quadtree::add_border(cur, &mut dict),
            k,
            &mut dict,
            &mut next_step_dp,
            &mut stats,
        );
    }
    let new_alive = cur
        .to_alive(&dict, &mut AHashMap::new())
        .into_iter()
        .map(|Point { x, y }| Point::new(x + start_x, y + start_y))
        .collect();
    SolveOutput {
        alive: new_alive,
        stats,
    }
}
#[wasm_bindgen(start, private)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
