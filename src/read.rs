use super::*;

use std::mem::replace;

use num::Float;

#[derive(Debug)]
enum BuildState {
    Unit,
    Exponent
}

pub fn unit_from_str<N: Float>(sys: &UnitSystem<N>, s: &str) -> Option<Unit<N>> {
    let mut proto_unit = String::new();
    let mut proto_exponent = String::new();
    let mut cur_state = BuildState::Unit;

    let mut units = Vec::<(String, i16)>::new();

    for c in s.chars().filter(|&c| c != '^').map(super_to_num).chain(Some(' ')) {
        if proto_unit.is_empty() && c == ' ' {
            continue
        }
        match cur_state {
            BuildState::Unit if c == ' ' => {
                let unit = replace(&mut proto_unit, String::new());
                units.push((unit, 1));
            }
            BuildState::Unit => {
                if c.is_alphabetic() {
                    proto_unit.push(c);
                } else if c == '-' || c.is_numeric() {
                    cur_state = BuildState::Exponent;
                    proto_exponent.push(c);
                }
            }
            BuildState::Exponent => {
                if c == '-' || c.is_numeric() {
                    proto_exponent.push(c);
                } else if c.is_alphabetic() || c == ' ' {
                    let unit = replace(&mut proto_unit, String::new());
                    let exponent = replace(&mut proto_exponent, String::new());
                    units.push((unit, exponent.parse().ok()?));

                    proto_unit.push(c);
                    cur_state = BuildState::Unit;
                }
            }
        }
    }

    units.into_iter()
        .map(|(n, i)| sys.get_unit(&n).map(|u| u*i))
        .fold(Some(Unit::new(NUL)), |acc, elem| acc.and_then(|a| elem.map(|e| a+e)))
}
