extern crate unit_maths;

use std::io::stdin;

use unit_maths::*;

fn main() {
    let si = UnitSystem::<f64>::si();

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s = s.trim().to_owned();
    let val = si.val_s(&s).unwrap();

    println!("Value: {}", si.display(&val));
    println!("Dimension: {:#} (scale: {})", val.1.dimension, val.1.factor);
}
