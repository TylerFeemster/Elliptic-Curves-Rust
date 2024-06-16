use std::collections::HashMap;
use std::ops::{Add, Neg, Sub, Mul};
use std::cmp::{PartialEq, Eq};
use std::fmt::{Display, Formatter, Result};

use crate::curve::Curve;
use crate::rational::Rational;

#[derive(Clone)]
pub enum PointKind {
    Origin,
    FinitePoint(Rational, Rational)
}

#[derive(Clone)]
pub struct Point<'a> {
    curve: &'a Curve,
    point: PointKind
}

impl<'a> Display for Point<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.point {
            PointKind::Origin => write!(f, "Origin"),
            PointKind::FinitePoint(x, y) => { 
                write!(f, "({}, {})", x, y) 
            }
        }
    }
}

impl PartialEq for Point<'_> {
    fn eq(&self, other: &Point) -> bool {
        if !self.same_curve(other) { return false; }
        match &self.point {
            PointKind::Origin => {
                match other.point {
                   PointKind::Origin => { return true; },
                   _ => { return false; } 
                }
            },
            PointKind::FinitePoint(x1, y1) => {
                match &other.point {
                    PointKind::FinitePoint(x2, y2) => {
                        return x1 == x2 && y1 == y2;
                    },
                    _ => { return false; }
                }
            }
        }
    }
}

impl Eq for Point<'_> {}

fn big_lambda(x: Rational, y: Rational, vals: &HashMap<String, i64>) -> Rational {
    let numerator = 3 * x.clone() * x.clone() + 2 * vals["a2"] * x.clone()
    + vals["a4"] - vals["a1"] * y.clone();
    let denominator = 2 * y + vals["a1"] * x + vals["a3"];
    numerator / denominator   
}

fn big_lambda_nu(x: Rational, y: Rational, vals: &HashMap<String, i64>) -> (Rational, Rational) {
    // lambda
    let numerator = 3 * x.clone() * x.clone() + 2 * vals["a2"] * x.clone()
    + vals["a4"] - vals["a1"] * y.clone();

    let denominator = 2 * y.clone() + vals["a1"] * x.clone() + vals["a3"];

    let lambda = numerator / denominator.clone();

    // nu 
    // -- numerator terms
    let numerator = - x.clone() * x.clone() * x.clone() + vals["a4"] * x
    + 2 * vals["a6"] - vals["a3"] * y;

    let nu = numerator / denominator; // has same denominator

    (lambda, nu)
}

fn small_lambda_nu(x1: Rational, y1: Rational, x2: Rational, y2: Rational) -> (Rational, Rational) {
    // lambda
    let numerator = y2.clone() - y1.clone();
    let denominator = x2.clone() - x1.clone();
    let lambda = numerator/denominator.clone();

    // nu
    let numerator = y1 * x2 - x1 * y2;
    let nu = numerator/denominator;

    (lambda, nu)
}

impl<'a> Neg for Point<'a> {
    type Output = Point<'a>;

    fn neg(self) -> Point<'a> {
        let curve = self.curve;
        let x: Rational;
        let y: Rational;
        match self.point {
            PointKind::Origin => {
                return Point::origin(curve)
            },
            PointKind::FinitePoint(x0, y0) => { x = x0; y = y0; }
        }

        let vals = &curve.values;
        let y = -y - vals["a1"] * x.clone() - vals["a3"];

        Point::new(curve, x, y)
    }
}

impl<'a> Add for Point<'a> {
    type Output = Point<'a>;

    fn add(self, other: Point<'a>) -> Point<'a> {
        // assert points are on same curve
        assert!(self.same_curve(&other));

        let curve = self.curve;
        let vals = &curve.values;

        // if one is origin, return other
        let x1: &Rational;
        let y1: &Rational;
        match &self.point {
            PointKind::Origin => return other,
            PointKind::FinitePoint(x, y) => {x1 = x; y1 = y;}
        }
        let x2: &Rational;
        let y2: &Rational;
        match &other.point {
            PointKind::Origin => return self,
            PointKind::FinitePoint(x, y) => {x2 = x; y2 = y;}
        }

        // if inverse points, return origin
        if self == -other.clone() {
            return Point::origin(curve)
        }

        // lambda, nu
        let (lambda, nu) = if x1 == x2 {
            big_lambda_nu(x1.clone(), y1.clone(), &vals)
        } else {
            small_lambda_nu(x1.clone(), y1.clone(), x2.clone(), y2.clone())
        };

        // output
        let x = lambda.clone() * lambda.clone() + vals["a1"] * lambda.clone()
        -vals["a2"] - x1.clone() - x2.clone();
        
        let y = -(lambda + vals["a1"]) * x.clone()
        - nu - vals["a3"];

        Point::new(curve, x, y)
    }
}

impl<'a> Sub for Point<'a> {
    type Output = Point<'a>;

    fn sub(self, other: Point<'a>) -> Point<'a> {
        self + (-other)
    }
}

impl<'a> Point<'a> {
    pub fn same_curve(&self, other: &Point) -> bool {
        let vals1 = &self.curve.values;
        let vals2 = &other.curve.values;
        for key in ["a1", "a2", "a3", "a4", "a6"] {
            if vals1[key] != vals2[key] {
                return false
            }
        }
        return true
    }

    pub fn verify(&self) -> bool {
        let vals = &self.curve.values;
        let x: &Rational;
        let y: &Rational;
        match &self.point {
            PointKind::Origin => { return true; },
            PointKind::FinitePoint(x0, y0) => {x = x0; y = y0;}
        }

        let lhs = y.clone() * y.clone() + vals["a1"] * x.clone() * y.clone() + vals["a3"] * y.clone();
        let rhs = x.clone() * x.clone() * x.clone() + vals["a2"] * x.clone() * x.clone() 
        + vals["a4"] * x.clone() + vals["a6"];

        lhs == rhs
    }

    pub fn origin(curve: &'a Curve) -> Point<'a> {
        return Point { curve, point: PointKind::Origin }
    }

    pub fn new(curve: &'a Curve, x: Rational, y: Rational) -> Point<'a> {
        return Point { curve, point : PointKind::FinitePoint(x, y) }
    }

    pub fn double(self) -> Point<'a> {
        let curve = self.curve;
        let x1: &Rational;
        let y1: &Rational;
        match &self.point {
            PointKind::Origin => {
                return Point { curve, point: PointKind::Origin }
            },
            PointKind::FinitePoint(x, y) => { x1 = x; y1 = y; }
        }

        // return origin if self inverse
        if self == -self.clone() {
            return Point { curve, point : PointKind::Origin }
        }

        let vals = &curve.values;

        // lambda
        let lambda = big_lambda(x1.clone(), y1.clone(), &vals);

        // new values
        let x = lambda.clone() * lambda.clone() + lambda.clone() * vals["a1"]
        - vals["a2"] - 2 * x1.clone();
        let y = -vals["a1"] * x.clone() - vals["a3"]
        + lambda * (x1.clone() - x.clone()) - y1.clone();

        Point::new(curve, x, y)
    }
}

impl<'a> Mul<i32> for Point<'a> {
    type Output = Point<'a>;

    fn mul(self, mut number: i32) -> Point<'a> {
        let curve = self.curve;
        match &self.point {
            PointKind::Origin => {
                return Point::origin(curve);
            },
            _ => ()
        }

        if number == 0 { return Point::origin(curve); }

        let mut point: Point<'a> = self;
        if number < 0 {
            number = -number;
            point = -point;
        }

        let mut bool_vector: Vec<bool> = Vec::new();
        while number > 0 {
            bool_vector.push(number % 2 == 1);
            number /= 2;
        }

        let mut powers: Vec<Point> = Vec::new();
        let mut doubler = point;
        for boolean in bool_vector {
            if boolean {
                powers.push(doubler.clone())
            }
            doubler = doubler.double();
        }

        let mut sum = Point::origin(curve);
        for summand in powers {
            sum = sum + summand;
        }
        sum
    }

}

impl<'a> Mul<Point<'a>> for i32 {
    type Output = Point<'a>;

    fn mul(self, point: Point<'a>) -> Point<'a> {
        point * self
    }
}
