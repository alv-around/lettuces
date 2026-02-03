use core::ops::{Add, Mul};

// INFO: source: https://eprint.iacr.org/2017/634.pdf
const KYBER_Q: u32 = 7681; // TODO: support u16
// TODO: find source
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

    #[test]
    fn test_field_wrapping() {
        assert!(DLITHIUM_Q < i32::MAX as u32);

        //zero
        assert!(KyberFf::new(KYBER_Q) == KyberFf::new(0));
        assert!(DlithiumFf::new(DLITHIUM_Q) == DlithiumFf::new(0));

        // one
        assert!(KyberFf::new(KYBER_Q + 1) == KyberFf::new(1));
        assert!(DlithiumFf::new(DLITHIUM_Q + 1) == DlithiumFf::new(1));

        // minus
        assert!(KyberFf::new(KYBER_Q - 1) == KyberFf::minus(1));
        assert!(DlithiumFf::new(DLITHIUM_Q - 1) == DlithiumFf::minus(1));
        assert!(KyberFf::minus(KYBER_Q + 1) == KyberFf::minus(1));
        assert!(DlithiumFf::minus(DLITHIUM_Q + 1) == DlithiumFf::minus(1));
    }

    #[test]
    fn test_field_addition() {
        assert!(KyberFf::new(1) + KyberFf::new(1) == KyberFf::new(2));
        assert!(KyberFf::minus(1) + KyberFf::new(1) == KyberFf::new(0));
        assert!(DlithiumFf::new(1) + DlithiumFf::new(1) == DlithiumFf::new(2));
        assert!(DlithiumFf::minus(1) + DlithiumFf::new(1) == DlithiumFf::new(0));
    }

    #[test]
    fn test_field_multiplication() {}
}
