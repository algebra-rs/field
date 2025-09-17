use num_bigint::BigInt;
use field::ring::Ring;

#[test]
fn test_zero_and_one() {
    assert_eq!(<BigInt as Ring>::zero(), BigInt::from(0));
    assert_eq!(<BigInt as Ring>::one(), BigInt::from(1));
}

#[test]
fn test_addition() {
    let a = BigInt::from(2);
    let b = BigInt::from(3);
    assert_eq!(a + b, BigInt::from(5));
}