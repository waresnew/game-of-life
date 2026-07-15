use criterion::{Criterion, criterion_group, criterion_main};
use game_of_life::renderer::{CELL_SIZE_EXP, Renderer, ScreenPoint, ViewportInfo};

fn filled_rect(c: &mut Criterion) {
    let mut renderer = Renderer::new(0);
    renderer.handle_zoom(CELL_SIZE_EXP as i32, ScreenPoint::new(0, 0));
    const CANVAS_SIZE: i64 = 100;
    for i in 0..CANVAS_SIZE {
        for j in 0..CANVAS_SIZE {
            renderer.handle_draw(ScreenPoint::new(i, j));
        }
    }
    renderer.update_viewport(ViewportInfo {
        canvas_dims: ScreenPoint::new(CANVAS_SIZE, CANVAS_SIZE),
    });
    c.bench_function("filled 1:1 pixel:cell ratio render", |b| {
        b.iter(|| renderer.render())
    });
}

criterion_group!(benches, filled_rect);
criterion_main!(benches);
