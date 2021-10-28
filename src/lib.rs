use wasm_bindgen::prelude::*;

mod utils;
mod entities;

use entities::universe::Universe;

extern crate web_sys;


macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    log!("hello ");
    // alert("Hello, canvas!");
}

#[wasm_bindgen]
pub fn create() {
    let universe = Universe::new();
}