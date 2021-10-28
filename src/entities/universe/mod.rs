use crate::entities::cell::Cell;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn new() -> Universe {
        let cell = Cell::new();
        Universe { width: 0, height: 0, cells: vec![cell] }
    }
}