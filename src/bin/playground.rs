use game_of_life::{Point, Solver};

fn main() {
    let mut solver = Solver::default();
    let alive = vec![
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
    let res = solver.solve(alive, 6000); //633 alive
    dbg!(res.len(), solver.perf_stats);
}
