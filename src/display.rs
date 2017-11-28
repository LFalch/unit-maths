use num::Float;

use super::*;

pub struct UnitDisplay<'a, N: 'a + Float> {
    val: &'a Value<N>,
    sys: &'a UnitSystem<N>,
}

pub fn make_display<'a, N: 'a + Float>(sys: &'a UnitSystem<N>, val: &'a Value<N>) -> UnitDisplay<'a, N> {
    UnitDisplay {val, sys}
}

impl<'a, N: 'a + Float + Display> Display for UnitDisplay<'a, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nu = self.sys.units
            .iter()
            .filter(|&(_, u)| u.dimension==self.val.1.dimension)
            .min_by(|&(_, u), &(_, u2)| {
                (u.factor-self.val.1.factor).abs().partial_cmp(&(u2.factor-self.val.1.factor).abs()).unwrap()
            });

        if let Some((name, unit)) = nu {
            (self.val.0  * self.val.1.factor / unit.factor).fmt(f)?;
            return write!(f, " {}", name)
        }

        let Dimension{mass,length,time,current,temperature,substance_amount,luminous_intensity} = self.val.1.dimension;
        let mut s = String::new();
        if mass != 0 {
            s.push_str(self.sys.base.mass);
            s.push_str(&to_superscript(&format!("{}",mass)));
        }
        if length != 0 {
            s.push_str(self.sys.base.length);
            s.push_str(&to_superscript(&format!("{}", length)));
        }
        if time != 0 {
            s.push_str(self.sys.base.time);
            s.push_str(&to_superscript(&format!("{}", time)));
        }
        if current != 0 {
            s.push_str(self.sys.base.current);
            s.push_str(&to_superscript(&format!("{}", current)));
        }
        if temperature != 0 {
            s.push_str(self.sys.base.temperature);
            s.push_str(&to_superscript(&format!("{}", temperature)));
        }
        if substance_amount != 0 {
            s.push_str(self.sys.base.substance_amount);
            s.push_str(&to_superscript(&format!("{}", substance_amount)));
        }
        if luminous_intensity != 0 {
            s.push_str(self.sys.base.luminous_intensity);
            s.push_str(&to_superscript(&format!("{}", luminous_intensity)));
        }
        Display::fmt(&(self.val.0 * self.val.1.factor), f)?;
        write!(f, " {}", s)
    }
}
