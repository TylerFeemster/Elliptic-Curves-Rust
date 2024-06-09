use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Curve {
    conductor: u64,
    isogenyclass: String,
    num_in_class: u16,
    curve: [i64; 5],
    rank: u8,
    torsion: u32,
}

impl Curve {
    fn from_string(line: String) -> Self {
        let mut iter = line.split_whitespace();

        let conductor = iter.next().expect("Expecting string slice.");
        let conductor: u64 = conductor.parse().expect("Expecting number.");

        let isogenyclass: String = iter.next().expect("Expecting string slice.").to_string();

        let num_in_class = iter.next().expect("Expecting string slice.");
        let num_in_class: u16 = num_in_class.parse().expect("Expecting number.");

        let mut curve_string = iter.next().expect("Expecting Weirdness.").to_string();
        curve_string.pop(); // remove ']' from end
        curve_string.remove(0); // remove '[' from start
        let string_nums = curve_string.split(",");

        let mut curve: [i64; 5] = [0; 5];
        for (i, s) in string_nums.enumerate() {
            curve[i] = s.parse().expect("Expecting a number.");
        }

        let rank = iter.next().expect("Expecting string slice.");
        let rank: u8 = rank.parse().expect("Expecting number.");

        let torsion = iter.next().expect("Expecting string slice.");
        let torsion: u32 = torsion.parse().expect("Expecting number.");

        println!("{}", conductor);
        println!("{}", isogenyclass);
        println!("{}", num_in_class);
        println!("{:?}", curve);
        println!("{}", rank);
        println!("{}", torsion);

        Curve {
            conductor,
            isogenyclass,
            num_in_class,
            curve,
            rank,
            torsion,
        }
    }
}

fn main() {

    let file_path = "./data/allcurves";
    let f = File::open(file_path)
    .expect("Should be able to read named file.");

    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let len = reader.read_line(&mut line).expect("Should be able to read lines.");
    println!("First line is {len} bytes long");
    println!("The line is: {line}");

    Curve::from_string(line);

}
