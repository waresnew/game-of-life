use game_of_life::renderer::{CellPoint, Renderer};

fn main() {
    let alive: Vec<CellPoint> = vec![
        (-2, 1),
        (-2, -1),
        (-3, -1),
        (0, 0),
        (1, -1),
        (2, -1),
        (3, -1),
    ]
    .into_iter()
    .map(CellPoint::from_tuple)
    .collect();
    let mut renderer = Renderer::new(12);
    for p in alive {
        renderer.toggle_cell(p.x, p.y);
    }
    let mut ans = vec![];
    for _ in 0..100 {
        renderer.next_step();
        ans = renderer.render_all();
    } //633 alive
    dbg!(ans.len(), renderer.perf_stats());
}
