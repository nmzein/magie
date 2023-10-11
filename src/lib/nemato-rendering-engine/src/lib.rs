use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}