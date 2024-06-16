use std::ops::{Add, AddAssign, Sub, Mul, Div, Neg};
use std::cmp::{PartialEq, Eq};
use std::fmt::{Display, Formatter, Result};
use crate::utils;

#[derive(Clone, Copy)]
pub struct Rational {
    p: i128,
    q: u128
}

impl Display for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.q == 1 {
            write!(f, "{}", self.p)
        }
        else {
            write!(f, "{}/{}", self.p, self.q)
        }
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Rational) -> bool {
        self.p == other.p && self.q == self.q
    }
}

impl Eq for Rational {}

impl Add for Rational {
    type Output = Rational;
    
    fn add(self, other: Rational) -> Rational {
        // (p1/q1 + p2/q2) = (1/q1q2)(p1*qq2 + p2*q1)
        let numerator = self.p * (other.q as i128)
        + other.p * (self.q as i128);
        let denominator = self.q * other.q;
        Rational::new(numerator, denominator)
    }
}

impl Add<i64> for Rational {
    type Output = Rational;

    fn add(self, other: i64) -> Rational {
        self + Rational::from_i64(other)
    }
}

impl Add<Rational> for i64 {
    type Output = Rational;

    fn add(self, other: Rational) -> Rational {
        other + self
    }
}

impl Sub<i64> for Rational {
    type Output = Rational;

    fn sub(self, other: i64) -> Rational {
        self - Rational::from_i64(other)
    }
}

impl Sub<Rational> for i64 {
    type Output = Rational;

    fn sub(self, other: Rational) -> Rational {
        Rational::from_i64(self) - other
    }
}

impl AddAssign for Rational {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other
    }
}

impl Sub for Rational {
    type Output = Rational;

    fn sub(self, other: Rational) -> Rational {
        // (p1/q1 - p2/q2) = (1/q1q2)(p1*q2 - p2*q1)
        let numerator = self.p * (other.q as i128)
        - other.p * (self.q as i128);
        let denominator = self.q * other.q;
        Rational::new(numerator, denominator)
    }
}

impl Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Rational {
        Rational { p: -self.p, q: self.q }
    }
}

impl Mul for Rational {
    type Output = Rational;

    fn mul(self, other: Rational) -> Rational {
        let numerator = self.p * other.p;
        let denominator = self.q * other.q;
        Rational::new(numerator, denominator)
    }
}

impl Div for Rational {
    type Output = Rational;

    fn div(self, other: Rational) -> Rational {

        assert!(other.p != 0, "Divide by zero.");
        
        let numerator: i128;
        let denominator: u128;
        if other.p > 0 {
            numerator = self.p * (other.q as i128);
            denominator = (other.p as u128) * self.q;
        }
        else {
            numerator = - self.p * (other.q as i128);
            denominator = (-other.p as u128) * self.q;
        }
        Rational::new(numerator, denominator)
    }
}

impl Mul<i64> for Rational {
    type Output = Rational;

    fn mul(self, number: i64) -> Rational {
        Rational::new(self.p * (number as i128), self.q)
    }
}

impl Mul<Rational> for i64 {
    type Output = Rational;

    fn mul(self, rational: Rational) -> Rational {
        rational * self
    }
}

impl Rational {

    pub fn new(numerator: i128, denominator: u128) -> Rational {
        let unum = if numerator < 0 {- numerator as u128} else {numerator as u128};
        let gcd = utils::gcd(unum, denominator);
        let p = numerator / (gcd as i128);
        let q = denominator / gcd;
        Rational { p , q }
    }

    pub fn zero() -> Rational {
        Rational { p : 0, q : 1 }
    }

    pub fn from_i64(num: i64) -> Rational {
        Rational { p : num as i128, q : 1 }
    }

}