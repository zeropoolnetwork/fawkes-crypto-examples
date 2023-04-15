mod utils;

use wasm_bindgen::prelude::*;
use fawkes_crypto_examples::{fibonacci_example, generate_parameters};

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn main() {
    alert("Generating parameters");
    let params = generate_parameters();

    alert("fibonacci_example(0)");
    fibonacci_example(&params, 0);

    alert("fibonacci_example(1)");
    fibonacci_example(&params, 1);

    alert("fibonacci_example(100)");
    fibonacci_example(&params, 100);
}
