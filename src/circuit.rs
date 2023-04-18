use fawkes_crypto::circuit::cs::CS;
use fawkes_crypto::circuit::num::CNum;
use fawkes_crypto::core::signal::Signal;

fn c_fibonacci<C: CS>(a: &CNum<C>, b: &CNum<C>, c: &CNum<C>) {
    (a + b).assert_eq(c);
}

pub fn circuit<C: CS>(public: CNum<C>, secret: (CNum<C>, CNum<C>)) {
    c_fibonacci::<C>(&secret.0, &secret.1, &public);
}