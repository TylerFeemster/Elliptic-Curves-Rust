use::num_bigint::BigUint;

pub fn gcd(mut item1: BigUint, mut item2: BigUint) -> BigUint {
    while item2 != BigUint::ZERO {
        let temp = item2.clone();
        item2 = item1 % item2;
        item1 = temp;
    }
    item1
}