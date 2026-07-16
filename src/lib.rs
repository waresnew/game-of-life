pub mod app;

mod input_handler;
mod point;
mod quadtree_pool;
mod renderer;
mod solver;

pub use crate::input_handler::{Camera, Viewport};
pub use crate::point::{CellPoint, ScreenPoint};
pub use crate::renderer::render_to_image;
pub use crate::solver::{GOL_RULES, Solver};
