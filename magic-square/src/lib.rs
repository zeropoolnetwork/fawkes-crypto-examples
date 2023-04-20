mod circuit;

use fawkes_crypto::{
    backend::plonk::{engines::Bn256, prover, setup::setup, verifier, Parameters},
    core::sizedvec::SizedVec,
    ff_uint::Num,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "magicCircle")]
pub fn magic_square(sum: u32, magic_square: &[u32], f: &js_sys::Function) {
    let callback = |msg: &str| {
        let this = JsValue::null();
        let msg = JsValue::from(msg);
        let _ = f.call1(&this, &msg);
    };

    let parameters = Parameters::<Bn256>::setup(10);

    callback("Compiling circuit...");
    let keys = setup::<_, _, _>(&parameters, circuit::circuit);
    callback("Circuit finished");

    callback(&format!(
        "Generating proof. Public input: {}, Private input: {:?}",
        sum, magic_square
    ));

    let n_magic_square = magic_square
        .iter()
        .map(|n| Num::from(*n))
        .collect::<SizedVec<_, 9>>();

    let (inputs, snark_proof) = prover::prove(
        &parameters,
        &keys.1,
        &Num::from(sum),
        &n_magic_square,
        circuit::circuit,
    );

    callback(&format!("Proof: {}", &hex::encode(&snark_proof.0)));

    callback(&format!("Verifying proof. Input: {:?}", &inputs));
    let result = verifier::verify(&parameters, &keys.0, &snark_proof, &inputs);
    callback(&format!("Verification result: {}", result));
}
