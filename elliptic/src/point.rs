use std::ops::{Add, Neg, Sub, Mul};
use std::fmt::{Display, Formatter, Result};

use crate::curve::Curve;
use crate::rational::Rational;

#[derive(Clone, PartialEq)]
pub enum PointKind {
    Origin,
    Finite(Rational, Rational)
}

#[derive(Clone, PartialEq)]
pub struct Point {
    curve: Curve,
    point: PointKind
}

impl Point {
    pub fn same_curve(&self, other: &Point) -> bool {
        self.curve == other.curve
    }

    pub fn is_inverse(&self, other: &Point) -> bool {
        *self == -other
    }

    pub fn verify(&self) -> bool {
        let c = self.curve;
        let x: &Rational;
        let y: &Rational;
        match &self.point {
            PointKind::Origin => { return true; },
            PointKind::Finite(x0, y0) => {x = x0; y = y0;}
        }
        let x2 = x * x;

        let lhs = y * y + c.a1 * (x * y) + c.a3 * y;
        let rhs = x * &x2 + c.a2 * &x2 + c.a4 * x + c.a6;

        lhs == rhs
    }

    pub fn origin(curve: Curve) -> Point {
        return Point { curve, point: PointKind::Origin }
    }

    pub fn new(curve: Curve, x: Rational, y: Rational) -> Point {
        return Point { curve, point : PointKind::Finite(x, y) }
    }

    pub fn double(&self) -> Point {
        let c = self.curve;
        let x1: &Rational;
        let y1: &Rational;
        match &self.point {
            PointKind::Origin => {
                return Point { curve: c, point: PointKind::Origin }
            },
            PointKind::Finite(x, y) => { x1 = &x; y1 = &y; }
        }

        // return origin if self inverse
        if self.is_inverse(&self) {
            return Point { curve: c, point : PointKind::Origin }
        }

        // lambda
        let lambda = big_lambda(x1, y1, &c);
        let m = &lambda;

        // new values
        let x = m * m + m * c.a1 - c.a2 - 2 * x1;
        let y = - c.a1 * &x + lambda * (x1 - &x) - (y1 + c.a3);

        Point::new(c, x, y)
    }
}

// DISPLAY

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.point {
            PointKind::Origin => write!(f, "Origin"),
            PointKind::Finite(x, y) => { 
                write!(f, "({}, {})", x, y) 
            }
        }
    }
}

fn big_lambda(x: &Rational, y: &Rational, c: &Curve) -> Rational {
    let numerator = 3 * (x * x) + 2 * c.a2 * x + c.a4 - c.a1 * y;
    let denominator = 2 * y + c.a1 * x + c.a3;
    numerator / denominator
}

fn big_lambda_nu(x: &Rational, y: &Rational, c: &Curve) -> (Rational, Rational) {
    // lambda
    let numerator = 3 * (x * x) + 2 * c.a2 * x
    + c.a4 - c.a1 * y;

    let denominator = 2 * y + c.a1 * x + c.a3;

    let lambda = &numerator / &denominator;

    // nu 
    // -- numerator terms
    let numerator = - x * (x * x) + c.a4 * x
    + 2 * c.a6 - c.a3 * y;

    let nu = numerator / denominator; // has same denominator

    (lambda, nu)
}

fn small_lambda_nu(x1: &Rational, y1: &Rational, x2: &Rational, y2: &Rational) -> (Rational, Rational) {
    // lambda
    let numerator = y2 - y1;
    let denominator = x2 - x1;
    let lambda = &numerator/&denominator;

    // nu
    let numerator = y1 * x2 - x1 * y2;
    let nu = numerator/denominator;

    (lambda, nu)
}

// NEG

impl Neg for &Point {
    type Output = Point;

    fn neg(self) -> Point {
        let c = self.curve;
        let x: &Rational;
        let y: &Rational;
        match &self.point {
            PointKind::Origin => {
                return Point::origin(c);
            },
            PointKind::Finite(x0, y0) => { x = x0; y = y0; }
        }

        let y = - y - (c.a1 * x) - c.a3;
        Point::new(c , x.clone(), y)
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        -&self
    }
}

// ADD

impl Add for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        // assert points are on same curve
        assert!(self.same_curve(&other));

        let c = self.curve;

        // if one is origin, return other
        let x1: &Rational;
        let y1: &Rational;
        match &self.point {
            PointKind::Origin => return other.clone(),
            PointKind::Finite(x, y) => {x1 = x; y1 = y;}
        }
        let x2: &Rational;
        let y2: &Rational;
        match &other.point {
            PointKind::Origin => return self.clone(),
            PointKind::Finite(x, y) => {x2 = x; y2 = y;}
        }

        // if inverse points, return origin
        if self.is_inverse(other) {
            return Point::origin(c)
        }

        // lambda, nu
        let (lambda, nu) = if x1 == x2 {
            big_lambda_nu(x1, y1, &c)
        } else {
            small_lambda_nu(x1, y1, x2, y2)
        };
        let m = &lambda;

        // output
        let x = m * m + c.a1 * m - c.a2 - (x1 + x2);
        let y = -&x * (m + c.a1) - (nu + c.a3);
        Point::new(c, x, y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        &self + &other
    }
}

// SUB

impl Sub for &Point {
    type Output = Point;

    fn sub(self, other: &Point) -> Point {
        self + &(-other)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        &self - &other
    }
}

// MUL

impl Mul<i32> for &Point {
    type Output = Point;

    fn mul(self, mut number: i32) -> Point {
        let c = self.curve;
        match &self.point {
            PointKind::Origin => {
                return Point::origin(c);
            },
            _ => ()
        }

        if number == 0 { return Point::origin(c); }

        let point: Point;
        if number < 0 {
            number = -number;
            point = -self;
        }
        else {
            point = self.clone();
        }

        let mut bool_vector: Vec<bool> = Vec::new();
        while number > 0 {
            bool_vector.push(number % 2 == 1);
            number /= 2;
        }
        bool_vector.reverse();

        let mut grower = point.clone();
        for boolean in &bool_vector[1..] {
            grower = grower.double();
            if *boolean {
                grower = &grower + &point;
            }
        }
        grower
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, number: i32) -> Point {
        &self * number
    }
}

impl Mul<&Point> for i32 {
    type Output = Point;

    fn mul(self, point: &Point) -> Point {
        point * self
    }
}

impl Mul<Point> for i32 {
    type Output = Point;

    fn mul(self, point: Point) -> Point {
        &point * self
    }
}
