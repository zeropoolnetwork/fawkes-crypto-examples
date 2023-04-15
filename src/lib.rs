use fawkes_crypto::ff_uint::Num;
use fawkes_crypto::{
    backend::plonk::{engines::Bn256, setup::setup, *},
    circuit::cs::CS,
    circuit::num::CNum,
    core::signal::Signal,
};

pub fn generate_parameters() -> Parameters<Bn256> {
    Parameters::<Bn256>::setup(10)
}

pub fn fibonacci_example(parameters: &Parameters<Bn256>, n: usize) -> bool {
    fn circuit<C: CS>(public: CNum<C>, secret: (CNum<C>, CNum<C>)) {
        c_fibonacci::<C>(&secret.0, &secret.1, &public);
    }

    let keys = setup::<_, _, _>(&parameters, circuit);
    println!("Circuit finished");

    let seq = fibonacci_sequence(n + 1);
    let fib = seq[n];
    let (fib1, fib2) = if n == 0 {
        (0, 0)
    } else if n == 1 {
        (1, 0)
    } else {
        (seq[n - 1], seq[n - 2])
    };

    let (inputs, snark_proof) = prover::prove(
        &parameters,
        &keys.1,
        &Num::from(fib),
        &(Num::from(fib1), Num::from(fib2)),
        circuit,
    );
    println!("Proof generated");

    verifier::verify(&parameters, &keys.0, &snark_proof, &inputs)
}

fn c_fibonacci<C: CS>(a: &CNum<C>, b: &CNum<C>, c: &CNum<C>) {
    (a + b).assert_eq(c);
}

fn fibonacci_sequence(len: usize) -> Vec<u64> {
    let mut seq = Vec::with_capacity(len);

    let (mut a, mut b) = (0, 1);

    for _ in 0..len.max(1) {
        seq.push(a);
        let next = a + b;
        a = b;
        b = next;
    }

    seq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_example() {
        let parameters = generate_parameters();

        assert!(fibonacci_example(&parameters, 0));
        assert!(fibonacci_example(&parameters, 1));
        assert!(fibonacci_example(&parameters, 100));
    }
}
