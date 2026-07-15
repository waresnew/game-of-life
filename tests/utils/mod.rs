use game_of_life::renderer::CellPoint;
use malachite::Integer;

pub fn convert_coords(input: Vec<(i64, i64)>) -> Vec<CellPoint> {
    input
        .into_iter()
        .map(|(x, y)| (Integer::from(x), Integer::from(y)))
        .map(CellPoint::from_tuple)
        .collect()
}

#[macro_export]
macro_rules! test_solve {
    ($input:expr, $k:expr, $output:expr, $min_point:expr) => {
        use game_of_life::renderer::Renderer;
        let output = $crate::utils::convert_coords($output);
        let alive = $crate::utils::convert_coords($input);
        let mut renderer = Renderer::new($k);
        for p in alive {
            renderer.toggle_cell(&p);
        }
        renderer.next_step();
        assert_eq!(renderer.perf_stats().alives, output.len().to_string());
        for p in output {
            assert_eq!(renderer.query_cell(&p), true);
        }
    };
}
