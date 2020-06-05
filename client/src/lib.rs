#![allow(unused_imports)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate serde;

use cfg_if::cfg_if;

pub mod components;
pub mod geometry;
pub mod views;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}
