use fawkes_crypto::{
    circuit::{cs::CS, num::CNum, poseidon::c_poseidon},
    core::signal::Signal,
    engines::bn256::Fr,
    native::poseidon::PoseidonParams,
};
use once_cell::sync::Lazy;

pub static POSEIDON_PARAMS: Lazy<PoseidonParams<Fr>> =
    Lazy::new(|| PoseidonParams::<Fr>::new(6, 8, 53));

pub fn circuit<C: CS<Fr = Fr>>(public: CNum<C>, secret: CNum<C>) {
    let h = c_poseidon(&[secret], &*POSEIDON_PARAMS);
    h.assert_eq(&public);
}
