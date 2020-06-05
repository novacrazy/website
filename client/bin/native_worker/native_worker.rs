use wasm_bindgen::prelude::*;

use yew::agent::Threaded;

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    //multi_thread::native_worker::Worker::register();
}
