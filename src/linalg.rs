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

    // TODO: refactor
    #[allow(non_snake_case)]
    pub fn dot_product(self, A: Matrix<N, P>) -> Vector<N, P> {
        let mut result = [FiniteField::zero(); N];
        for (i, val) in result.iter_mut().enumerate() {
            let mut vec = [FiniteField::zero(); N];
            for (j, row) in A.0.iter().enumerate() {
                vec[j] = self.0[j] * row.0[i];
            }
            *val = vec.into_iter().sum();
        }
        Vector(result)
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
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut product = [FiniteField::<P>::zero(); N];
        for (i, val) in product.iter_mut().enumerate() {
            *val = self.0[i] * rhs.0[i]
        }
        Vector(product)
    }
}

#[derive(Clone, Copy)]
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
    P::Repr: Add<Output = P::Repr>
        + Sub<Output = P::Repr>
        + Rem<Output = P::Repr>
        + Mul<Output = P::Repr>,
    StandardUniform: Distribution<FiniteField<P>>,
{
    pub fn random<R: Rng>(rng: &mut R) -> Self {
        let mut key = [Vector([FiniteField::zero(); N]); N];
        for coeffs in key.iter_mut() {
            *coeffs = Vector::random(rng);
        }
        Matrix(key)
    }

    pub fn dot_product(self, v: Vector<N, P>) -> Vector<N, P> {
        let mut result = [FiniteField::zero(); N];
        for (i, val) in self.0.into_iter().enumerate() {
            result[i] = (val * v).0.into_iter().sum();
        }
        Vector(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ff::{KyberFp, KyberParams};
    use rand::{SeedableRng, rngs::SmallRng};

    const ZEROES: [KyberFp; 4] = [KyberFp::zero(); 4];
    const KYBER_V: Vector<4, KyberParams> = Vector(ZEROES);
    const KYBER_Q: u16 = KyberParams::MODULUS;

    #[test]
    fn test_vector() {
        assert!(KYBER_V == Vector::new([0; 4]));
        assert_ne!(KYBER_V, Vector::new([1; 4]));
    }

    #[test]
    fn test_vector_addition() {
        let array: Vector<4, KyberParams> = Vector::new([1, 2, 3, 4]);
        let array_rev: Vector<_, KyberParams> =
            Vector::new([KYBER_Q - 4, KYBER_Q - 3, KYBER_Q - 2, KYBER_Q - 1]);
        let sum = array + array_rev;
        assert_eq!(
            sum,
            Vector([
                KyberFp::minus(3),
                KyberFp::minus(1),
                KyberFp::new(1),
                KyberFp::new(3)
            ])
        );
    }

    #[test]
    fn test_vector_scalar_addition() {
        let mut ones = Vector(ZEROES);
        ones.add_scalar(KyberFp::new(1));
        assert_eq!(ones, Vector::new([1, 1, 1, 1]));
    }

    #[test]
    fn test_vector_dot_product() {
        let basis_vector_1 = Vector::<_, KyberParams>::new([1, 0]);
        let basis_vector_2 = Vector::new([0, 1]);

        assert_eq!(basis_vector_1 * basis_vector_2, Vector::new([0, 0]));
    }

    #[test]
    fn test_diffie_hellmann() {
        let mut rng = SmallRng::from_seed([0u8; 32]);
        #[allow(non_snake_case)]
        let A: Matrix<8, KyberParams> = Matrix::random(&mut rng);

        // key for Alice
        let s: Vector<8, KyberParams> = Vector::random(&mut rng);
        let t = A.dot_product(s);

        // key for Bob
        let r: Vector<8, KyberParams> = Vector::random(&mut rng);
        let u = r.dot_product(A);

        let us: KyberFp = (u * s).0.into_iter().sum();
        let rt: KyberFp = (r * t).0.into_iter().sum();
        assert_eq!(us, rt);
    }
}
