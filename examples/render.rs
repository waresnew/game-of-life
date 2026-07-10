use game_of_life::renderer::{CellPoint, Renderer};

fn main() {
    let mut input = Vec::new();
    for i in -32..=32 {
        for j in -32..=32 {
            input.push(CellPoint::new(i, j))
        }
    }
    let mut renderer = Renderer::new(12);
    for p in input.clone() {
        renderer.toggle_cell(p.x, p.y);
    }

    for _i in 0..1000 {
        renderer.render_all();
    }
}
