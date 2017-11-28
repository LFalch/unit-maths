extern crate unit_maths;

use unit_maths::*;

macro_rules! print_eval {
    ($eval:expr) => (
        println!("{} = {}", stringify!($eval), $eval);
    );
    ($eval:expr, $si:expr) => (
        print!("{} = {}", stringify!($eval), $si.display(&$eval));
        println!(" ({:#})", $eval.1.dimension);
    );
    ($eval:expr, $si:expr, $assign:ident) => (
        let $assign = $eval;
        print!("{} := {} = {}", stringify!($assign), stringify!($eval), $si.display(&$assign));
        println!(" ({:#})", $eval.1.dimension);
    );
}

fn main() {
    let si = UnitSystem::<f64>::si();

    // Strong acid
    let vol1 = si.val(30., "mL");
    let con1 = si.val(0.1, "M");
    // Strong base
    let vol2 = si.val(15., "mL");
    let con2 = si.val(0.1, "M");

    let m3 = si["m"]*3;

    print_eval!(vol1, si);
    print_eval!(si.as_(vol1, "L"), si);
    print_eval!(si.cast(vol1, &m3), si);
    print_eval!(con1, si);
    print_eval!(vol2, si);
    print_eval!(si.as_(vol2, "L"), si);
    print_eval!(si.cast(vol2, &m3), si);
    print_eval!(con2, si);

    print_eval!(vol1*con1, si, amt1);
    print_eval!(vol2*con2, si, amt2);
    print_eval!(amt1-amt2, si, amt_hydronium);
    print_eval!(vol1+vol2, si, vol);
    print_eval!(amt_hydronium/vol, si, con);
    print_eval!(-con.0.log10());
}
