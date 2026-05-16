use game_of_life::Point;

pub fn convert_coords(input: Vec<(i64, i64)>) -> Vec<Point> {
    input.into_iter().map(Point::from_tuple).collect()
}

#[macro_export]
macro_rules! test_solve {
    ($input:expr, $k:expr, $output:expr) => {
        let mut output = $crate::utils::convert_coords($output);
        let alive = $crate::utils::convert_coords($input);
        let mut solver = game_of_life::Solver::default();
        solver.init(alive, $k);
        let mut res = solver.solve();
        res.sort();
        output.sort();
        assert_eq!(res, output);
    };
}
