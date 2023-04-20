use fawkes_crypto::circuit::cs::CS;
use fawkes_crypto::circuit::num::CNum;
use fawkes_crypto::core::signal::Signal;
use fawkes_crypto::ff_uint::Num;

/// Simple circuit that computes the Nth fibonacci number.
fn c_fibonacci<C: CS, const N: usize>(n: &CNum<C>, num: &CNum<C>) {
    let c_n: CNum<C> = n.derive_const(&Num::from(N as u64));
    n.assert_eq(&c_n);
    let mut a: CNum<C> = num.derive_const(&Num::from(0));
    let mut b: CNum<C> = num.derive_const(&Num::from(1));

    for _ in 0..N-1 {
        let tmp = &a + &b;
        a = b;
        b = tmp;
    }

    b.assert_eq(num);
}

/// Wrapper around `c_fibonacci` to make it usable in fawkes-crypto's `setup` and `prove` functions.
pub fn circuit<C: CS, const N: usize>(public: CNum<C>, secret: CNum<C>) {
    c_fibonacci::<C, { N }>(&public, &secret);
}