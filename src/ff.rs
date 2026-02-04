use core::ops::{Add, Mul};

// INFO: source: https://eprint.iacr.org/2017/634.pdf
const KYBER_Q: u32 = 7681; // TODO: support u16
// INFO: source: https://eprint.iacr.org/2017/633.pdf
const DLITHIUM_Q: u32 = 8380417;

pub struct FiniteField<const Q: u32>(u32);

impl<const Q: u32> FiniteField<Q> {
    pub fn new(x: u32) -> Self {
        Self(x % Q)
    }

    pub fn minus(x: u32) -> Self {
        let i = x % Q;
        Self(Q - i)
    }

    pub fn zero() -> Self {
        FiniteField::new(0)
    }

    pub fn one() -> Self {
        FiniteField::new(1)
    }
}

impl<const Q: u32> PartialEq for FiniteField<Q> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<const Q: u32> Add for FiniteField<Q> {
    type Output = FiniteField<Q>;

    // strict because we know sofar that no sum of 2 Fields would overflow.
    // Kyber & Dlitiums qs are far from 2^32
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.strict_add(rhs.0) % Q)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<const Q: u32> Mul for FiniteField<Q> {
    type Output = FiniteField<Q>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mult_expanded = (self.0 as u64).strict_mul(rhs.0 as u64);
        let mod_mult = mult_expanded % (Q as u64);
        Self(mod_mult as u32)
    }
}

pub type KyberFf = FiniteField<KYBER_Q>;
pub type DlithiumFf = FiniteField<DLITHIUM_Q>;

#[cfg(test)]
mod test {
    use super::*;
    use rand::random_range;

    #[test]
    fn test_field_wrapping() {
        assert!(DLITHIUM_Q < i32::MAX as u32);

        //zero
        assert!(KyberFf::new(KYBER_Q) == KyberFf::zero());
        assert!(DlithiumFf::new(DLITHIUM_Q) == DlithiumFf::zero());

        // one
        assert!(KyberFf::new(KYBER_Q + 1) == KyberFf::one());
        assert!(DlithiumFf::new(DLITHIUM_Q + 1) == DlithiumFf::one());

        // minus
        assert!(KyberFf::new(KYBER_Q - 1) == KyberFf::minus(1));
        assert!(DlithiumFf::new(DLITHIUM_Q - 1) == DlithiumFf::minus(1));
        assert!(KyberFf::minus(KYBER_Q + 1) == KyberFf::minus(1));
        assert!(DlithiumFf::minus(DLITHIUM_Q + 1) == DlithiumFf::minus(1));
    }

    #[test]
    fn test_field_addition() {
        // addition
        assert!(KyberFf::one() + KyberFf::new(1) == KyberFf::new(2));
        assert!(KyberFf::minus(1) + KyberFf::one() == KyberFf::zero());
        assert!(DlithiumFf::one() + DlithiumFf::new(1) == DlithiumFf::new(2));
        assert!(DlithiumFf::minus(1) + DlithiumFf::one() == DlithiumFf::zero());
        assert!(KyberFf::minus(1) + KyberFf::new(2) == KyberFf::one());
        assert!(DlithiumFf::minus(1) + DlithiumFf::new(2) == DlithiumFf::one());
        assert!(KyberFf::new(2) + KyberFf::new(7) == KyberFf::new(9));
        assert!(DlithiumFf::new(2) + DlithiumFf::new(7) == DlithiumFf::new(9));

        // Addition by neutral element
        assert!(KyberFf::new(11) + KyberFf::zero() == KyberFf::new(11));
        assert!(DlithiumFf::new(11) + DlithiumFf::zero() == DlithiumFf::new(11));

        // use random numbers
        let a = random_range(0..=KYBER_Q);
        let b = random_range(0..=KYBER_Q);
        let c = a + b;
        assert!(KyberFf::new(a) + KyberFf::new(b) == KyberFf::new(c));
        let a = random_range(0..=DLITHIUM_Q);
        let b = random_range(0..=DLITHIUM_Q);
        let c = a + b;
        assert!(DlithiumFf::new(a) + DlithiumFf::new(b) == DlithiumFf::new(c));

        // substraction
        assert!(KyberFf::one() + KyberFf::minus(1) == KyberFf::zero());
        assert!(DlithiumFf::one() + DlithiumFf::minus(1) == DlithiumFf::zero());
    }

    #[test]
    fn test_field_multiplication() {
        // multiply by 0
        assert!(KyberFf::new(17) * KyberFf::zero() == KyberFf::zero());
        assert!(DlithiumFf::new(17) * DlithiumFf::zero() == DlithiumFf::zero());

        // multiply by 1
        assert!(KyberFf::new(17) * KyberFf::one() == KyberFf::new(17));
        assert!(DlithiumFf::new(17) * DlithiumFf::one() == DlithiumFf::new(17));

        // use random numbers
        let a = random_range(0..=KYBER_Q) as u64;
        let b = random_range(0..=KYBER_Q) as u64;
        let c = (a * b) % (KYBER_Q as u64);
        assert!(KyberFf::new(a as u32) * KyberFf::new(b as u32) == KyberFf::new(c as u32));
        let a = random_range(0..=DLITHIUM_Q) as u64;
        let b = random_range(0..=DLITHIUM_Q) as u64;
        let c = (a * b) % (DLITHIUM_Q as u64);
        assert!(DlithiumFf::new(a as u32) * DlithiumFf::new(b as u32) == DlithiumFf::new(c as u32));
    }
}
