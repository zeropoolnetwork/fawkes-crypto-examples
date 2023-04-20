mod utils;

use wasm_bindgen::prelude::*;
use fawkes_crypto_examples::{fibonacci_example, multiplier_example, generate_parameters};

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn main() {
    log("Generating parameters");
    let params = generate_parameters();

    log("Starting fibonacci_example");
    assert!(fibonacci_example(&params));

    log("Starting multiplier_example");
    assert!(multiplier_example(&params, 2, 3, 6));
}
