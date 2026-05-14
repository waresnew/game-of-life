mod utils;
#[test]
fn one_step_cross() {
    test_solve!(
        vec![(0, 0), (0, 1), (1, 0), (-1, 0), (0, -1)],
        0,
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
fn one_step_empty() {
    test_solve!(vec![], 0, vec![]);
}

#[test]
fn one_step_glider() {
    test_solve!(
        vec![(-1, -1), (0, -1), (0, 1), (1, 0), (1, -1)],
        0,
        vec![(-1, 0), (0, -2), (0, -1), (1, -1), (1, 0)]
    );
}
#[test]
fn one_step_two() {
    test_solve!(vec![(0, 0), (1, 0)], 0, vec![]);
}
