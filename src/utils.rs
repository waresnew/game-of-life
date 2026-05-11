use ahash::AHashMap;
use wasm_bindgen::prelude::*;

use crate::quadtree::Quadtree;

#[cfg(target_arch = "wasm32")]
pub mod web {
    use web_sys::console;
    pub struct Timer<'a> {
        name: &'a str,
    }
    impl<'a> Timer<'a> {
        pub fn start(name: &'a str) -> Self {
            console::time_with_label(name);
            Self { name }
        }
    }
    impl<'a> Drop for Timer<'a> {
        fn drop(&mut self) {
            console::time_end_with_label(self.name);
        }
    }
}

pub fn decompose_bits(n: u64) -> Vec<u32> {
    let mut ans = Vec::new();
    let mut remaining = n.count_ones();
    for k in 0..64 {
        if (n >> k) & 1 == 1 {
            ans.push(k);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
    }
    ans
}
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Default)]
pub struct PerfStats {
    pub cache_hits: u64,
    pub cache_misses: u64,
}
pub fn update_dict(t: Quadtree, dict: &mut AHashMap<u64, Quadtree>) -> u64 {
    let hash = t.calc_hash();
    dict.insert(hash, t);
    hash
}
