use fawkes_crypto::{
    backend::plonk::{engines::Bn256, prover, setup::setup, verifier, Parameters},
    ff_uint::Num,
};

mod circuit;

fn main() {
    let a = std::env::args()
        .nth(1)
        .map(|s| s.parse::<u64>().unwrap())
        .unwrap_or(2);
    let b = std::env::args()
        .nth(2)
        .map(|s| s.parse::<u64>().unwrap())
        .unwrap_or(3);
    let c = std::env::args()
        .nth(3)
        .map(|s| s.parse::<u64>().unwrap())
        .unwrap_or(a * b);

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
