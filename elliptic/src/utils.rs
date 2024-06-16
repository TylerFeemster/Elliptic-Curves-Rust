
pub fn gcd(mut item1: u128, mut item2: u128) -> u128 {
    while item2 != 0 {
        let temp = item2;
        item2 = item1 % item2;
        item1 = temp;
    }
    item1
}