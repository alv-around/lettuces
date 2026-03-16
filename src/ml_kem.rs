use core::marker::PhantomData;
use core::ops::{Add, Rem, Sub};
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

use crate::ff::{FieldParams, FiniteField};
use crate::linalg::{Matrix, Vector};
// use no_std_io::io::{Read, Write};

pub struct MlKem<const N: usize, P: FieldParams> {
    _marker: PhantomData<P>,
}

impl<const N: usize, P: FieldParams + Copy> MlKem<N, P>
where
    P: FieldParams + PartialEq + Copy,
    P::Repr: Add<Output = P::Repr> + Sub<Output = P::Repr> + Rem<Output = P::Repr>,
    StandardUniform: Distribution<FiniteField<P>>,
{
    pub fn keygen<R: Rng>(rng: &mut R) -> Matrix<N, P> {
        let mut key = [Vector::new([FiniteField::<P>::zero(); N]); N];
        for coeffs in key.iter_mut() {
            *coeffs = Vector::random(rng);
        }
        key
    }

    // fn encrypt<'a>(key: Self::Key, text: &'a u8) -> impl Write {}
    // fn decrypt<'a>(key: Self::Key, ciphertxt: &'a u8) -> Result<impl Read, &'static str> {}
}
