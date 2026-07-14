use std::fmt;

use num_bigint::BigInt;
use wasm_bindgen::prelude::*;

//TODO: reduce duplication
#[derive(Default, Hash, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
#[wasm_bindgen]
pub struct ScreenPoint {
    // can be negative for out of bounds pixels
    //TODO: is 128 really necessary?
    pub x: i64,
    pub y: i64,
}
impl fmt::Debug for ScreenPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[wasm_bindgen]
impl ScreenPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}
impl ScreenPoint {
    pub fn from_tuple((x, y): (i64, i64)) -> Self {
        Self::new(x, y)
    }
    pub fn negate(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
#[derive(Default, Hash, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct CellPoint {
    pub x: BigInt,
    pub y: BigInt,
}
impl fmt::Debug for CellPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl CellPoint {
    pub fn new(x: BigInt, y: BigInt) -> Self {
        Self { x, y }
    }
    pub fn from_tuple((x, y): (BigInt, BigInt)) -> Self {
        Self::new(x, y)
    }
    pub fn negate(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
#[derive(Default, PartialEq, PartialOrd, Clone)]
pub struct WorldPoint {
    pub x: BigInt,
    pub y: BigInt,
}
impl fmt::Debug for WorldPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl WorldPoint {
    pub fn new(x: BigInt, y: BigInt) -> Self {
        Self { x, y }
    }
    pub fn from_tuple((x, y): (BigInt, BigInt)) -> Self {
        Self::new(x, y)
    }
    pub fn negate(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
