#[macro_use]
extern crate serde;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

pub mod geometry;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(a: f32, b: f32, c: f32) {
    let x = a.mul_add(b, c);

    alert(&format!("Hello Humans 2, {}!", x ));
}