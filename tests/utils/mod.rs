use game_of_life::CellPoint;

pub fn convert_coords(input: Vec<(i128, i128)>) -> Vec<CellPoint> {
    input.into_iter().map(CellPoint::from_tuple).collect()
}

#[macro_export]
macro_rules! test_solve {
    ($input:expr, $k:expr, $output:expr, $min_point:expr) => {
        use game_of_life::{GOL_RULES, Solver};
        let output = $crate::utils::convert_coords($output);
        let alive = $crate::utils::convert_coords($input);
        let mut solver = Solver::new($k, GOL_RULES);
        for p in alive {
            solver.toggle_cell(p);
        }
        solver.next_step();
        assert_eq!(solver.stats().alives, output.len());
        for p in output {
            assert_eq!(solver.query_cell(p), true);
        }
    };
}
