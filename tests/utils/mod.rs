use game_of_life::renderer::WorldPoint;

pub fn convert_coords(input: Vec<(i64, i64)>) -> Vec<WorldPoint> {
    input.into_iter().map(WorldPoint::from_tuple).collect()
}

#[macro_export]
macro_rules! test_solve {
    ($input:expr, $k:expr, $output:expr, $min_point:expr) => {
        use game_of_life::renderer::{Renderer, WorldPoint};
        let mut output = $crate::utils::convert_coords($output);
        let alive = $crate::utils::convert_coords($input);
        let mut renderer = Renderer::new($k);
        for p in alive {
            renderer.toggle_cell(p.x, p.y);
        }
        renderer.next_step();
        let mut res: Vec<WorldPoint> = renderer.render_all();
        res.sort();
        output.sort();
        assert_eq!(res, output);
    };
}
