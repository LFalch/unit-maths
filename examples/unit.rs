extern crate unit_maths;

use std::io::stdin;

use unit_maths::*;

fn main() {
    let si = UnitSystem::<f64>::si();

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s = s.trim().to_owned();
    let unit = unit_from_str(&si, &s).unwrap();
    let val = Value(1., unit);

    println!("Unit: {}\nFactor: {}", si.display(&val), unit.factor);
    println!("Dimension: {:#}", unit.dimension);
}
