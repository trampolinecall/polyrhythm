use num_traits::ToPrimitive;
use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

use num_rational::Ratio;

use crate::rhythm::NoteDuration;

// durations are expressed in terms of whole notes
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Duration(Ratio<u32>);

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Seconds(Ratio<u32>);

impl Duration {
    // it is safe to use new_raw here because we know that the denominators are not 0
    pub const ZERO: Duration = Duration(Ratio::new_raw(0, 1));
    pub const WHOLE_NOTE: Duration = Duration(Ratio::new_raw(1, 1));
    pub const HALF_NOTE: Duration = Duration(Ratio::new_raw(1, 2));
    pub const QUARTER_NOTE: Duration = Duration(Ratio::new_raw(1, 4));
    pub const EIGTH_NOTE: Duration = Duration(Ratio::new_raw(1, 8));
    pub const SIXTEENTH_NOTE: Duration = Duration(Ratio::new_raw(1, 16));
    pub const THIRTYSECOND_NOTE: Duration = Duration(Ratio::new_raw(1, 32));
    pub const SIXTYFOURTH_NOTE: Duration = Duration(Ratio::new_raw(1, 64));
    pub const ONEHUNDREDTWENTYSECOND_NOTE: Duration = Duration(Ratio::new_raw(1, 128));

    pub fn to_ratio(self) -> Ratio<u32> {
        self.0
    }

    pub fn to_seconds(self, tempo: (NoteDuration, u32)) -> Seconds {
        let whole_notes_per_minute = tempo.0.to_duration().to_ratio() * Ratio::from_integer(tempo.1);
        let whole_notes_per_second = whole_notes_per_minute / Ratio::from_integer(60);
        let seconds_per_whole_note = Ratio::from_integer(1) / whole_notes_per_second;
        Seconds(self.0 * seconds_per_whole_note)
    }
}

impl Add<Duration> for Duration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Duration {
        Duration(self.0 + rhs.0)
    }
}
impl Sub<Duration> for Duration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Duration {
        Duration(self.0 - rhs.0)
    }
}
impl Mul<Ratio<u32>> for Duration {
    type Output = Duration;

    fn mul(self, rhs: Ratio<u32>) -> Duration {
        Duration(self.0 * rhs)
    }
}
impl Div<Ratio<u32>> for Duration {
    type Output = Duration;

    fn div(self, rhs: Ratio<u32>) -> Duration {
        Duration(self.0 / rhs)
    }
}
impl Sum for Duration {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Duration::ZERO, |a, b| a + b)
    }
}
impl Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0) // TODO: do this better
    }
}

impl AddAssign<Duration> for Duration {
    fn add_assign(&mut self, rhs: Duration) {
        self.0 += rhs.0;
    }
}
impl SubAssign<Duration> for Duration {
    fn sub_assign(&mut self, rhs: Duration) {
        self.0 -= rhs.0;
    }
}
impl ToPrimitive for Duration {
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }

    fn to_isize(&self) -> Option<isize> {
        self.0.to_isize()
    }

    fn to_i8(&self) -> Option<i8> {
        self.0.to_i8()
    }

    fn to_i16(&self) -> Option<i16> {
        self.0.to_i16()
    }

    fn to_i32(&self) -> Option<i32> {
        self.0.to_i32()
    }

    fn to_i128(&self) -> Option<i128> {
        self.0.to_i128()
    }

    fn to_usize(&self) -> Option<usize> {
        self.0.to_usize()
    }

    fn to_u8(&self) -> Option<u8> {
        self.0.to_u8()
    }

    fn to_u16(&self) -> Option<u16> {
        self.0.to_u16()
    }

    fn to_u32(&self) -> Option<u32> {
        self.0.to_u32()
    }

    fn to_u128(&self) -> Option<u128> {
        self.0.to_u128()
    }

    fn to_f32(&self) -> Option<f32> {
        self.0.to_f32()
    }

    fn to_f64(&self) -> Option<f64> {
        self.0.to_f64()
    }
}

impl Add<Seconds> for Seconds {
    type Output = Seconds;

    fn add(self, rhs: Seconds) -> Seconds {
        Seconds(self.0 + rhs.0)
    }
}
impl Sub<Seconds> for Seconds {
    type Output = Seconds;

    fn sub(self, rhs: Seconds) -> Seconds {
        Seconds(self.0 - rhs.0)
    }
}
impl Display for Seconds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}s", self.0)
    }
}

impl ToPrimitive for Seconds {
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }

    fn to_isize(&self) -> Option<isize> {
        self.0.to_isize()
    }

    fn to_i8(&self) -> Option<i8> {
        self.0.to_i8()
    }

    fn to_i16(&self) -> Option<i16> {
        self.0.to_i16()
    }

    fn to_i32(&self) -> Option<i32> {
        self.0.to_i32()
    }

    fn to_i128(&self) -> Option<i128> {
        self.0.to_i128()
    }

    fn to_usize(&self) -> Option<usize> {
        self.0.to_usize()
    }

    fn to_u8(&self) -> Option<u8> {
        self.0.to_u8()
    }

    fn to_u16(&self) -> Option<u16> {
        self.0.to_u16()
    }

    fn to_u32(&self) -> Option<u32> {
        self.0.to_u32()
    }

    fn to_u128(&self) -> Option<u128> {
        self.0.to_u128()
    }

    fn to_f32(&self) -> Option<f32> {
        self.0.to_f32()
    }

    fn to_f64(&self) -> Option<f64> {
        self.0.to_f64()
    }
}
