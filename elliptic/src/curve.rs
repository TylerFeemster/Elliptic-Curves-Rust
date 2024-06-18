#[derive(Clone, Copy, PartialEq)]
pub struct Curve {
    pub a1: i32,
    pub a2: i32,
    pub a3: i32,
    pub a4: i32,
    pub a6: i32
}

pub fn from_coeffs(coeffs: &[i32; 5]) -> Curve {
    let a1 = coeffs[0];
    let a2 = coeffs[1];
    let a3 = coeffs[2];
    let a4 = coeffs[3];
    let a6 = coeffs[4];
    
    Curve { a1, a2, a3, a4, a6 }
}