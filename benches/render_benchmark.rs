use criterion::{Criterion, criterion_group, criterion_main};
use game_of_life::{
    Camera, CellPoint, ScreenPoint, Viewport,
    app::CELL_SIZE_EXP,
    {GOL_RULES, Solver},
};
use malachite::Integer;

fn filled_rect(c: &mut Criterion) {
    let mut solver = Solver::new(0, GOL_RULES);
    const CANVAS_SIZE: i64 = 512;
    for i in -256..=256 {
        for j in -256..=256 {
            solver.toggle_cell(&CellPoint::new(Integer::from(i), Integer::from(j)));
        }
    }
    let canvas_dims = ScreenPoint::new(CANVAS_SIZE, CANVAS_SIZE);
    let viewport = Viewport {
        canvas_dims,
        camera: Camera {
            zoom_out_exp: CELL_SIZE_EXP,
            ..Default::default()
        },
    };
    c.bench_function("filled 512x512 render", |b| {
        b.iter(|| {
            game_of_life::render_to_image(
                &viewport,
                solver.root,
                &solver.pool,
                &solver.get_min_point(),
            )
        })
    });
}

criterion_group!(benches, filled_rect);
criterion_main!(benches);
