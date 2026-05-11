use game_of_life::{Point, solve};

fn main() {
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
    let res = solve(alive, 5206);
    assert_eq!(res.alive.len(), 633);
    dbg!(res.alive.len(), res.stats);
}
