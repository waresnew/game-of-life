use game_of_life::{Point, Solver};

fn main() {
    let mut solver = Solver::default();
    let alive: Vec<Point> = vec![
        (-2, 1),
        (-2, -1),
        (-3, -1),
        (0, 0),
        (1, -1),
        (2, -1),
        (3, -1),
    ]
    .into_iter()
    .map(Point::from_tuple)
    .collect();
    solver.init(alive, 12);
    let mut ans = vec![];
    for i in 0..100 {
        ans = solver.solve();
    } //633 alive
    dbg!(ans.len(), solver.perf_stats);
}
