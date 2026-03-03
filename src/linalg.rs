use crate::ff::FiniteField;

pub struct Vector<const N: usize>([FiniteField; N]);

impl<const N: usize> Vector<N> {
    pub fn new<const Q: u32>(values: [FiniteField<Q>; N]) -> Self {
        Self(values)
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() % 4 == N);
        let mut fields = [FiniteField::zero(); N];
        for (i, chunk) in bytes.chunks(4).enumerate() {
            let bytes: [u8; 4] = chunk.try_into().unwrap();
            let value = u32::from_be_bytes(bytes);
            fields[i] = FiniteField::new(value);
        }
        Self(fields)
    }
}

impl<const N: usize> PartialEq for Vector<N> {
    fn eq(&self, other: &Self) -> bool {
        for (a, b) in self.0.iter().zip(other.0) {
            if *a != b {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ff::KyberFf;

    #[test]
    fn test_vector_from_bytes() {
        let zero = [0u8, 0u8, 0u8, 0u8];
        let f_zero = Vector::<1>::from_bytes(&zero);
    }
}
