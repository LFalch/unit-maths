extern crate unit_maths;

use unit_maths::*;

macro_rules! print_eval {
    ($eval:expr) => (
        println!("{} = {}", stringify!($eval), $eval);
    );
    ($eval:expr, $si:expr) => (
        let x = $eval;
        print!("{} = {}", stringify!($eval), $si.display(&x));
        println!(" ({:#})", x.1.dimension);
    );
    ($eval:expr, $si:expr, $assign:ident) => (
        let $assign = $eval;
        print!("{} := {} = {}", stringify!($assign), stringify!($eval), $si.display(&$assign));
        println!(" ({:#})", $assign.1.dimension);
    );
}

fn main() {
    let si = UnitSystem::<f64>::si();

    // Strong acid
    let vol1 = si.val(30., "mL").unwrap();
    let con1 = si.val(0.1, "M").unwrap();
    // Strong base
    let vol2 = si.val(15., "mL").unwrap();
    let con2 = si.val(0.1, "M").unwrap();

    print_eval!(vol1, si);
    print_eval!(si.as_(vol1, "L"), si);
    print_eval!(si.as_(vol1, "m³"), si);
    print_eval!(con1, si);
    print_eval!(vol2, si);
    print_eval!(si.as_(vol2, "L"), si);
    print_eval!(si.as_(vol2, "m³"), si);
    print_eval!(con2, si);

    print_eval!(vol1*con1, si, amt1);
    print_eval!(vol2*con2, si, amt2);
    print_eval!(amt1-amt2, si, amt_hydronium);
    print_eval!(vol1+vol2, si, vol);
    print_eval!(amt_hydronium/vol, si, con);
    print_eval!(-con.0.log10());
}
