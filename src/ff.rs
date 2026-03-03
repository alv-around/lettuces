use core::marker::PhantomData;
use core::ops::{Add, Mul, Rem, Sub};

// Kyber Setup
#[derive(PartialEq)]
pub struct KyberParams;

// INFO: source: https://eprint.iacr.org/2017/634.pdf
impl FieldParams for KyberParams {
    type Repr = u16;
    const MODULUS: u16 = 7681;

    fn mul_reduce(lhs: u16, rhs: u16) -> u16 {
        let intermediate = (lhs as u32) * (rhs as u32);
        let reduced = intermediate % (Self::MODULUS as u32);
        reduced as u16
    }
}

// Dilithium Setup
#[derive(PartialEq)]
pub struct DlithiumParams;

// INFO: source: https://eprint.iacr.org/2017/633.pdf
impl FieldParams for DlithiumParams {
    type Repr = u32;
    const MODULUS: u32 = 8380417;

    fn mul_reduce(lhs: u32, rhs: u32) -> u32 {
        let intermediate = (lhs as u64) * (rhs as u64);
        let reduced = intermediate % (Self::MODULUS as u64);
        reduced as u32
    }
}

// Fps
pub type KyberFp = FiniteField<KyberParams>;
pub type DlithiumFp = FiniteField<DlithiumParams>;

pub trait FieldParams {
    /// The underlying storage type (e.g., u16, u32)
    type Repr: Copy + Default + PartialOrd;
    /// The prime modulus (Q)
    const MODULUS: Self::Repr;

    fn mul_reduce(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FiniteField<P: FieldParams> {
    value: P::Repr,
    _marker: PhantomData<P>,
}

impl<P: FieldParams> FiniteField<P>
where
    P::Repr: Rem<Output = P::Repr> + Sub<Output = P::Repr>,
{
    pub fn new(x: P::Repr) -> Self {
        let value = x % P::MODULUS as P::Repr;
        Self {
            value,
            _marker: PhantomData,
        }
    }

    pub fn minus(x: P::Repr) -> Self {
        let q = P::MODULUS as P::Repr;
        let i = x % q;
        Self::new(q - i)
    }

    // TODO: create const constructor new(0) and new(1)
}

impl<P: FieldParams> Add for FiniteField<P>
where
    P::Repr: Add<Output = P::Repr> + PartialOrd + Sub<Output = P::Repr>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = self.value + rhs.value;
        if sum >= P::MODULUS {
            sum = sum - P::MODULUS;
        }

        Self {
            value: sum,
            _marker: PhantomData,
        }
    }
}

impl<P: FieldParams> Mul for FiniteField<P>
where
    P::Repr: PartialOrd,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let value = P::mul_reduce(self.value, rhs.value);

        Self {
            value,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    extern crate std;
    use rand::random_range;

    const KYBER_Q: u16 = KyberParams::MODULUS;
    const DLITHIUM_Q: u32 = DlithiumParams::MODULUS;

    #[test]
    fn test_field_wrapping() {
        assert!(KYBER_Q < i16::MAX as u16);
        assert!(DLITHIUM_Q < i32::MAX as u32);

        //zero
        assert!(KyberFp::new(KYBER_Q) == KyberFp::new(0));
        assert!(DlithiumFp::new(DLITHIUM_Q) == DlithiumFp::new(0));

        // one
        assert!(KyberFp::new(KYBER_Q + 1) == KyberFp::new(1));
        assert!(DlithiumFp::new(DLITHIUM_Q + 1) == DlithiumFp::new(1));

        // // minus
        assert!(KyberFp::new(KYBER_Q - 1) == KyberFp::minus(1));
        assert!(DlithiumFp::new(DLITHIUM_Q - 1) == DlithiumFp::minus(1));
        assert!(KyberFp::minus(KYBER_Q + 1) == KyberFp::minus(1));
        assert!(DlithiumFp::minus(DLITHIUM_Q + 1) == DlithiumFp::minus(1));
    }

    #[test]
    fn test_field_addition() {
        // addition
        assert!(KyberFp::new(1) + KyberFp::new(1) == KyberFp::new(2));
        assert!(KyberFp::minus(1) + KyberFp::new(1) == KyberFp::new(0));
        assert!(DlithiumFp::new(1) + DlithiumFp::new(1) == DlithiumFp::new(2));
        assert!(DlithiumFp::minus(1) + DlithiumFp::new(1) == DlithiumFp::new(0));
        assert!(KyberFp::minus(1) + KyberFp::new(2) == KyberFp::new(1));
        assert!(DlithiumFp::minus(1) + DlithiumFp::new(2) == DlithiumFp::new(1));
        assert!(KyberFp::new(2) + KyberFp::new(7) == KyberFp::new(9));
        assert!(DlithiumFp::new(2) + DlithiumFp::new(7) == DlithiumFp::new(9));

        // Addition by neutral element
        assert!(KyberFp::new(11) + KyberFp::new(0) == KyberFp::new(11));
        assert!(DlithiumFp::new(11) + DlithiumFp::new(0) == DlithiumFp::new(11));

        // use random numbers
        let a = random_range(0..=KYBER_Q);
        let b = random_range(0..=KYBER_Q);
        let c = a + b;
        assert!(KyberFp::new(a) + KyberFp::new(b) == KyberFp::new(c));
        let a = random_range(0..=DLITHIUM_Q);
        let b = random_range(0..=DLITHIUM_Q);
        let c = a + b;
        assert!(DlithiumFp::new(a) + DlithiumFp::new(b) == DlithiumFp::new(c));

        // substraction
        assert!(KyberFp::new(1) + KyberFp::minus(1) == KyberFp::new(0));
        assert!(DlithiumFp::new(1) + DlithiumFp::minus(1) == DlithiumFp::new(0));
    }

    #[test]
    fn test_field_multiplication() {
        // multiply by 0
        assert!(KyberFp::new(17) * KyberFp::new(0) == KyberFp::new(0));
        assert!(DlithiumFp::new(17) * DlithiumFp::new(0) == DlithiumFp::new(0));

        // multiply by 1
        assert!(KyberFp::new(17) * KyberFp::new(1) == KyberFp::new(17));
        assert!(DlithiumFp::new(17) * DlithiumFp::new(1) == DlithiumFp::new(17));

        // use random numbers
        let a = random_range(0..=KYBER_Q);
        let b = random_range(0..=KYBER_Q);
        let c = (a as u32 * b as u32) % KYBER_Q as u32;
        assert!(KyberFp::new(a) * KyberFp::new(b) == KyberFp::new(c.try_into().unwrap()));
        let a = random_range(0..=DLITHIUM_Q);
        let b = random_range(0..=DLITHIUM_Q);
        let c = (a as u64 * b as u64) % DLITHIUM_Q as u64;
        assert!(DlithiumFp::new(a) * DlithiumFp::new(b) == DlithiumFp::new(c.try_into().unwrap()));
    }
}
