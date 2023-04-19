mod utils;

use wasm_bindgen::prelude::*;
use fawkes_crypto_examples::{fibonacci_example, generate_parameters};

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
    fibonacci_example(&params);
}
