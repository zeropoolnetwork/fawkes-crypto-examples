use fawkes_crypto::circuit::cs::CS;
use fawkes_crypto::circuit::num::CNum;
use fawkes_crypto::core::signal::Signal;


fn c_multiplier<C: CS>(a: &CNum<C>, b: &CNum<C>) -> CNum<C> {
    a * b
}

pub fn circuit<C: CS>(public: CNum<C>, secret: (CNum<C>, CNum<C>)) {
    let c = c_multiplier(&secret.0, &secret.1);
    c.assert_eq(&public);
}