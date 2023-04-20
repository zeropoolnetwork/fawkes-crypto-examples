mod circuit;

use fawkes_crypto::{
    backend::plonk::{engines::Bn256, prover, setup::setup, verifier, Parameters},
    core::sizedvec::SizedVec,
    ff_uint::Num,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
}

#[wasm_bindgen(js_name = "magicCircle")]
pub fn magic_square(sum: u32, magic_square: &[u32]) -> bool {
    let parameters = Parameters::<Bn256>::setup(10);

    let magic_square = magic_square
        .iter()
        .map(|n| Num::from(*n))
        .collect::<SizedVec<_, 9>>();

    log("Compiling circuit...");
    let keys = setup::<_, _, _>(&parameters, circuit::circuit);
    log("Circuit finished");

    log("Generating proof...");
    let (inputs, snark_proof) = prover::prove(
        &parameters,
        &keys.1,
        &Num::from(sum),
        &magic_square,
        circuit::circuit,
    );
    log("Proof generated");

    log("Verifying proof...");
    verifier::verify(&parameters, &keys.0, &snark_proof, &inputs)
}
