use game_of_life::{Renderer, Solver, WorldPoint};

fn main() {
    let alive: Vec<WorldPoint> = vec![
        (-2, 1),
        (-2, -1),
        (-3, -1),
        (0, 0),
        (1, -1),
        (2, -1),
        (3, -1),
    ]
    .into_iter()
    .map(WorldPoint::from_tuple)
    .collect();
    let mut renderer = Renderer::new(12, 50);
    for x in alive {
        renderer.toggle_cell(x);
    }
    let mut ans = vec![];
    for i in 0..100 {
        renderer.next_step();
        ans = renderer.render(1.0, WorldPoint::new(-200, -200), WorldPoint::new(200, 200));
    } //633 alive
    dbg!(ans.len(), renderer.perf_stats());
}
