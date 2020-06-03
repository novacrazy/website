pub mod body;
pub mod bootstrap;
pub mod navbar;
pub mod views;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn bindgen_start() {
    log!("WASM Starting...");
}

#[wasm_bindgen]
pub fn run_app() {
    log!("Bootstrapping body...");

    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app_with_props::<body::Model>(body::Properties {})
}
