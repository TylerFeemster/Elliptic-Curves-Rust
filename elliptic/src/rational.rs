use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt::{Display, Formatter, Result};
use crate::utils;
use num_bigint::BigUint;
use num_traits::Zero;

#[derive(Clone, Copy, PartialEq)]
pub enum Sign {
    Pos, // also 0
    Neg
}

impl Sign {
    pub fn flip(&self) -> Sign {
        match self {
            Sign::Pos => Sign::Neg,
            Sign::Neg => Sign::Pos
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Sign::Pos => "".to_string(),
            Sign::Neg => "-".to_string()
        }
    }
}

impl Mul for Sign {
    type Output = Sign;
    
    fn mul(self, other: Sign) -> Sign {
        if self == other { return Sign::Pos }
        Sign::Neg  
    }
}

#[derive(Clone, PartialEq)]
pub struct Rational {
    sign: Sign,
    p: BigUint,
    q: BigUint
}

impl Display for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let sgn = self.sign.to_string();
        if self.q == BigUint::from(1 as u8) {
            write!(f, "{}{}", sgn, self.p)
        }
        else {
            write!(f, "{}{}/{}", sgn, self.p, self.q)
        }
    }
}

// Main Stuff

impl Rational {

    pub fn new(sign: Sign, numerator: BigUint, denominator: BigUint) -> Rational {
        if numerator.is_zero() { return Rational::zero() }
        let gcd = utils::gcd(numerator.clone(), denominator.clone());
        let p = numerator / &gcd;
        let q = denominator / gcd;
        Rational { sign, p , q }
    }

    pub fn zero() -> Rational {
        Rational { sign: Sign::Pos, 
            p: BigUint::from(0 as u8), 
            q: BigUint::from(1 as u8) }
    }

    pub fn is_zero(&self) -> bool {
        return self.p.is_zero()
    }

    pub fn from(mut num: i32) -> Rational {
        let sign: Sign;
        if num >= 0 {
            sign = Sign::Pos;
        }
        else {
            num = -num;
            sign = Sign::Neg;
        }

        Rational { sign, p : BigUint::from(num as u64), q : BigUint::from(1 as u8) }
    }

}

// Reference Versions

impl<'a, 'b> Add<&'b Rational> for &'a Rational {
    type Output = Rational;
    
    fn add(self, other: &'b Rational) -> Rational {

        let denominator = &self.q * &other.q;
        
        let num1 = &self.p * &other.q;
        let num2 = &self.q * &other.p;

        let numerator: BigUint;
        let sign: Sign;
        if self.sign == other.sign {
            numerator = num1 + num2;
            sign = self.sign;
        }
        else if num1 >= num2 {
            numerator = num1 - num2;
            sign = self.sign;
        }
        else {
            numerator = num2 - num1;
            sign = other.sign;
        }
        Rational::new(sign, numerator, denominator)
    }
}

impl<'a> Add<i32> for &'a Rational {
    type Output = Rational;

    fn add(self, other: i32) -> Rational {
        self + &Rational::from(other)
    }
}

impl<'a> Add<&'a Rational> for i32 {
    type Output = Rational;

    fn add(self, other: &'a Rational) -> Rational {
        other + self
    }
}

impl<'a> Neg for &'a Rational {
    type Output = Rational;

    fn neg(self) -> Rational {
        if self.p.is_zero() { return Rational::zero() }
        let p = self.p.clone();
        let q = self.q.clone();
        Rational { sign: self.sign.flip(), p, q }
    }
}

impl<'a, 'b> Sub<&'b Rational> for &'a Rational {
    type Output = Rational;

    fn sub(self, other: &'b Rational) -> Rational {
        self + &(-other)
    }
}

impl<'a> Sub<i32> for &'a Rational {
    type Output = Rational;

    fn sub(self, other: i32) -> Rational {
        self - &Rational::from(other)
    }
}

impl<'a> Sub<&'a Rational> for i32 {
    type Output = Rational;

    fn sub(self, other: &'a Rational) -> Rational {
        Rational::from(self) + (-other)
    }
}

impl<'a, 'b> Mul<&'b Rational> for &'a Rational {
    type Output = Rational;

    fn mul(self, other: &'b Rational) -> Rational {
        if self.is_zero() || other.is_zero() {
            return Rational::zero()
        }
        let sign = self.sign * other.sign;
        let numerator = &self.p * &other.p;
        let denominator = &self.q * &other.q;
        Rational::new(sign, numerator, denominator)
    }
}

impl<'a, 'b> Div<&'b Rational> for &'b Rational {
    type Output = Rational;

    fn div(self, other: &'b Rational) -> Rational {
        assert!(!other.is_zero(), "Divide by zero.");
        let sign = self.sign * other.sign;
        let numerator = &self.p * &other.q;
        let denominator = &self.q * &other.p;
        Rational::new(sign, numerator, denominator)
    }
}

impl<'a> Mul<i32> for &'a Rational {
    type Output = Rational;

    fn mul(self, number: i32) -> Rational {
        self * &Rational::from(number)
    }
}

impl<'a> Mul<&'a Rational> for i32 {
    type Output = Rational;

    fn mul(self, rational: &'a Rational) -> Rational {
        rational * self
    }
}

// Converted to Referenced

impl Add for Rational {
    type Output = Rational;
    
    fn add(self, other: Rational) -> Rational {
        &self + &other
    }
}

impl Add<i32> for Rational {
    type Output = Rational;

    fn add(self, other: i32) -> Rational {
        &self + other
    }
}

impl Add<Rational> for i32 {
    type Output = Rational;

    fn add(self, other: Rational) -> Rational {
        other + self
    }
}

impl Sub<i32> for Rational {
    type Output = Rational;

    fn sub(self, other: i32) -> Rational {
        &self - other
    }
}

impl Sub<Rational> for i32 {
    type Output = Rational;

    fn sub(self, other: Rational) -> Rational {
        self - &other
    }
}

impl Sub for Rational {
    type Output = Rational;

    fn sub(self, other: Rational) -> Rational {
        self + (-other)
    }
}

impl Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Rational {
        - &self
    }
}

impl Mul for Rational {
    type Output = Rational;

    fn mul(self, other: Rational) -> Rational {
        &self * &other
    }
}

impl Div for Rational {
    type Output = Rational;

    fn div(self, other: Rational) -> Rational {
        &self / &other
    }
}

impl Mul<i32> for Rational {
    type Output = Rational;

    fn mul(self, number: i32) -> Rational {
        &self * number
    }
}

impl Mul<Rational> for i32 {
    type Output = Rational;

    fn mul(self, rational: Rational) -> Rational {
        &rational * self
    }
}
