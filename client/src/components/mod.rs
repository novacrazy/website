pub mod body;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn bindgen_start() {
    log!("WASM Starting...");
}

#[wasm_bindgen]
pub fn run_app() {
    log!("Bootstrapping body...");

    yew::App::<body::Model>::new().mount_to_body_with_props(body::Properties {});
}
