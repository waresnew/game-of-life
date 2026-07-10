use wasm_bindgen::prelude::*;

pub mod config;
pub mod quadtree_pool;
pub mod renderer;
pub mod solver;

#[wasm_bindgen(start, private)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
