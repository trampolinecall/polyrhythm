use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

use num_rational::Ratio;
use num_traits::{ConstZero, Zero};

use crate::units::WholeNotes;

// durations are expressed in terms of whole notes
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Duration<Unit>(pub Unit);
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Time<Unit>(pub Unit);

impl<Unit: ConstZero> ConstZero for Duration<Unit> {
    const ZERO: Duration<Unit> = Duration(Unit::ZERO);
}
impl<Unit: Zero> Zero for Duration<Unit> {
    fn zero() -> Self {
        Duration(Unit::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}
impl Duration<WholeNotes> {
    // it is safe to use new_raw here because we know that the denominators are not 0
    pub const WHOLE_NOTE: Duration<WholeNotes> = Duration(WholeNotes(Ratio::new_raw(1, 1)));
    pub const HALF_NOTE: Duration<WholeNotes> = Duration(WholeNotes(Ratio::new_raw(1, 2)));
    pub const QUARTER_NOTE: Duration<WholeNotes> = Duration(WholeNotes(Ratio::new_raw(1, 4)));
    pub const EIGTH_NOTE: Duration<WholeNotes> = Duration(WholeNotes(Ratio::new_raw(1, 8)));
    pub const SIXTEENTH_NOTE: Duration<WholeNotes> = Duration(WholeNotes(Ratio::new_raw(1, 16)));
    pub const THIRTYSECOND_NOTE: Duration<WholeNotes> = Duration(WholeNotes(Ratio::new_raw(1, 32)));
    pub const SIXTYFOURTH_NOTE: Duration<WholeNotes> = Duration(WholeNotes(Ratio::new_raw(1, 64)));
    pub const ONEHUNDREDTWENTYSECOND_NOTE: Duration<WholeNotes> = Duration(WholeNotes(Ratio::new_raw(1, 128)));
}

impl<Unit: Add<Output = Unit>> Add<Duration<Unit>> for Duration<Unit> {
    type Output = Duration<Unit>;

    fn add(self, rhs: Duration<Unit>) -> Duration<Unit> {
        Duration(self.0 + rhs.0)
    }
}
impl<Unit: Sub<Output = Unit>> Sub<Duration<Unit>> for Duration<Unit> {
    type Output = Duration<Unit>;

    fn sub(self, rhs: Duration<Unit>) -> Duration<Unit> {
        Duration(self.0 - rhs.0)
    }
}
impl<RT, Unit: Mul<Ratio<RT>, Output = Unit>> Mul<Ratio<RT>> for Duration<Unit> {
    type Output = Duration<Unit>;

    fn mul(self, rhs: Ratio<RT>) -> Duration<Unit> {
        Duration(self.0 * rhs)
    }
}
impl<RT, Unit: Div<Ratio<RT>, Output = Unit>> Div<Ratio<RT>> for Duration<Unit> {
    type Output = Duration<Unit>;

    fn div(self, rhs: Ratio<RT>) -> Duration<Unit> {
        Duration(self.0 / rhs)
    }
}
impl<Unit: Add<Output = Unit> + ConstZero> Sum for Duration<Unit> {
    fn sum<I: Iterator<Item = Duration<Unit>>>(iter: I) -> Duration<Unit> {
        iter.fold(Duration::ZERO, |a, b| a + b)
    }
}
impl<Unit: Display> Display for Duration<Unit> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} long", self.0)
    }
}

impl<Unit: AddAssign> AddAssign<Duration<Unit>> for Duration<Unit> {
    fn add_assign(&mut self, rhs: Duration<Unit>) {
        self.0 += rhs.0;
    }
}
impl<Unit: SubAssign> SubAssign<Duration<Unit>> for Duration<Unit> {
    fn sub_assign(&mut self, rhs: Duration<Unit>) {
        self.0 -= rhs.0;
    }
}

impl<Unit: Add<Output = Unit>> Add<Duration<Unit>> for Time<Unit> {
    type Output = Time<Unit>;

    fn add(self, rhs: Duration<Unit>) -> Time<Unit> {
        Time(self.0 + rhs.0)
    }
}
impl<Unit: Sub<Output = Unit>> Sub<Duration<Unit>> for Time<Unit> {
    type Output = Time<Unit>;

    fn sub(self, rhs: Duration<Unit>) -> Time<Unit> {
        Time(self.0 - rhs.0)
    }
}
impl<Unit: Add<Output = Unit>> Add<Time<Unit>> for Time<Unit> {
    type Output = Time<Unit>;

    fn add(self, rhs: Time<Unit>) -> Time<Unit> {
        Time(self.0 + rhs.0)
    }
}
impl<Unit: Sub<Output = Unit>> Sub<Time<Unit>> for Time<Unit> {
    type Output = Time<Unit>;

    fn sub(self, rhs: Time<Unit>) -> Time<Unit> {
        Time(self.0 - rhs.0)
    }
}
impl<RT, Unit: Mul<Ratio<RT>, Output = Unit>> Mul<Ratio<RT>> for Time<Unit> {
    type Output = Time<Unit>;

    fn mul(self, rhs: Ratio<RT>) -> Time<Unit> {
        Time(self.0 * rhs)
    }
}
impl<RT, Unit: Div<Ratio<RT>, Output = Unit>> Div<Ratio<RT>> for Time<Unit> {
    type Output = Time<Unit>;

    fn div(self, rhs: Ratio<RT>) -> Time<Unit> {
        Time(self.0 / rhs)
    }
}
impl<Unit: AddAssign> AddAssign<Duration<Unit>> for Time<Unit> {
    fn add_assign(&mut self, rhs: Duration<Unit>) {
        self.0 += rhs.0;
    }
}
impl<Unit: SubAssign> SubAssign<Duration<Unit>> for Time<Unit> {
    fn sub_assign(&mut self, rhs: Duration<Unit>) {
        self.0 -= rhs.0;
    }
}
impl<Unit: ConstZero> ConstZero for Time<Unit> {
    const ZERO: Time<Unit> = Time(Unit::ZERO);
}
impl<Unit: Zero> Zero for Time<Unit> {
    fn zero() -> Self {
        Time(Unit::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}
