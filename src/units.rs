use std::ops::Div;
use std::fmt::Debug;
use std::cmp::Ordering;
use std::collections::HashMap;

use num::Float;

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

pub struct UnitSystem<N: Float> {
    pub base: BaseUnits,
    pub units: HashMap<&'static str, Unit<N>>
}

impl<N: Float> UnitSystem<N> {
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
        let mut ret = Self::new_base_with_cap(SI, 16);

        ret.units.insert("J", Unit::new(ENERGY));
        ret.units.insert("min", Unit::with_factor(TIME, N::from(60).unwrap()));
        ret.units.insert("h", Unit::with_factor(TIME, N::from(3600).unwrap()));
        ret.units.insert("km", Unit::with_factor(LENGTH, N::from(1000).unwrap()));
        ret.units.insert("g", Unit::with_factor(MASS, N::from(1e-3).unwrap()));
        ret.units.insert("Hz", Unit::new(FREQUENCY));
        ret.units.insert("L", Unit::with_factor(VOLUME, N::from(1e-3).unwrap()));
        ret.units.insert("mL", Unit::with_factor(VOLUME, N::from(1e-6).unwrap()));
        ret.units.insert("M", Unit::with_factor(CONCENTRATION, N::from(1e3).unwrap()));
        ret.units.insert("N", Unit::new(FORCE));
        ret.units.insert("kN", Unit::with_factor(FORCE, N::from(1e3).unwrap()));
        ret.units.insert("W", Unit::new(POWER));
        ret.units.insert("V", Unit::new(VOLTAGE));
        ret.units.insert("mA", Unit::with_factor(CURRENT, N::from(1e-3).unwrap()));
        ret.units.insert("Ω", Unit::new(RESISTANCE));
        ret.units.insert("C", Unit::new(CHARGE));
        ret.units.insert("Pa", Unit::new(PRESSURE));

        ret
    }
    pub fn add_unit(&mut self, name: &'static str, unit: Unit<N>) -> Option<Unit<N>> {
        self.units.insert(name, unit)
    }
    pub fn get_unit(&self, name: &str) -> Option<Unit<N>> {
        self.units.get(name).cloned()
    }
    pub fn val(&self, val: N, unit: &str) -> Value<N> {
        Value(val, self.units[unit])
    }
    pub fn as_(&self, val: Value<N>, unit: &str) -> Value<N> {
        self.cast(val, &self[unit])
    }
    pub fn cast(&self, val: Value<N>, unit: &Unit<N>) -> Value<N> {
        if val.1.dimension == unit.dimension {
            Value(val.0 * (val.1.factor/unit.factor), *unit)
        } else {
            panic!("Tried to cast from {:#} to {:#}", val.1.dimension, unit.dimension);
        }
    }
    pub fn display<'a>(&'a self, val: &'a Value<N>) -> UnitDisplay<'a, N> {
        display::make_display(self, val)
    }
}

use std::ops::Index;

impl<'a, N: Float> Index<&'a str> for UnitSystem<N> {
    type Output = Unit<N>;
    fn index(&self, name: &'a str) -> &Self::Output {
        &self.units[name]
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Unit<N: Float> {
    pub dimension: Dimension,
    pub factor: N
}

impl<N: Float> PartialOrd for Unit<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.dimension == other.dimension {
            self.factor.partial_cmp(&other.factor)
        } else {
            None
        }
    }
}

impl<N: Float> Unit<N> {
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

impl<N: Float> Add for Unit<N> {
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

impl<N: Float> Sub for Unit<N> {
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

impl<N: Float> Mul<i16> for Unit<N> {
    type Output = Self;
    fn mul(self, rhs: i16) -> Self::Output {
        let Unit{factor, dimension} = self;
        Unit{
            factor: factor.powi(rhs as i32),
            dimension: rhs*dimension
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Value<N: Float>(pub N, pub Unit<N>);

impl<N: Float> PartialOrd for Value<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.1.dimension == other.1.dimension {
            let factor = self.1.factor/other.1.factor;
            self.0.partial_cmp(&(self.0 * factor))
        } else {
            None
        }
    }
}

impl<N: Float + Debug> Add for Value<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        assert_eq!(self.1, rhs.1);
        Value(self.0+rhs.0, self.1)
    }
}

impl<N: Float + Debug> Sub for Value<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        assert_eq!(self.1, rhs.1);
        Value(self.0-rhs.0, self.1)
    }
}

impl<N: Float> Mul for Value<N> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Value(self.0*rhs.0, self.1+rhs.1)
    }
}

impl<N: Float> Div for Value<N> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Value(self.0/rhs.0, self.1-rhs.1)
    }
}

impl<N: Float> Mul<N> for Value<N> {
    type Output = Self;
    fn mul(self, rhs: N) -> Self {
        Value(self.0*rhs, self.1)
    }
}

impl<N: Float> Div<N> for Value<N> {
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

mul_div_primitive!{f32 f64}
