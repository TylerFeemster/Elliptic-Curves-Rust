mod curve;
mod point;
mod utils;
mod rational;

use rational::Rational;
use point::Point;

fn main() {

    let mut rational = Rational::from(-65)/Rational::from(6);
    for _ in 1..5 {
        println!("{}", rational);
        rational = &rational * &rational + 3 * &rational;
    }

    //let coeffs = [0, 0, 0, -1, 0];
    //let curve = curve::from_coeffs(&coeffs);

    //let p1 = Point::new(&curve, Rational::zero(), Rational::zero());
    //println!("{}", p1.clone());
    //println!("{}", p1.clone() + p1);

    let coeffs = [0, 0, 0, 73, 0];
    let curve = curve::from_coeffs(&coeffs);

    let p0 = Point::new(curve, Rational::from(36), Rational::from(222));
    let mut p = p0.clone();
    for _ in 1..14 {
        println!("{} ... {}", p, p.verify());
        p = p + p0.clone();
    }

    let mul = 9 * p0.clone();
    println!("{} ... {}", mul, mul.verify());

    let same_mul = p0.clone() + p0.double().double().double();
    println!("{} ... {}", same_mul, same_mul.verify());

}
