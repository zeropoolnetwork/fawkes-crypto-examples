use fawkes_crypto::{
    backend::plonk::{engines::Bn256, prover, setup::setup, verifier, Parameters},
    ff_uint::Num,
};

mod circuit;

fn main() {
    let (a, b, c) = (2, 3, 6);

    let parameters = Parameters::<Bn256>::setup(10);

    println!("Compiling circuit...");
    let keys = setup::<_, _, _>(&parameters, circuit::circuit);
    println!("Circuit finished");

    println!("Generating proof...");
    let (inputs, snark_proof) = prover::prove(
        &parameters,
        &keys.1,
        &Num::from(c),
        &(Num::from(a), Num::from(b)),
        circuit::circuit,
    );
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
