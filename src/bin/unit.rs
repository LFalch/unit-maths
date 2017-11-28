extern crate unit_maths;

use std::io::stdin;

use unit_maths::*;

fn main() {
    let si = UnitSystem::<f64>::si();

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s = s.trim().to_owned();
    let unit = unit_from_str(&si, &s);
    let val = Value(unit.factor, unit);

    println!("Unit {}", si.display(&val));
}
