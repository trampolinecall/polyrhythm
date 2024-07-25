use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};

use num_rational::Ratio;

// durations are expressed in terms of whole notes
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Duration(Ratio<u32>);

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
}
