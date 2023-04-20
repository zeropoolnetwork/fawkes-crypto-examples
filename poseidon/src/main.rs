use fawkes_crypto::{
    backend::plonk::{engines::Bn256, prover, setup::setup, verifier, Parameters},
    engines::bn256::Fr,
    ff_uint::Num,
    native::poseidon::poseidon,
};

mod circuit;

fn main() {
    let parameters = Parameters::<Bn256>::setup(12);

    let data = Num::from(1u64);
    let hash = poseidon::<Fr>(&[data], &*circuit::POSEIDON_PARAMS);

    println!("Compiling circuit...");
    let keys = setup::<_, _, _>(&parameters, circuit::circuit);
    println!("Circuit finished");

    println!("Generating proof...");
    let (inputs, snark_proof) = prover::prove(&parameters, &keys.1, &hash, &data, circuit::circuit);
    println!("Proof generated");

    println!("Verifying proof...");
    assert!(verifier::verify(
        &parameters,
        &keys.0,
        &snark_proof,
        &inputs
    ));
    println!("Proof verified");
}
