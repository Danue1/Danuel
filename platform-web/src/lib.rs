mod utils;

use core_engine::{Builder, CoreEngine, CoreEngineBuilder, Version, WorldBuilder};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, hello!");
}

type Image = String;

#[wasm_bindgen]
pub fn builder() -> JsValue {
    let core_engine: CoreEngine<Image> = CoreEngineBuilder::new()
        .version(Version::new(0, 1, 0))
        .world(
            WorldBuilder::new()
                .width(256)
                .height(256)
                .name("Danuel".to_owned())
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();
    JsValue::from_serde(&core_engine).unwrap()
}
