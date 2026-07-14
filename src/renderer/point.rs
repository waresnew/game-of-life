use std::fmt;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

//TODO: reduce duplication
#[derive(Serialize, Deserialize, Default, Hash, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
#[wasm_bindgen]
pub struct ScreenPoint {
    // can be negative for out of bounds pixels
    //TODO: is 128 really necessary?
    pub x: i128,
    pub y: i128,
}
impl fmt::Debug for ScreenPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[wasm_bindgen]
impl ScreenPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i128, y: i128) -> Self {
        Self { x, y }
    }
}
impl ScreenPoint {
    pub fn from_tuple((x, y): (i128, i128)) -> Self {
        Self::new(x, y)
    }
    pub fn negate(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
#[derive(Serialize, Deserialize, Default, Hash, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
#[wasm_bindgen]
pub struct CellPoint {
    pub x: i128,
    pub y: i128,
}
impl fmt::Debug for CellPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl CellPoint {
    pub fn new(x: i128, y: i128) -> Self {
        Self { x, y }
    }
    pub fn from_tuple((x, y): (i128, i128)) -> Self {
        Self::new(x, y)
    }
    pub fn negate(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
#[derive(Serialize, Deserialize, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct WorldPoint {
    pub x: i128,
    pub y: i128,
}
impl fmt::Debug for WorldPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl WorldPoint {
    pub fn new(x: i128, y: i128) -> Self {
        Self { x, y }
    }
    pub fn from_tuple((x, y): (i128, i128)) -> Self {
        Self::new(x, y)
    }
    pub fn negate(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
