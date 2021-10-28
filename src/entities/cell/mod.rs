use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Cell {
    x: i32,
    y: i32,    
}

impl Cell {
    pub fn new() -> Cell {
        Cell{x: 0, y:0}
    }
}