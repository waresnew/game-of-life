use criterion::{Criterion, criterion_group, criterion_main};
use game_of_life::renderer::{CellPoint, Renderer, ScreenPoint, ViewportInfo};
use num_bigint::BigInt;

fn random_rect(c: &mut Criterion) {
    let mut input = Vec::new();
    for i in -32..=32 {
        for j in -32..=32 {
            input.push(CellPoint::new(BigInt::from(i), BigInt::from(j)))
        }
    }
    let mut renderer = Renderer::new(12);
    for p in input.clone() {
        renderer.toggle_cell(&p);
        assert!(renderer.query_cell(&p));
    }
    renderer.update_viewport(ViewportInfo {
        canvas_dims: ScreenPoint::new(1500, 1500),
    });
    renderer.handle_zoom(-3, ScreenPoint::new(0, 0));
    c.bench_function("filled 64x64 render", |b| {
        b.iter(|| renderer.render_visible())
    });
}

criterion_group!(benches, random_rect);
criterion_main!(benches);
