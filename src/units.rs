use std::ops::{Add, Sub, Mul, Div};
use std::fmt::Debug;
use std::cmp::Ordering;
use std::collections::HashMap;

use num::Float;

use super::*;

/// The base units for each dimension
pub struct BaseUnits {
    /// Base unit for length
    pub length: &'static str,
    /// Base unit for time
    pub time: &'static str,
    /// Base unit for mass
    pub mass: &'static str,
    /// Base unit for current
    pub current: &'static str,
    /// Base unit for temperature
    pub temperature: &'static str,
    /// Base unit for amount of substance
    pub substance_amount: &'static str,
    /// Base unit for luminous intensity
    pub luminous_intensity: &'static str,
}

/// The base units of SI
pub const SI: BaseUnits = BaseUnits {
    /// The SI base unit for length: metres
    length: "m",
    /// The SI base unit for time: seconds
    time: "s",
    /// The SI base unit for mass: kilogrammes
    mass: "kg",
    /// The SI base unit for current: amperes
    current: "A",
    /// The SI base unit for temperature: kelvin
    temperature: "K",
    /// The SI base unit for amount of substance: moles
    substance_amount: "mol",
    /// The SI base unit for luminous intensity: candelas
    luminous_intensity: "cd",
};

/// A collection of units and their ratios to each other
///
/// Used for writing and reading units from and to strings
pub struct UnitSystem<N: Float> {
    /// The set of base units for this system
    pub base: BaseUnits,
    /// Derivative units for this system and their relation to the base units
    pub units: HashMap<&'static str, Unit<N>>
}

impl<N: Float> UnitSystem<N> {
    /// Creates a new `UnitSystem` from a base with a specified capacity
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
    /// Creates a system with SI units
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
    /// Add a unit to the system
    pub fn add_unit(&mut self, name: &'static str, unit: Unit<N>) -> Option<Unit<N>> {
        self.units.insert(name, unit)
    }
    /// Returns the unit with the given name if it exists
    ///
    /// This can only take units that aren't composite (i.e m, C, s, etc., but not m³, m/s or s^-1)
    pub fn get_unit(&self, name: &str) -> Option<Unit<N>> {
        self.units.get(name).cloned()
    }
    /// Returns a composite unit from a string
    ///
    /// E.g `"m²"` should return a `Unit` for m, if it exists, squared
    pub fn unit_from_str(&self, s: &str) -> Option<Unit<N>> {
        unit_from_str(self, s)
    }
    /// Returns a value with the given composite unit from a string if the units exist
    pub fn val(&self, val: N, unit_str: &str) -> Option<Value<N>> {
        Some(Value(val, self.unit_from_str(unit_str)?))
    }
    /// Returns a value from a string
    pub fn val_s(&self, value: &str) -> Option<Value<N>>
    where N: std::str::FromStr {
        value_from_str(self, value)
    }
    /// Casts a `Value` to the one given
    ///
    /// `unit` may be composite
    /// ## Panics
    /// Panicks if `val` doesn't have the same dimension as the unit
    pub fn as_(&self, val: Value<N>, unit: &str) -> Value<N> {
        self.cast(val, &self.unit_from_str(unit).unwrap())
    }
    /// Casts a `Value` to the one given
    ///
    /// ## Panics
    /// Panicks if `val` doesn't have the same dimension as `unit`
    pub fn cast(&self, val: Value<N>, unit: &Unit<N>) -> Value<N> {
        if val.1.dimension == unit.dimension {
            Value(val.0 * (val.1.factor/unit.factor), *unit)
        } else {
            panic!("Tried to cast from {:#} to {:#}", val.1.dimension, unit.dimension);
        }
    }
    /// Returns a `UnitDisplay` used to display a value
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
/// A unit
pub struct Unit<N: Float> {
    /// Dimensions of this unit
    pub dimension: Dimension,
    /// The number to multiply it by to get it in terms of base units
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
    /// Creates a new unit with the given dimension
    ///
    /// Will assume to be made of base units
    pub fn new(dimension: Dimension) -> Self {
        Unit {
            factor: N::one(),
            dimension
        }
    }
    /// Creates a new unit with the given dimension and factor
    ///
    /// Factor is the number to multiply it by to get it in terms of base units
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
/// A floating value with an associated unit.
///
/// Will only allow addition and subtraction with values of the same dimensions.
/// Unit will change accordingly when performing mathematical operations.
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
        assert_eq!(self.1.dimension, rhs.1.dimension);
        let convert;
        if self.1.factor != rhs.1.factor {
            convert = rhs.1.factor/self.1.factor;
        } else {
            convert = N::one();
        }
        Value(self.0+convert*rhs.0, self.1)
    }
}

impl<N: Float + Debug> Sub for Value<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        assert_eq!(self.1.dimension, rhs.1.dimension);
        let convert;
        if self.1.factor != rhs.1.factor {
            convert = rhs.1.factor/self.1.factor;
        } else {
            convert = N::one();
        }
        Value(self.0-convert*rhs.0, self.1)
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
