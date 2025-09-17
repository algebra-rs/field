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
    + From<u128>
    + From<u64>
    + From<u32>
    + From<u16>
    + From<u8>
    + From<i128>
    + From<i64>
    + From<i32>
    + From<i16>
    + From<i8>
{
    fn zero() -> Self;
    fn one() -> Self;

    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}

impl Ring for BigInt {
    fn zero() -> Self { BigInt::from(0) }
    fn one() -> Self { BigInt::from(1) }
}
