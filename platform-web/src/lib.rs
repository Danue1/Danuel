use core_engine::{Builder, CoreEngine, CoreEngineBuilder, Version, WorldBuilder};
use wasm_bindgen::prelude::*;

type Image = String;

#[wasm_bindgen]
pub fn builder() {
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
    let _ = core_engine.run();
}
