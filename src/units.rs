use std::collections::HashMap;

use num::Num;
use std::ops::Div;

use super::*;

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

pub struct UnitSystem<N: Num> {
    pub base: BaseUnits,
    pub units: HashMap<&'static str, Unit<N>>
}

impl<N: Num + From<u32> + From<f32> + Copy> UnitSystem<N> {
    pub fn new_base_with_cap(base: BaseUnits, cap: usize) -> Self {
        let mut units = HashMap::with_capacity(7+cap);
        units.insert(base.length, Unit::new(LENGTH));
        units.insert(base.time, Unit::new(TIME));
        units.insert(base.mass, Unit::new(MASS));
        units.insert(base.current, Unit::new(CURRENT));
        units.insert(base.temperature, Unit::new(TEMPERATURE));
        units.insert(base.substance_amount, Unit::new(AMOUNT_OF_SUBSTANCE));
        units.insert(base.luminous_intensity, Unit::new(LUMINOUS_INTENSITY));

        UnitSystem {
            base,
            units,
        }
    }

    pub fn si() -> Self {
        let mut ret = Self::new_base_with_cap(SI, 13);

        ret.units.insert("J", Unit::new(ENERGY));
        ret.units.insert("min", Unit::with_factor(TIME, N::from(60)));
        ret.units.insert("h", Unit::with_factor(TIME, N::from(3600)));
        ret.units.insert("km", Unit::with_factor(LENGTH, N::from(1000)));
        ret.units.insert("g", Unit::with_factor(MASS, N::from(1e-3)));
        ret.units.insert("Hz", Unit::new(FREQUENCY));
        ret.units.insert("L", Unit::with_factor(VOLUME, N::from(1e-3)));
        ret.units.insert("mL", Unit::with_factor(VOLUME, N::from(1e-6)));
        ret.units.insert("M", Unit::with_factor(CONCENTRATION, N::from(1e3)));
        ret.units.insert("N", Unit::new(FORCE));
        ret.units.insert("W", Unit::new(POWER));
        ret.units.insert("V", Unit::new(VOLTAGE));
        ret.units.insert("Ω", Unit::new(RESISTANCE));
        ret.units.insert("C", Unit::new(CHARGE));
        ret.units.insert("Pa", Unit::new(PRESSURE));

        ret
    }
    pub fn add_unit(&mut self, name: &'static str, unit: Unit<N>) -> Option<Unit<N>> {
        self.units.insert(name, unit)
    }
    pub fn get_unit(&self, name: &str) -> Unit<N> {
        self.units[name]
    }
    pub fn val(&self, val: N, unit: &str) -> Value<N> {
        Value(val, self.units[unit])
    }
    pub fn display<'a>(&'a self, val: &'a Value<N>) -> UnitDisplay<'a, N> {
        display::make_display(self, val)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Unit<N: Num> {
    pub dimension: Dimension,
    pub factor: N
}

impl<N: Num> Unit<N> {
    pub fn new(dimension: Dimension) -> Self {
        Unit {
            factor: N::one(),
            dimension
        }
    }
    pub fn with_factor(dimension: Dimension, factor: N) -> Self {
        Unit {
            factor,
            dimension
        }
    }
}

impl<N: Num> Add for Unit<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let Unit{factor, dimension} = self;
        let Unit{factor:f, dimension:d} = rhs;
        Unit{
            factor: factor*f,
            dimension: dimension+d
        }
    }
}

impl<N: Num> Sub for Unit<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let Unit{factor, dimension} = self;
        let Unit{factor:f, dimension:d} = rhs;
        Unit{
            factor: factor/f,
            dimension: dimension-d
        }
    }
}

impl<N: Num> Mul<i16> for Unit<N> {
    type Output = Self;
    fn mul(self, rhs: i16) -> Self::Output {
        let Unit{factor, dimension} = self;
        Unit{
            factor: factor,
            dimension: rhs*dimension
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Value<N: Num>(pub N, pub Unit<N>);

use std::fmt::Debug;

impl<N: Num + Debug> Add for Value<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        assert_eq!(self.1, rhs.1);
        Value(self.0+rhs.0, self.1)
    }
}

impl<N: Num + Debug> Sub for Value<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        assert_eq!(self.1, rhs.1);
        Value(self.0-rhs.0, self.1)
    }
}

impl<N: Num> Mul for Value<N> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Value(self.0*rhs.0, self.1+rhs.1)
    }
}

impl<N: Num> Div for Value<N> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Value(self.0/rhs.0, self.1-rhs.1)
    }
}

impl<N: Num> Mul<N> for Value<N> {
    type Output = Self;
    fn mul(self, rhs: N) -> Self {
        Value(self.0*rhs, self.1)
    }
}

impl<N: Num> Div<N> for Value<N> {
    type Output = Self;
    fn div(self, rhs: N) -> Self {
        Value(self.0/rhs, self.1)
    }
}

macro_rules! mul_div_primitive {
    ($($t:ty)*) => ($(
        impl Mul<Value<$t>> for $t {
            type Output = Value<Self>;
            fn mul(self, rhs: Value<$t>) -> Self::Output {
                Value(self*rhs.0, rhs.1)
            }
        }
    )*);
}

mul_div_primitive!{f32 f64 i8 u8 i16 u16 i32 u32 i64 u64}
