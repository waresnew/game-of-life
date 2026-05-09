#[test]
fn one_step() {
    let alive = vec![(0, 0), (0, 1), (1, 0), (-1, 0), (0, -1)];
    let mut res = game_of_life::solve(alive, (-1, -1), 2);
    res.sort();
    assert_eq!(
        res,
        vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1)
        ]
    );
}
