use std::panic;

use wasm_bindgen::prelude::*;

mod baseball;
mod games;
mod lacrosse;
mod rankings;
mod softball;
mod sport;
mod team;

#[wasm_bindgen]
pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

#[wasm_bindgen]
pub fn init_panics() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

// #[wasm_bindgen(getter_with_clone)]
// pub struct Team {

// }
