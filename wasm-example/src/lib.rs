mod circuit;

use fawkes_crypto::{
    backend::plonk::{engines::Bn256, prover, setup::setup, verifier, Parameters},
    ff_uint::Num,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn main() {
    let (a, b, c) = (2, 3, 6);

    let parameters = Parameters::<Bn256>::setup(10);

    log("Compiling circuit...");
    let keys = setup::<_, _, _>(&parameters, circuit::circuit);
    log("Circuit finished");

    log("Generating proof...");
    let (inputs, snark_proof) = prover::prove(
        &parameters,
        &keys.1,
        &Num::from(c),
        &(Num::from(a), Num::from(b)),
        circuit::circuit,
    );
    log("Proof generated");

    log("Verifying proof...");
    assert!(verifier::verify(
        &parameters,
        &keys.0,
        &snark_proof,
        &inputs
    ));

    alert("Proof verified");
}
