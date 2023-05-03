use fawkes_crypto::{
    circuit::{cs::CS, num::CNum},
    core::signal::Signal,
    ff_uint::Num,
};

/// Simple circuit that computes the Nth fibonacci number.
fn c_fibonacci<C: CS, const N: usize>(n: &CNum<C>) -> CNum<C> {
    let mut a: CNum<C> = n.derive_const(&Num::from(0));
    let mut b: CNum<C> = n.derive_const(&Num::from(1));
    let mut n: CNum<C> = n.clone();
    let one: CNum<C> = n.derive_const(&Num::from(1));

    for _ in 0..N {
        let is_zero = n.is_zero();

        // if n.is_zero() {
        //     continue;
        // } else {
        //     let tmp = &a + &b;
        //     a = b;
        //     b = tmp;
        //     n = &n - &n.derive_const(&Num::from(1));
        // }
        let tmp = &a + &b;
        a = a.switch(&is_zero, &b);
        b = b.switch(&is_zero, &tmp);
        n = n.switch(&is_zero, &(&n - &one));
    }

    a
}

/// Wrapper around `c_fibonacci` to make it usable in fawkes-crypto's `setup` and `prove` functions.
pub fn circuit<C: CS, const N: usize>(public: CNum<C>, secret: CNum<C>) {
    let num = c_fibonacci::<C, { N }>(&public);
    num.assert_eq(&secret);
}
