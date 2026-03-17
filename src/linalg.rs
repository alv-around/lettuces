use core::fmt;
use core::ops::{Add, Mul, Rem, Sub};
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

use crate::ff::{FieldParams, FiniteField};

#[derive(Clone, Copy)]
pub struct Vector<const N: usize, P: FieldParams>(pub [FiniteField<P>; N]);

impl<const N: usize, P: FieldParams> fmt::Debug for Vector<N, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl<const N: usize, P: FieldParams> Vector<N, P>
where
    P: FieldParams,
    P::Repr: Rem<Output = P::Repr> + Add<Output = P::Repr> + Sub<Output = P::Repr>,
    StandardUniform: Distribution<FiniteField<P>>,
{
    pub fn new(values: [P::Repr; N]) -> Self {
        Self(values.map(|v| FiniteField::new(v)))
    }

    pub fn add_scalar(&mut self, x: FiniteField<P>) {
        for field in self.0.iter_mut() {
            *field = *field + x;
        }
    }

    pub fn random<R: Rng>(rng: &mut R) -> Self {
        let mut rand_vals = [FiniteField::<P>::zero(); N];
        for val in rand_vals.iter_mut() {
            *val = rng.random();
        }
        Self(rand_vals)
    }
}

impl<const N: usize, P> PartialEq for Vector<N, P>
where
    P: FieldParams + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        for (a, b) in self.0.iter().zip(&other.0) {
            if *a != *b {
                return false;
            }
        }
        true
    }
}

impl<const N: usize, P> Add for Vector<N, P>
where
    P: FieldParams + PartialEq + Copy,
    P::Repr: Add<Output = P::Repr> + Sub<Output = P::Repr> + Rem<Output = P::Repr>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut array = [FiniteField::<_>::zero(); N];
        for (i, a) in self.0.into_iter().enumerate() {
            let b = other.0[i];
            array[i] = a + b;
        }
        Self(array)
    }
}

impl<const N: usize, P> Mul for Vector<N, P>
where
    P: FieldParams + PartialEq + Copy,
    P::Repr: Add<Output = P::Repr>
        + Mul<Output = P::Repr>
        + Rem<Output = P::Repr>
        + Sub<Output = P::Repr>,
{
    type Output = FiniteField<P>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut dot_product = FiniteField::<_>::zero();
        for i in 0..N {
            dot_product = dot_product + self.0[i] * rhs.0[i]
        }
        dot_product
    }
}

pub struct Matrix<const N: usize, P: FieldParams>(pub [Vector<N, P>; N]);

impl<const N: usize, P: FieldParams> fmt::Debug for Matrix<N, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[")?;
        for vec in self.0.iter() {
            writeln!(f, "{:?}", vec)?
        }
        writeln!(f, "]")
    }
}

impl<const N: usize, P: FieldParams> Matrix<N, P>
where
    P: FieldParams + PartialEq + Copy,
    P::Repr: Add<Output = P::Repr> + Sub<Output = P::Repr> + Rem<Output = P::Repr>,
    StandardUniform: Distribution<FiniteField<P>>,
{
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        let mut key = [Vector([FiniteField::zero(); N]); N];
        for coeffs in key.iter_mut() {
            *coeffs = Vector::random(rng);
        }
        Matrix(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ff::{KyberFp, KyberParams};

    const ZEROES: [KyberFp; 4] = [KyberFp::zero(); 4];
    const KYBER_V: Vector<4, KyberParams> = Vector(ZEROES);

    #[test]
    fn test_vector() {
        assert!(KYBER_V == Vector::new([0; 4]));
        assert_ne!(KYBER_V, Vector::new([1; 4]));
    }

    #[test]
    fn test_vector_addition() {
        let array: Vector<4, KyberParams> = Vector::new([1, 2, 3, 4]);
        let array_rev: Vector<_, KyberParams> = Vector::new([4, 3, 2, 1]);
        let sum = array + array_rev;
        assert_eq!(sum, Vector::new([3, 1, 3, 1]));
    }

    #[test]
    fn test_vector_scalar_addition() {
        let mut ones = Vector(ZEROES);
        ones.add_scalar(KyberFp::new(1));
        assert_eq!(ones, Vector::new([1, 1, 1, 1]));
    }

    #[test]
    fn test_vector_dot_product() {
        let basis_vector_1 = Vector::new([1, 0]);
        let basis_vector_2 = Vector::new([0, 1]);

        assert_eq!(basis_vector_1 * basis_vector_2, KyberFp::zero());
    }
}
