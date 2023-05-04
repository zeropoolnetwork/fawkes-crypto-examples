use fawkes_crypto::{
    circuit::{cs::CS, num::CNum, bool::CBool},
    core::signal::Signal,
    ff_uint::Num,
};

/// Simple circuit that computes the nth fibonacci number.
fn c_fibonacci<C: CS, const N: usize>(n: &CNum<C>) -> CNum<C> {
    let mut a: CNum<C> = n.derive_const(&Num::from(0));
    let mut b: CNum<C> = n.derive_const(&Num::from(1));

    let mut res = a.clone();

    for i in 1..N {
        // Regular Fibonacci iteration.
        let tmp = &a + &b;
        a = b.clone();
        b = tmp;

        // Check if n == i, and update res if so.
        let i_const: CNum<C> = n.derive_const(&Num::from(i as u32));
        let update_res: CBool<C> = n.is_eq(&i_const);
        res = a.switch(&update_res, &res);
    }

    res
}

/// Wrapper around `c_fibonacci` to make it usable in fawkes-crypto's `setup` and `prove` functions.
pub fn circuit<C: CS, const N: usize>(public: CNum<C>, secret: CNum<C>) {
    let num = c_fibonacci::<C, { N }>(&public);
    num.assert_eq(&secret);
}
