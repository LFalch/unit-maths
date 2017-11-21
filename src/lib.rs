use std::ops::{Add, Mul, Sub, Div, Neg};
use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq, Default, Copy, Clone)]
pub struct Dimension {
    length: i16,
    time: i16,
    mass: i16,
    current: i16,
    temperature: i16,
    substance_amount: i16,
    luminous_intensity: i16
}

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

pub const NUL: Dimension = Dimension{mass:0,length:0,time:0,current:0,temperature:0,substance_amount:0,luminous_intensity:0};

macro_rules! dims {
    ($($cnst:ident, $display_name:expr; {$($n:ident : $v:expr),+},)*) => (
        $(
            pub const $cnst: Dimension = Dimension{$($n: $v,)+..NUL};
        )*
        impl Display for Dimension {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if f.alternate() {$(
                    if *self== $cnst {
                        return $display_name.fmt(f);
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
                s.fmt(f)
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

    FREQUENCY, "Frequency"; {time:-1},

    VELOCITY, "Velocity"; {length:1,time:-1},
    ACCELERATION, "Acceleration"; {length:1,time:-2},
    MOLAR_MASS, "Molar Mass"; {substance_amount:1,mass: -1},
    FORCE, "Force"; {mass:1,length:1,time:-2},
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

pub struct BaseUnits {
    pub length: &'static str,
    pub time: &'static str,
    pub mass: &'static str,
    pub current: &'static str,
    pub temperature: &'static str,
    pub substance_amount: &'static str,
    pub luminous_intensity: &'static str,
}

pub const SI: BaseUnits = BaseUnits {
    length: "m",
    time: "s",
    mass: "kg",
    current: "A",
    temperature: "K",
    substance_amount: "mol",
    luminous_intensity: "cd",
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Value<N>(pub N, pub Dimension) where N: Add<Output=N> + Mul<Output=N> + Sub<Output=N> + Div<Output=N> + Neg;

impl<N: Display> Display for Value<N>
where N: Add<Output=N> + Mul<Output=N> + Sub<Output=N> + Div<Output=N> + Neg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)?;
        if f.alternate() {
            f.write_fmt(format_args!(" ({:#})", self.1))
        } else {
            f.write_fmt(format_args!(" ({})", self.1))
        }
    }
}

impl<N: Add<Output=N> + Mul<Output=N> + Sub<Output=N> + Div<Output=N> + Neg> Add for Value<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        assert_eq!(self.1, rhs.1);
        Value(self.0+rhs.0, self.1)
    }
}

impl<N: Add<Output=N> + Mul<Output=N> + Sub<Output=N> + Div<Output=N> + Neg> Sub for Value<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        assert_eq!(self.1, rhs.1);
        Value(self.0-rhs.0, self.1)
    }
}

impl<N: Add<Output=N> + Mul<Output=N> + Sub<Output=N> + Div<Output=N> + Neg> Mul for Value<N> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Value(self.0*rhs.0, self.1+rhs.1)
    }
}

impl<N: Add<Output=N> + Mul<Output=N> + Sub<Output=N> + Div<Output=N> + Neg> Div for Value<N> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Value(self.0/rhs.0, self.1-rhs.1)
    }
}
