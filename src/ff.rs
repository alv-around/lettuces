// INFO: source: https://eprint.iacr.org/2017/634.pdf
const KYBER_Q: u32 = 7681; // TODO: support u16
// TODO: find source
const DLITHIUM_Q: u32 = 8380417;

pub struct FiniteField<const Q: u32>(u32);

impl<const Q: u32> FiniteField<Q> {
    fn new(x: i32) -> Self {
        let x = if x.is_positive() {
            x as u32 % Q
        } else {
            let q = Q as i32;
            (((x % q) + q) % q) as u32
        };
        Self(x)
    }
}

impl<const Q: u32> PartialEq for FiniteField<Q> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub type KyberFf = FiniteField<KYBER_Q>;
pub type DlithiumFf = FiniteField<DLITHIUM_Q>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wrapping() {
        assert!(DLITHIUM_Q < i32::MAX as u32);

        //zero
        assert!(KyberFf::new(KYBER_Q as i32) == KyberFf::new(0));
        assert!(DlithiumFf::new(DLITHIUM_Q as i32) == DlithiumFf::new(0));

        // one
        assert!(KyberFf::new(KYBER_Q as i32 + 1) == KyberFf::new(1));
        assert!(DlithiumFf::new(DLITHIUM_Q as i32 + 1) == DlithiumFf::new(1));

        // minus one
        assert!(KyberFf::new(KYBER_Q as i32 - 1) == KyberFf::new(-1));
        assert!(DlithiumFf::new(DLITHIUM_Q as i32 - 1) == DlithiumFf::new(-1));
    }
}
