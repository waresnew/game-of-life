use gloo_console::log;
use num_bigint::{BigInt, BigUint};
use regex::regex;

use crate::{
    quadtree_pool::{ALIVE_CELL_ID, DEAD_CELL_ID, Quadtree, Subtree},
    renderer::{CELL_SIZE_EXP, CellPoint, Renderer, WorldPoint},
};

impl Renderer {
    pub fn toggle_cell(&mut self, point: &CellPoint) {
        while BigUint::from(1_u32) << (self.solver.pool[self.solver.root].as_subtree().height - 1)
            <= *point.x.magnitude().max(point.y.magnitude())
        {
            self.solver.root = self.solver.pool.add_border(self.solver.root);
        }
        self.solver.root =
            self.toggle_cell_and_return_root(point, self.solver.root, self.solver.get_min_point());
        self.solver.update_stats();
    }
    #[must_use]
    fn toggle_cell_and_return_root(
        &mut self,
        point: &CellPoint,
        root: usize,
        min: CellPoint,
    ) -> usize {
        match self.solver.pool[root] {
            Quadtree::Subtree(Subtree {
                tl,
                tr,
                bl,
                br,
                height,
                ..
            }) => {
                let CellPoint { x: min_x, y: min_y } = min.clone();
                if !Self::point_in_box(
                    point,
                    &min,
                    &CellPoint::new(
                        &min_x + (BigInt::from(1) << height) - 1,
                        &min_y + (BigInt::from(1) << height) - 1,
                    ),
                ) {
                    return root;
                }
                let mid = BigInt::from(1) << (height - 1);
                let tl = self.toggle_cell_and_return_root(
                    point,
                    tl,
                    CellPoint::new(min_x.clone(), &min_y + &mid),
                );
                let tr = self.toggle_cell_and_return_root(
                    point,
                    tr,
                    CellPoint::new(&min_x + &mid, &min_y + &mid),
                );
                let bl = self.toggle_cell_and_return_root(point, bl, min.clone());
                let br = self.toggle_cell_and_return_root(
                    point,
                    br,
                    CellPoint::new(&min_x + &mid, min_y.clone()),
                );
                self.solver.pool.join(tl, tr, bl, br, height)
            }
            Quadtree::Cell(alive) => {
                if &min == point {
                    if alive { DEAD_CELL_ID } else { ALIVE_CELL_ID }
                } else {
                    root
                }
            }
        }
    }

    pub(super) fn load_rle_pattern(&mut self, pattern: String) {
        self.clear_grid();
        let mut content: Vec<&str> = pattern
            .trim()
            .split("\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty() && !x.starts_with("#"))
            .collect();
        let header_regex = regex!(r"^x = (\d+), y = (\d+), rule = [bB]?(\d+)\/[sS]?(\d+)");
        let (_, [width, height, borns, survives]) =
            header_regex.captures(content.remove(0)).unwrap().extract();
        let (width, height): (i128, i128) = (width.parse().unwrap(), height.parse().unwrap());
        self.solver.set_rule(
            borns
                .chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect(),
            survives
                .chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect(),
        );
        let mut x = -width / 2;
        let mut y = height / 2;
        self.fit_camera_to_dims(&CellPoint::new(BigInt::from(width), BigInt::from(height)));
        let lines: Vec<String> = content.join("").split("$").map(|x| x.to_string()).collect();
        for line in lines {
            let mut cnt_str = String::new();
            for c in line.chars() {
                if c == '!' {
                    return;
                }
                if c.is_ascii_digit() {
                    cnt_str.push(c);
                } else {
                    let cnt = if !cnt_str.is_empty() {
                        cnt_str.parse().unwrap()
                    } else {
                        1
                    };
                    if c != 'b' {
                        for _ in 0..cnt {
                            self.toggle_cell(&CellPoint::new(BigInt::from(x), BigInt::from(y)));
                            x += 1;
                        }
                    } else {
                        x += cnt;
                    }
                    cnt_str.clear();
                }
            }
            y -= if !cnt_str.is_empty() {
                cnt_str.parse().unwrap()
            } else {
                1
            };
            x = -width / 2;
        }
    }
    fn fit_camera_to_dims(&mut self, dims: &CellPoint) {
        self.camera.centre = WorldPoint::new(BigInt::from(0), BigInt::from(0));
        self.camera.zoom_out_exp = (((&dims.x * (BigInt::from(1) << CELL_SIZE_EXP))
            / self.viewport_info.canvas_dims.x)
            .max((&dims.y * (1 << CELL_SIZE_EXP)) / self.viewport_info.canvas_dims.y)
            .bits()
            .max(1)
            .min(u32::MAX as u64)
            + 1) as u32;
    }
}
