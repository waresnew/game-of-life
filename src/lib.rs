use wasm_bindgen::prelude::*;

pub mod quadtree;
pub mod renderer;
pub mod solver;

#[wasm_bindgen(start, private)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
