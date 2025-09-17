use num_bigint::BigInt;
use std::ops::{Add, Sub, Mul, Neg};

pub trait Ring:
    Sized
    + Clone
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;

    fn from_i64(n: i64) -> Self;

    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}

impl Ring for BigInt {
    fn zero() -> Self { BigInt::from(0) }
    fn one() -> Self { BigInt::from(1) }

    fn from_i64(n: i64) -> Self { BigInt::from(n) }
}
