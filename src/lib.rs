use fawkes_crypto::{
    backend::plonk::{engines::Bn256, setup::setup, *},
    ff_uint::Num,
};

mod circuits;

const N: usize = 10;

pub fn fibonacci_example(parameters: &Parameters<Bn256>) -> bool {
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

    let keys = setup::<_, _, _>(&parameters, circuits::fibonacci::circuit::<_, { N }>);
    println!("Circuit finished");

    let num = fibonacci_number(N);

    let (inputs, snark_proof) = prover::prove(
        &parameters,
        &keys.1,
        &Num::from(N as u64),
        &Num::from(num),
        circuits::fibonacci::circuit::<_, { N }>,
    );
    println!("Proof generated");

    verifier::verify(&parameters, &keys.0, &snark_proof, &inputs)
}

pub fn multiplier_example(parameters: &Parameters<Bn256>, a: u64, b: u64, c: u64) -> bool {
    let keys = setup::<_, _, _>(&parameters, circuits::multiplier::circuit);
    println!("Circuit finished");

    let (inputs, snark_proof) = prover::prove(
        &parameters,
        &keys.1,
        &Num::from(c),
        &(Num::from(a), Num::from(b)),
        circuits::multiplier::circuit,
    );
    println!("Proof generated");

    verifier::verify(&parameters, &keys.0, &snark_proof, &inputs)
}

pub fn generate_parameters() -> Parameters<Bn256> {
    Parameters::<Bn256>::setup(10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_example() {
        let parameters = generate_parameters();

        assert!(fibonacci_example(&parameters));
    }

    #[test]
    fn test_multiplier_example() {
        let parameters = generate_parameters();

        assert!(multiplier_example(&parameters, 5, 10, 50));
    }
}
