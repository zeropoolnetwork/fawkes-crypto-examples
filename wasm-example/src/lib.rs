mod utils;

use fawkes_crypto_examples::{fibonacci_example, generate_parameters, multiplier_example};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
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
