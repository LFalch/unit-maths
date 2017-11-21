extern crate unit_maths;

use unit_maths::*;

macro_rules! print_eval {
    ($eval:expr) => (println!("{} = {:#.2}", stringify!($eval), $eval));
}

fn main() {
    let distance = Value(5.0, LENGTH);
    let time = Value(3.0, TIME);
    let speed = Value(4., VELOCITY);

    print_eval!(distance/time);
    print_eval!(speed*time);
    print_eval!(distance*time*speed);
}
