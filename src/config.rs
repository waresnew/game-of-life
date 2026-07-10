use crate::renderer::WorldPoint;
use tsify::Ts;
use tsify::Tsify;

use serde::Serialize;
use wasm_bindgen::prelude::*;
//serde the config to avoid wasm calls every time js reads them
macro_rules! define_config {
    ($($name:ident: $type:ty = $val:expr,)*) => {
        $(pub const $name: $type = $val;)*

        #[allow(non_snake_case)]
        #[derive(Serialize, Tsify)]
        pub struct Config {
            $(pub $name:$type,)*
        }

        #[wasm_bindgen]
        pub fn get_config()->Ts<Config> {
            let config=Config {
                $($name: $val,)*
            };
            config.into_ts().unwrap()
        }
    };
}
define_config! {
    MAX_HEIGHT:u32=47,
    MIN_POINT:WorldPoint=WorldPoint {

        x: -1 << (MAX_HEIGHT - 1),
        y: -1 << (MAX_HEIGHT - 1),
    },
    CELL_SIZE_EXP:u32=5,
    RENDER_OUTPUT_SIZE:usize=3,
}
