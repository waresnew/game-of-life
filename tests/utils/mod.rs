use game_of_life::Point;

pub fn convert_coords(input: Vec<(i64, i64)>) -> Vec<Point> {
    input.into_iter().map(|x| Point::from_tuple(x)).collect()
}

#[macro_export]
macro_rules! test_solve {
    ($input:expr, $n:expr, $output:expr) => {
        let mut output = crate::utils::convert_coords($output);
        let alive = crate::utils::convert_coords($input);
        let mut res = game_of_life::solve(alive, $n);
        res.alive.sort();
        output.sort();
        assert_eq!(res.alive, output);
    };
}
