#![warn(missing_docs)]
//! Crate for doing maths with units

extern crate num;

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

fn super_to_num(c: char) -> char {
    match c {
        '⁻' => '-',
        '⁰' => '0',
        '¹' => '1',
        '²' => '2',
        '³' => '3',
        '⁴' => '4',
        '⁵' => '5',
        '⁶' => '6',
        '⁷' => '7',
        '⁸' => '8',
        '⁹' => '9',
        c => c
    }
}

#[inline]
fn to_superscript(src: &str) -> String {
    src.chars().map(num_to_super).collect()
}
#[inline]
fn superscript_to_number(src: &str) -> String {
    src.chars().map(super_to_num).collect()
}

#[test]
fn super_test() {
    assert_eq!(to_superscript("-124"), "⁻¹²⁴");
    assert_eq!(to_superscript("asdg ja-kage4²"), "asdg ja⁻kage⁴²");
    assert_eq!(superscript_to_number("⁻¹²⁴"), "-124");
    assert_eq!(superscript_to_number("asdg ja⁻kage4²"), "asdg ja-kage42");
}

mod dimensions;
pub use dimensions::*;

mod units;
pub use units::*;

mod display;
pub use display::UnitDisplay;

mod read;
use read::*;
