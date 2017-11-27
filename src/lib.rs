extern crate num;

use std::ops::{Add, Mul, Sub};
use std::fmt::{self, Display};

fn num_to_super(c: char) -> char {
    match c {
        '-' => '⁻',
        '0' => '⁰',
        '1' => '¹',
        '2' => '²',
        '3' => '³',
        '4' => '⁴',
        '5' => '⁵',
        '6' => '⁶',
        '7' => '⁷',
        '8' => '⁸',
        '9' => '⁹',
        c => c
    }
}

fn to_superscript(src: &str) -> String {
    src.chars().map(num_to_super).collect()
}

#[test]
fn super_test() {
    assert_eq!(to_superscript("-124"), "⁻¹²⁴");
    assert_eq!(to_superscript("asdg ja-kage4²"), "asdg ja⁻kage⁴²");
}

mod dimensions;
pub use dimensions::*;

mod units;
pub use units::*;

mod display;
pub use display::UnitDisplay;
