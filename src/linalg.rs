use crate::ff::{FieldParams, FiniteField};
use core::ops::{Rem, Sub};

pub struct Vector<const N: usize, P: FieldParams>([FiniteField<P>; N]);

impl<const N: usize, P: FieldParams> Vector<N, P>
where
    P: FieldParams,
    P::Repr: Rem<Output = P::Repr> + Sub<Output = P::Repr>,
{
    pub fn new(values: [FiniteField<P>; N]) -> Self {
        Self(values)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ff::KyberParams;

    #[test]
    fn test_vector_equality() {
        todo!("assert vector equality!");
    }
}
