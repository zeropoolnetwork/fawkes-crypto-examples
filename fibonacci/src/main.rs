use fawkes_crypto::{
    backend::plonk::{engines::Bn256, prover, setup::setup, verifier, Parameters},
    ff_uint::Num,
};

mod circuit;

const N: usize = 10;

fn main() {
    // For testing
    fn fibonacci_number(n: usize) -> u64 {
        let (mut a, mut b) = (0, 1);

        for _ in 0..n - 1 {
            let next = a + b;
            a = b;
            b = next;
        }

        b
    }

    let parameters = Parameters::<Bn256>::setup(10);

    println!("Compiling circuit...");
    let keys = setup::<_, _, _>(&parameters, circuit::circuit::<_, { N }>);
    println!("Circuit finished");

    let num = fibonacci_number(N);

    println!("Generating proof...");
    let (inputs, snark_proof) = prover::prove(
        &parameters,
        &keys.1,
        &Num::from(N as u64),
        &Num::from(num),
        circuit::circuit::<_, { N }>,
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
