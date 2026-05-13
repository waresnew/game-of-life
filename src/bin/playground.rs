use game_of_life::{Point, Solver};

fn main() {
    let mut solver = Solver::default();
    let mut alive: Vec<Point> = vec![
        (-2, 1),
        (-2, -1),
        (-3, -1),
        (0, 0),
        (1, -1),
        (2, -1),
        (3, -1),
    ]
    .into_iter()
    .map(|x| Point::from_tuple(x))
    .collect();
    solver.load_alive(&mut alive);
    for i in 0..100 {
        alive = solver.solve(4000);
    } //633 alive
    dbg!(alive.len(), solver.perf_stats);
}
