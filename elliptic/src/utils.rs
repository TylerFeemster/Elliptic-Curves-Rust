use num_bigint::BigUint;
use num_traits::Zero;


pub fn gcd(mut item1: BigUint, mut item2: BigUint) -> BigUint {
    while !item2.is_zero() {
        let temp = item2.clone();
        item2 = item1 % item2;
        item1 = temp;
    }
    item1
}