use super::*;

use std::ops::{Add, Mul, Sub};
use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq, Default, Copy, Clone, Hash)]
/// Struct to keep track of the dimension of a quantity for dimensional analysis.
/// Based on the SI units.
pub struct Dimension {
    /// The exponent of length
    pub length: i16,
    /// The exponent of time
    pub time: i16,
    /// The exponent of mass
    pub mass: i16,
    /// The exponent of current
    pub current: i16,
    /// The exponent of temperature
    pub temperature: i16,
    /// The exponent of amount of substance
    pub substance_amount: i16,
    /// The exponent of luminous intensity
    pub luminous_intensity: i16
}

#[test]
fn super_test() {
    assert_eq!(to_superscript("-124"), "⁻¹²⁴");
    assert_eq!(to_superscript("asdg ja-kage4²"), "asdg ja⁻kage⁴²");
}

/// The null-dimension, indicating a dimensionless quantity
pub const NUL: Dimension = Dimension{mass:0,length:0,time:0,current:0,temperature:0,substance_amount:0,luminous_intensity:0};

macro_rules! dims {
    ($($cnst:ident, $display_name:expr; {$($n:ident : $v:expr),+},)*) => (
        $(
            /// A dimension constant
            pub const $cnst: Dimension = Dimension{$($n: $v,)+..NUL};
        )*
        impl Display for Dimension {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if f.alternate() {$(
                    if *self== $cnst {
                        return Display::fmt($display_name, f);
                    }
                )*}

                let Dimension{mass,length,time,current,temperature,substance_amount,luminous_intensity} = *self;
                let mut s = String::new();
                if mass != 0 {
                    s.push_str("[mass]");
                    s.push_str(&to_superscript(&format!("{}",mass)));
                }
                if length != 0 {
                    s.push_str("[length]");
                    s.push_str(&to_superscript(&format!("{}", length)));
                }
                if time != 0 {
                    s.push_str("[time]");
                    s.push_str(&to_superscript(&format!("{}", time)));
                }
                if current != 0 {
                    s.push_str("[current]");
                    s.push_str(&to_superscript(&format!("{}", current)));
                }
                if temperature != 0 {
                    s.push_str("[temperature]");
                    s.push_str(&to_superscript(&format!("{}", temperature)));
                }
                if substance_amount != 0 {
                    s.push_str("[substance amount]");
                    s.push_str(&to_superscript(&format!("{}", substance_amount)));
                }
                if luminous_intensity != 0 {
                    s.push_str("[luminous intensity]");
                    s.push_str(&to_superscript(&format!("{}", luminous_intensity)));
                }
                if s.is_empty() && f.alternate() {
                    s.push_str("Dimensionless")
                }
                Display::fmt(&s, f)
            }
        }
    );
}

dims!{
    MASS, "Mass"; {mass:1},
    LENGTH, "Length"; {length:1},
    TIME, "Time"; {time:1},
    CURRENT, "Current"; {current:1},
    TEMPERATURE, "Temperature"; {temperature:1},
    AMOUNT_OF_SUBSTANCE, "Amount of Substance"; {substance_amount:1},
    LUMINOUS_INTENSITY, "Luminous Intensity"; {luminous_intensity:1},

    AREA, "Area"; {length:2},
    VOLUME, "Volume"; {length:3},
    DENSITY, "Density"; {mass:1,length:-3},

    FREQUENCY, "Frequency"; {time:-1},

    VELOCITY, "Velocity"; {length:1,time:-1},
    ACCELERATION, "Acceleration"; {length:1,time:-2},
    MOMENTUM, "Momentum"; {length:1,mass:1,time:-1},
    FORCE, "Force"; {mass:1,length:1,time:-2},
    ACTION, "Action"; {mass:1,length:2,time:-1},
    ENERGY, "Energy"; {mass:1,length:2,time:-2},

    MOLAR_MASS, "Molar Mass"; {mass:1,substance_amount:-1},
    CONCENTRATION, "Concentration"; {substance_amount:1,length:-3},

    POWER, "Power"; {mass:1,length:2,time:-3},
    VOLTAGE, "Voltage"; {mass:1,length:2,time:-3,current:-1},
    RESISTANCE, "Resistance"; {mass:1,length:2,time:-3,current:-2},
    CHARGE, "Charge"; {current:1,time:1},

    PRESSURE, "Pressure"; {mass:1,length:-1,time:-2},
}

impl Add for Dimension {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let Dimension{length, time, mass, current, temperature, substance_amount, luminous_intensity} = self;
        let Dimension{length:s, time:t, mass:m, current:i, temperature:temp, substance_amount:n, luminous_intensity:j} = rhs;

        Dimension{
            length: length + s,
            time: time + t,
            mass: mass + m,
            current: current + i,
            temperature: temperature + temp,
            substance_amount: substance_amount + n,
            luminous_intensity: luminous_intensity + j,
        }
    }
}

impl Sub for Dimension {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let Dimension{length, time, mass, current, temperature, substance_amount, luminous_intensity} = self;
        let Dimension{length:s, time:t, mass:m, current:i, temperature:temp, substance_amount:n, luminous_intensity:j} = rhs;

        Dimension{
            length: length - s,
            time: time - t,
            mass: mass - m,
            current: current - i,
            temperature: temperature - temp,
            substance_amount: substance_amount - n,
            luminous_intensity: luminous_intensity - j,
        }
    }
}

impl Mul<i16> for Dimension {
    type Output = Self;
    fn mul(self, factor: i16) -> Self {
        let Dimension{length, time, mass, current, temperature, substance_amount, luminous_intensity} = self;

        Dimension{
            length: length * factor,
            time: time * factor,
            mass: mass * factor,
            current: current * factor,
            temperature: temperature * factor,
            substance_amount: substance_amount * factor,
            luminous_intensity: luminous_intensity * factor,
        }
    }
}

impl Mul<Dimension> for i16 {
    type Output = Dimension;
    fn mul(self, dim: Dimension) -> Dimension {
        let Dimension{length, time, mass, current, temperature, substance_amount, luminous_intensity} = dim;

        Dimension{
            length: length * self,
            time: time * self,
            mass: mass * self,
            current: current * self,
            temperature: temperature * self,
            substance_amount: substance_amount * self,
            luminous_intensity: luminous_intensity * self,
        }
    }
}
