use rand_core::RngCore;
use std::ops::{Add, Mul, Neg, Sub};

pub trait Ring:
    Sized
    + Clone
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
{
    const ZERO: Self;

    const ONE: Self;

    fn random(rng: impl RngCore) -> Self;

    fn from_i64(n: i64) -> Self;

    fn is_zero(&self) -> bool {
        self == &Self::ZERO
    }
}

pub trait CommutativeRing: Ring {}
