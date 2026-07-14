use criterion::{Criterion, criterion_group, criterion_main};
use game_of_life::renderer::{CELL_SIZE_EXP, CellPoint, Renderer, ScreenPoint, ViewportInfo};
use num_bigint::BigInt;

fn filled_rect(c: &mut Criterion) {
    let mut renderer = Renderer::new(0);
    for i in 0..=16 {
        for j in 0..=16 {
            renderer.handle_draw(ScreenPoint::new(
                i * (1 << CELL_SIZE_EXP),
                j * (1 << CELL_SIZE_EXP),
            ));
        }
    }
    renderer.update_viewport(ViewportInfo {
        canvas_dims: ScreenPoint::new(1500, 1500),
    });
    renderer.handle_zoom(-3, ScreenPoint::new(0, 0));
    c.bench_function("filled 64x64 render", |b| b.iter(|| renderer.render()));
}

criterion_group!(benches, filled_rect);
criterion_main!(benches);
