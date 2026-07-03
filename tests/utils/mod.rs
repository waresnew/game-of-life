use game_of_life::WorldPoint;

pub fn convert_coords(input: Vec<(i64, i64)>) -> Vec<WorldPoint> {
    input.into_iter().map(WorldPoint::from_tuple).collect()
}

#[macro_export]
macro_rules! test_solve {
    ($input:expr, $k:expr, $output:expr, $min_point:expr) => {
        let mut output = $crate::utils::convert_coords($output);
        let alive = $crate::utils::convert_coords($input);
        let mut renderer = game_of_life::Renderer::new($k, 50);
        for x in alive {
            renderer.toggle_cell(x);
        }
        renderer.next_step();
        let mut res = renderer.render(
            1.0,
            game_of_life::WorldPoint::from_tuple($min_point),
            game_of_life::WorldPoint::negate(game_of_life::WorldPoint::from_tuple($min_point)),
        );
        res.sort();
        output.sort();
        assert_eq!(res, output);
    };
}
