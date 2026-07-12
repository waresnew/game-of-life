use game_of_life::renderer::{CellPoint, Renderer};

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
        ],
        (-20, -20)
    );
}
#[test]
fn one_step_empty() {
    test_solve!(vec![], 0, vec![], (-20, -20));
}

#[test]
fn one_step_glider() {
    test_solve!(
        vec![(-1, -1), (0, -1), (0, 1), (1, 0), (1, -1)],
        0,
        vec![(-1, 0), (0, -2), (0, -1), (1, -1), (1, 0)],
        (-20, -20)
    );
}
#[test]
fn one_step_two() {
    test_solve!(vec![(0, 0), (1, 0)], 0, vec![], (-20, -20));
}
#[test]
fn one_step_acorn() {
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
    let mut renderer = Renderer::new(13);
    for p in alive {
        renderer.toggle_cell(p.x, p.y);
    }
    renderer.next_step();
    assert_eq!(renderer.perf_stats().alives, "633");
}
