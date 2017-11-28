use super::*;

use num::Float;

#[derive(Debug)]
enum BuildState {
    Unit,
    Exponent
}

pub fn unit_from_str<N: Float>(sys: &UnitSystem<N>, s: &str) -> Option<Unit<N>> {
    let mut proto_unit = String::with_capacity(4);
    let mut proto_exponent = String::with_capacity(4);
    let mut cur_state = BuildState::Unit;
    let mut inverse = false;

    let mut unit = Unit::new(NUL);

    for c in s.replace('/', " /").chars().filter(|&c| c != '^').map(super_to_num).chain(Some(' ')) {
        let c = match c {
            '*' | '·' => ' ',
            c => c,
        };
        if proto_unit.is_empty() && c == ' ' {
            continue
        }
        match cur_state {
            _ if c == '/' => inverse = true,
            BuildState::Unit if c == ' ' => {
                let un = sys.get_unit(&proto_unit)? * if inverse{-1}else{1};
                proto_unit.clear();
                unit = unit + un;
                inverse = false;
            }
            BuildState::Unit => {
                if c.is_alphabetic() {
                    proto_unit.push(c);
                } else if c == '-' || c.is_numeric() {
                    cur_state = BuildState::Exponent;
                    proto_exponent.push(c);
                } else {
                    return None;
                }
            }
            BuildState::Exponent => {
                if c == '-' || c.is_numeric() {
                    proto_exponent.push(c);
                } else if c.is_alphabetic() || c == ' ' {
                    let ex = proto_exponent.parse().ok()?;
                    let un = sys.get_unit(&proto_unit)? * ex * if inverse{-1}else{1};
                    unit = unit + un;
                    inverse = false;
                    proto_exponent.clear();
                    proto_unit.clear();

                    if c != ' ' {
                        proto_unit.push(c);
                    }
                    cur_state = BuildState::Unit;
                } else {
                    return None;
                }
            }
        }
    }
    Some(unit)
}

use std::str::FromStr;

pub fn value_from_str<N: Float + FromStr>(sys: &UnitSystem<N>, s: &str) -> Option<Value<N>> {
    let index = s.find(<char>::is_whitespace).unwrap_or(s.len());
    let (val, unit) = s.split_at(index);

    Some(Value(val.parse().ok()?, unit_from_str(sys, unit)?))
}
