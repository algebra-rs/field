//! This crate provides traits for working with fields.
use rand_core::RngCore;

/// This trait represents an element of a field.
pub trait Field:
    Sized
    + Eq
    + Copy
    + Clone
    + Default
{
    /// The zero element of the field, the additive identity.
    const ZERO: Self;

    /// The one element of the field, the multiplicative identity.
    const ONE: Self;

    fn random(rng: impl RngCore) -> Self;

    fn is_zero(&self) -> bool;

    fn is_zero_vartime(&self) -> bool {
        self.is_zero().into()
    }

    #[must_use]
    fn square(&self) -> Self;

    #[must_use]
    fn cube(&self) -> Self;

    #[must_use]
    fn double(&self) -> Self;

    fn invert(&self) -> Option<Self>;

    fn sqrt_ratio(num: &Self, div: &Self) -> (bool, Self);

    fn sqrt_alt(&self) -> (bool, Self) {
        Self::sqrt_ratio(self, &Self::ONE)
    }
    
    fn sqrt(&self) -> Option<Self>;

    fn pow<S: AsRef<[u64]>>(&self, exp: S) -> Self;

    fn pow_vartime<S: AsRef<[u64]>>(&self, exp: S) -> Self;
}
