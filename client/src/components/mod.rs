pub mod body;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    log!("Bootstrapping body...");

    yew::App::<body::BodyModel>::new().mount_to_body_with_props(body::Properties {});
}
