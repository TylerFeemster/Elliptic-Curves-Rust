use std::ops::{Add, AddAssign, Sub, Mul, Div, Neg};
use std::cmp::{PartialEq, Eq};
use std::fmt::{Display, Formatter, Result};
use crate::utils;

use::num_bigint::{BigInt, BigUint, Sign};

#[derive(Clone)]
pub struct Rational {
    p: BigInt,
    q: BigUint
}

impl Display for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.q == BigUint::from(1 as u8) {
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
        let numerator = self.p * BigInt::from_biguint(Sign::NoSign, other.q.clone()) 
        + other.p * BigInt::from_biguint(Sign::NoSign, self.q.clone());
        let denominator = &self.q * &other.q;
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
        let numerator = self.p * BigInt::from_biguint(Sign::NoSign, other.q.clone())
        - other.p * BigInt::from_biguint(Sign::NoSign, self.q.clone());
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
        let (sign, p) = other.p.into_parts();
        let q = BigInt::from_biguint(sign, other.q);

        self * Rational::new(q, p) // flipped
    }
}

impl Mul<i64> for Rational {
    type Output = Rational;

    fn mul(self, number: i64) -> Rational {
        let numerator = self.p * number;
        let denominator = self.q;
        Rational::new(numerator, denominator)
    }
}

impl Mul<Rational> for i64 {
    type Output = Rational;

    fn mul(self, rational: Rational) -> Rational {
        rational * self
    }
}

impl Rational {

    pub fn new(numerator: BigInt, denominator: BigUint) -> Rational {
        let mut p = numerator.clone();
        let mut q = denominator.clone();
        let gcd = utils::gcd(numerator.into_parts().1, denominator);
        p /= BigInt::from_biguint(Sign::NoSign, gcd.clone());
        q /= gcd;
        Rational { p, q }
    }

    pub fn zero() -> Rational {
        Rational { p : BigInt::from(0), q : BigUint::from(1 as u8) }
    }

    pub fn from_i64(num: i64) -> Rational {
        Rational { p : BigInt::from(num), q : BigUint::from(1 as u8) }
    }

}