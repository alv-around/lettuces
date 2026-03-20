use core::ops::{Add, Mul, Rem, Sub};
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

use crate::ff::{FieldParams, FiniteField};
use crate::linalg::Matrix;

pub trait PubKeyEncryption {
    type Key;
    fn keygen<R: Rng>(rng: &mut R) -> Self::Key;
    fn encrypt<'a>(msg: &'a str, key: &Self::Key) -> &'a [u8];
    fn decrypt<'a>(cyphertxt: &'a [u8], key: &Self::Key) -> Result<&'a str, &'static str>;
}

pub struct MlKem<const N: usize, P: FieldParams>(pub P);

impl<const N: usize, P> PubKeyEncryption for MlKem<N, P>
where
    P: FieldParams + PartialEq + Copy,
    P::Repr: Rem<Output = P::Repr>
        + Mul<Output = P::Repr>
        + Add<Output = P::Repr>
        + Sub<Output = P::Repr>,
    StandardUniform: Distribution<FiniteField<P>>,
{
    type Key = Matrix<N, P>;

    #[allow(non_snake_case)]
    fn keygen<R: Rng>(rng: &mut R) -> Self::Key {
        Matrix::<N, P>::random(rng)
    }

    fn encrypt<'a>(msg: &'a str, key: &Self::Key) -> &'a [u8] {
        todo!()
    }

    fn decrypt<'a>(cyphertxt: &'a [u8], key: &Self::Key) -> Result<&'a str, &'static str> {
        todo!()
    }
}
