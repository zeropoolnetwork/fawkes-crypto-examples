use fawkes_crypto::ff_uint::Num;
use fawkes_crypto::{
    backend::plonk::{engines::Bn256, setup::setup, *},
    circuit::cs::CS,
    circuit::num::CNum,
    core::signal::Signal,
};

mod circuit;

const N: usize = 10;

pub fn fibonacci_example(parameters: &Parameters<Bn256>) -> bool {
    let keys = setup::<_, _, _>(&parameters, circuit::circuit::<_, { N }>);
    println!("Circuit finished");

    let num = fibonacci_number(N);

    let (inputs, snark_proof) = prover::prove(
        &parameters,
        &keys.1,
        &Num::from(N as u64),
        &Num::from(num),
        circuit::circuit::<_, { N }>,
    );
    println!("Proof generated");

    verifier::verify(&parameters, &keys.0, &snark_proof, &inputs)
}

fn fibonacci_number(n: usize) -> u64 {
    let (mut a, mut b) = (0, 1);

    for _ in 0..n-1 {
        let next = a + b;
        a = b;
        b = next;
    }

    b
}

pub fn generate_parameters() -> Parameters<Bn256> {
    Parameters::<Bn256>::setup(10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_example() {
        let parameters = generate_parameters();

        assert!(fibonacci_example(&parameters));
    }
}
