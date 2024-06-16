use core::iter::zip;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CurveInfo {
    pub conductor: u64,
    pub isogenyclass: String,
    pub num_in_class: u16,
    pub values: HashMap<String, i64>,
    pub rank: u8,
    pub torsion: u32
}

fn coeffs_to_values(array: &[i64; 5], map: &mut HashMap<String, i64>) {
    // a values
    for (key, value) in zip(["a1", "a2", "a3", "a4", "a6"], *array) {
        map.insert(key.to_string(), value);
    }

    // b values (formulas in terms of a values)
    let bees: [i64; 4] = [
        map["a1"] * map["a1"] + 4 * map["a4"],
        2 * map["a4"] + map["a1"] * map["a3"],
        map["a3"] * map["a3"] + 4 * map["a6"],
        map["a1"] * map["a1"] * map["a6"] + 4 * map["a2"] * map["a6"]
        - map["a1"] * map["a3"] * map["a4"] + map["a2"] * map["a3"] * map["a3"]
        - map["a4"] * map["a4"]
    ];
    for (key, value) in zip(["b2", "b4", "b6", "b8"], bees) {
        map.insert(key.to_string(), value);
    }

    let cees: [i64; 2] = [
        map["b2"] * map["b2"] - 24 * map["b4"],
        - map["b2"] * map["b2"] * map["b2"]
        + 36 * map["b2"] * map["b4"]
        - 216 * map["b6"]
    ];
    for (key, value) in zip(["c4", "c6"], cees) {
        map.insert(key.to_string(), value);
    }

    let disc = - map["b2"] * map["b2"] * map["b8"] 
    - 8 * map["b4"] * map["b4"] * map["b4"]
    - 27 * map["b6"] * map["b6"] + 9 * map["b2"] * map["b4"] * map["b6"];
    map.insert("D".to_string(), disc);

}

impl CurveInfo {
    pub fn from_string(line: String) -> Self {
        let mut iter = line.split_whitespace();

        let conductor = iter.next().expect("Expecting string slice.");
        let conductor: u64 = conductor.parse().expect("Expecting number.");

        let isogenyclass: String = iter.next().expect("Expecting string slice.").to_string();

        let num_in_class = iter.next().expect("Expecting string slice.");
        let num_in_class: u16 = num_in_class.parse().expect("Expecting number.");

        let mut curve_string = iter.next().expect("Expecting string slice.").to_string();
        curve_string.pop(); // remove ']' from end
        curve_string.remove(0); // remove '[' from start
        let string_nums = curve_string.split(",");

        let mut coefficients: [i64; 5] = [0; 5];
        for (i, s) in string_nums.enumerate() {
            coefficients[i] = s.parse().expect("Expecting a number.");
        }

        let rank = iter.next().expect("Expecting string slice.");
        let rank: u8 = rank.parse().expect("Expecting number.");

        let torsion = iter.next().expect("Expecting string slice.");
        let torsion: u32 = torsion.parse().expect("Expecting number.");

        let mut values: HashMap<String, i64> = HashMap::new();
        coeffs_to_values(&coefficients, &mut values);

        CurveInfo {
            conductor,
            isogenyclass,
            num_in_class,
            values,
            rank,
            torsion
        }
    }

}

pub struct Curve {
    pub values: HashMap<String, i64>
}
pub fn from_coeffs(coeffs: &[i64; 5]) -> Curve {
    let mut values: HashMap<String, i64> = HashMap::new();
    coeffs_to_values(coeffs, &mut values);
    Curve { values }
}
