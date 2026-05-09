#[test]
fn one_step_cross() {
    let alive = vec![(0, 0), (0, 1), (1, 0), (-1, 0), (0, -1)];
    let mut res = game_of_life::solve(alive, 2);
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
#[test]
fn empty() {
    let alive = vec![];
    let res = game_of_life::solve(alive, 1);
    assert_eq!(res.len(), 0);
}

#[test]
fn one_step_glider() {
    let alive = vec![(-1, -1), (0, -1), (0, 1), (1, 0), (1, -1)];
    let mut res = game_of_life::solve(alive, 2);
    res.sort();
    assert_eq!(res, vec![(-1, 0), (0, -2), (0, -1), (1, -1), (1, 0)]);
}
