use fawkes_crypto::{
    circuit::{cs::CS, num::CNum},
    core::{signal::Signal, sizedvec::SizedVec},
};

pub fn circuit<C: CS>(public: CNum<C>, secret: SizedVec<CNum<C>, 9>) {
    (&secret[0] + &secret[1] + &secret[2]).assert_eq(&public);
    (&secret[3] + &secret[4] + &secret[5]).assert_eq(&public);
    (&secret[6] + &secret[7] + &secret[8]).assert_eq(&public);

    (&secret[0] + &secret[3] + &secret[6]).assert_eq(&public);
    (&secret[1] + &secret[4] + &secret[7]).assert_eq(&public);
    (&secret[2] + &secret[5] + &secret[8]).assert_eq(&public);

    (&secret[0] + &secret[4] + &secret[8]).assert_eq(&public);
    (&secret[2] + &secret[4] + &secret[6]).assert_eq(&public);
}
