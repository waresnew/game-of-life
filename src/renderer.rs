use std::fmt;

use num_bigint::BigUint;
use wasm_bindgen::prelude::*;

use crate::{
    quadtree::Quadtree,
    solver::{PerfStats, Solver},
};
pub const MIN_POINT: WorldPoint = WorldPoint {
    x: -1_000_000_000_000_000,
    y: -1_000_000_000_000_000,
};
#[wasm_bindgen]
pub struct Renderer {
    solver: Solver,
    pub base_cell_size: u32,
}
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct RendererOutput {
    pub min: WorldPoint,
    pub size_exp: u32,
}
impl RendererOutput {
    pub fn unit_cell(min: WorldPoint) -> Self {
        Self { min, size_exp: 0 }
    }
}
#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(getter=perf_stats)]
    pub fn perf_stats(&self) -> PerfStats {
        self.solver.perf_stats.clone()
    }
    #[wasm_bindgen(constructor)]
    pub fn new(step_exp: u32, base_cell_size: u32) -> Self {
        Self {
            solver: Solver::new(step_exp),
            base_cell_size,
        }
    }
    pub fn next_step(&mut self) {
        self.solver.next_step();
    }
    pub fn set_step_exp(&mut self, step_exp: u32) {
        self.solver.set_step_exp(step_exp);
    }
    pub fn render(
        &self,
        zoom: f64,
        bound_min: WorldPoint,
        bound_max: WorldPoint,
    ) -> Vec<RendererOutput> {
        self.to_visible_alives(
            self.solver.root,
            (bound_min, bound_max),
            self.base_cell_size,
            zoom,
            MIN_POINT,
        )
    }
    pub fn toggle_cell(&mut self, point: WorldPoint) {
        self.solver.root =
            self.solver
                .toggle_cell_and_return_root(point, self.solver.root, MIN_POINT);
        self.solver.update_stats();
    }
    pub fn clear_grid(&mut self) {
        self.solver = Solver::new(self.solver.step_exp());
    }
}
impl Renderer {
    /// tests/benches only
    pub fn render_all(&self) -> Vec<RendererOutput> {
        self.render(1.0, MIN_POINT, WorldPoint::negate(MIN_POINT))
    }
    pub fn to_visible_alives(
        &self,
        id: usize,
        bounds: (WorldPoint, WorldPoint),
        base_cell_size: u32,
        zoom: f64,
        min: WorldPoint,
    ) -> Vec<RendererOutput> {
        match self.solver.query_pool(id) {
            Quadtree::Subtree(root) => {
                if Self::boxes_disjoint(
                    bounds,
                    (
                        min,
                        WorldPoint::new(
                            min.x + (1_i64 << root.height),
                            min.y + (1_i64 << root.height),
                        ),
                    ),
                ) {
                    return vec![];
                }
                if root.count == BigUint::ZERO {
                    return vec![];
                }
                if (1_i64 << root.height) as f64 * base_cell_size as f64 * zoom <= 1.0 {
                    if root.count > BigUint::ZERO {
                        return vec![RendererOutput {
                            min,
                            size_exp: root.height,
                        }];
                    } else {
                        return vec![];
                    }
                }
                let mid = 1_i64 << (root.height - 1);
                let tl_ans = self
                    .to_visible_alives(
                        root.tl,
                        bounds,
                        base_cell_size,
                        zoom,
                        WorldPoint::new(min.x, min.y + mid),
                    )
                    .into_iter();
                let tr_ans = self
                    .to_visible_alives(
                        root.tr,
                        bounds,
                        base_cell_size,
                        zoom,
                        WorldPoint::new(min.x + mid, min.y + mid),
                    )
                    .into_iter();
                let bl_ans = self.to_visible_alives(root.bl, bounds, base_cell_size, zoom, min);
                let br_ans = self
                    .to_visible_alives(
                        root.br,
                        bounds,
                        base_cell_size,
                        zoom,
                        WorldPoint::new(min.x + mid, min.y),
                    )
                    .into_iter();
                tl_ans.chain(tr_ans).chain(bl_ans).chain(br_ans).collect()
            }
            &Quadtree::Cell(alive) => {
                if alive {
                    vec![RendererOutput::unit_cell(min)]
                } else {
                    vec![]
                }
            }
        }
    }

    fn boxes_disjoint(
        (first1, first2): (WorldPoint, WorldPoint),
        (second1, second2): (WorldPoint, WorldPoint),
    ) -> bool {
        let (min_x1, max_x1) = (first1.x.min(first2.x), first1.x.max(first2.x));
        let (min_y1, max_y1) = (first1.y.min(first2.y), first1.y.max(first2.y));
        let (min_x2, max_x2) = (second1.x.min(second2.x), second1.x.max(second2.x));
        let (min_y2, max_y2) = (second1.y.min(second2.y), second1.y.max(second2.y));
        min_x1 > max_x2 || min_y1 > max_y2 || max_x1 < min_x2 || max_y1 < min_y2
    }
}
#[wasm_bindgen]
#[derive(Default, Hash, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct WorldPoint {
    pub x: i64,
    pub y: i64,
}
impl fmt::Debug for WorldPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[wasm_bindgen]
impl WorldPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}
impl WorldPoint {
    pub fn from_tuple((x, y): (i64, i64)) -> Self {
        WorldPoint::new(x, y)
    }
    pub fn negate(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
