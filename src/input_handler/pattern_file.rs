use crate::{
    app::CELL_SIZE_EXP,
    input_handler::InputHandler,
    point::{CellPoint, WorldPoint},
    solver::Solver,
};
use malachite::base::num::logic::traits::SignificantBits;
use regex::regex;

impl InputHandler {
    pub fn load_rle_pattern(&mut self, pattern: String, solver: &mut Solver) {
        solver.clear_grid();
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
        solver.set_rule(
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
        self.fit_camera_to_dims(CellPoint::new(width, height));
        let lines: Vec<String> = content.join("").split("$").map(|x| x.to_owned()).collect();
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
                            solver.toggle_cell(CellPoint::new(x, y));
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
    fn fit_camera_to_dims(&mut self, dims: CellPoint) {
        self.viewport.camera.centre = WorldPoint::new(0, 0);
        let cell_size = 1 << CELL_SIZE_EXP;
        self.viewport.camera.zoom_out_exp = (((dims.x * cell_size) / self.viewport.canvas_dims.x)
            .max((dims.y * cell_size) / self.viewport.canvas_dims.y)
            .significant_bits()
            .max(1)
            .min(u32::MAX as u64)
            + 1) as u32;
    }
}
