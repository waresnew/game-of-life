use regex::regex;

use crate::{
    config::{CELL_SIZE_EXP, MIN_POINT},
    quadtree_pool::{ALIVE_CELL_ID, DEAD_CELL_ID, Quadtree, Subtree},
    renderer::{CellPoint, Renderer, ScreenPoint, WorldPoint},
};

impl Renderer {
    pub fn toggle_cell(&mut self, point: CellPoint) {
        self.solver.root = self.toggle_cell_and_return_root(point, self.solver.root, MIN_POINT);
        self.solver.update_stats();
    }
    #[must_use]
    fn toggle_cell_and_return_root(
        &mut self,
        point: CellPoint,
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
                if !Self::point_in_box(
                    point,
                    min,
                    CellPoint::new(min.x + (1 << height) - 1, min.y + (1 << height) - 1),
                ) {
                    return root;
                }
                let mid = 1 << (height - 1);
                let tl =
                    self.toggle_cell_and_return_root(point, tl, CellPoint::new(min.x, min.y + mid));
                let tr = self.toggle_cell_and_return_root(
                    point,
                    tr,
                    CellPoint::new(min.x + mid, min.y + mid),
                );
                let bl = self.toggle_cell_and_return_root(point, bl, min);
                let br =
                    self.toggle_cell_and_return_root(point, br, CellPoint::new(min.x + mid, min.y));
                self.solver.pool.join(tl, tr, bl, br, height)
            }
            Quadtree::Cell(alive) => {
                if min == point {
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
        let (width, height) = (width.parse().unwrap(), height.parse().unwrap());
        self.solver.set_rules(
            borns.split("").map(|x| x.parse().unwrap()).collect(),
            survives.split("").map(|x| x.parse().unwrap()).collect(),
        );
        let mut x = -self.viewport_info.canvas_dims.x / 2;
        let mut y = -self.viewport_info.canvas_dims.y / 2;
        self.fit_camera_to_dims(ScreenPoint::new(width, height));
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
                            self.toggle_cell(CellPoint::new(x, y));
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
    fn fit_camera_to_dims(&mut self, dims: ScreenPoint) {
        self.camera.centre = WorldPoint::new(0, 0);
        self.camera.zoom_out_exp = ((dims.x * (1 << CELL_SIZE_EXP)) / (dims.x))
            .max((dims.y * (1 << CELL_SIZE_EXP)) / (dims.y))
            .ilog2()
            + 1;
    }
}
