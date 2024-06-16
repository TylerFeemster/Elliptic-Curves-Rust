use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use rational::Rational;
use point::Point;

mod curve;
mod point;
mod utils;
mod rational;

fn old_main1() {

    let mut curves = Vec::<curve::CurveInfo>::new();

    let file_path = "./data/allcurves";
    let f = File::open(file_path)
    .expect("Should be able to read named file.");

    let mut reader = BufReader::new(f);

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) | Err(_) => break, // read_line returns 0 at EOF
            _ => (),
        }

        let curve = curve::CurveInfo::from_string(line);
        //println!("{:?}", curve);
        curves.push(curve);
    }

    let mut total = 0;
    let mut ntorsion = 0;
    let mut nrank = 0;
    let mut nboth = 0;
    for curve in curves {
        total += 1;
        if curve.torsion > 1 { ntorsion += 1; }
        if curve.rank > 0 { nrank += 1; }
        if curve.rank > 0 && curve.torsion > 1 { nboth += 1; }
    }

    println!("Proportion of curves with nontrivial torsion: {}/{}", ntorsion, total);
    println!("Proportion of curves with nontrivial rank: {}/{}", nrank, total);
    println!("Proportion of curves with nontrivial rank and torsion: {}/{}", nboth, total);

    //println!("{}", utils::gcd(175, 425));

}

fn main() {

    let coeffs = [0, 0, 0, -1, 0];
    let curve = curve::from_coeffs(&coeffs);

    let p1 = Point::new(&curve, Rational::zero(), Rational::zero());
    println!("{}", p1.clone());
    println!("{}", p1.clone() + p1);

    let coeffs = [0, -1, 0, -2, 1];
    let curve = curve::from_coeffs(&coeffs);

    let p0 = Point::new(&curve, Rational::zero(), Rational::from_i64(1));
    let mut p = p0.clone();
    for _ in 1..10 {
        println!("{} ... {}", p, p.verify());
        p = p + p0.clone();
    }

    let mul = 9 * p0.clone();
    println!("{} ... {}", mul, mul.verify());

    let same_mul = p0.clone() + p0.clone().double().double().double();
    println!("{} ... {}", same_mul, same_mul.verify() );

    let big_boy = p0.clone().double().double().double().double();
    println!("{} ... {}", big_boy, big_boy.verify());

}
