extern crate nova_client;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("WASM Started!");
}

#[wasm_bindgen]
pub fn run_app() {
    use nova_client::views::{MainView, Properties};

    log::info!("Bootstrapping main...");

    yew::start_app_with_props::<MainView>(Properties {})
}
