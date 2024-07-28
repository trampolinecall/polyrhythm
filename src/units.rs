use num_traits::{ConstZero, Signed, Zero};
use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use num_rational::Ratio;

use crate::rhythm::NoteDuration;

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct WholeNotes(pub Ratio<i32>);
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Seconds(pub Ratio<i32>);

impl WholeNotes {
    pub fn to_seconds(self, tempo: (NoteDuration, u32)) -> Seconds {
        let whole_notes_per_minute = tempo.0.to_duration().0 * Ratio::from_integer(tempo.1.try_into().unwrap());
        let whole_notes_per_second = whole_notes_per_minute / Ratio::from_integer(60);
        let seconds_per_whole_note = Seconds(Ratio::from_integer(1)) / whole_notes_per_second.0;
        seconds_per_whole_note * self.0
    }
}
impl ConstZero for WholeNotes {
    const ZERO: Self = WholeNotes(Ratio::ZERO);
}
impl Zero for WholeNotes {
    fn zero() -> Self {
        Self::ZERO
    }

    fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
}
impl Add<WholeNotes> for WholeNotes {
    type Output = WholeNotes;

    fn add(self, rhs: WholeNotes) -> WholeNotes {
        WholeNotes(self.0 + rhs.0)
    }
}
impl Sub<WholeNotes> for WholeNotes {
    type Output = WholeNotes;

    fn sub(self, rhs: WholeNotes) -> WholeNotes {
        WholeNotes(self.0 - rhs.0)
    }
}
impl Mul<Ratio<i32>> for WholeNotes {
    type Output = WholeNotes;

    fn mul(self, rhs: Ratio<i32>) -> WholeNotes {
        WholeNotes(self.0 * rhs)
    }
}
impl Div<Ratio<i32>> for WholeNotes {
    type Output = WholeNotes;

    fn div(self, rhs: Ratio<i32>) -> WholeNotes {
        WholeNotes(self.0 / rhs)
    }
}
impl Sum for WholeNotes {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(WholeNotes::ZERO, |a, b| a + b)
    }
}
impl Display for WholeNotes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} whole notes", self.0)
    }
}

impl AddAssign<WholeNotes> for WholeNotes {
    fn add_assign(&mut self, rhs: WholeNotes) {
        self.0 += rhs.0;
    }
}
impl SubAssign<WholeNotes> for WholeNotes {
    fn sub_assign(&mut self, rhs: WholeNotes) {
        self.0 -= rhs.0;
    }
}

impl Seconds {
    pub(crate) fn abs(&self) -> Seconds {
        Seconds(self.0.abs())
    }
}
impl ConstZero for Seconds {
    const ZERO: Self = Seconds(Ratio::ZERO);
}
impl Zero for Seconds {
    fn zero() -> Self {
        Self::ZERO
    }

    fn is_zero(&self) -> bool {
        *self == Self::ZERO
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
impl Mul<Ratio<i32>> for Seconds {
    type Output = Seconds;

    fn mul(self, rhs: Ratio<i32>) -> Seconds {
        Seconds(self.0 * rhs)
    }
}
impl Div<Ratio<i32>> for Seconds {
    type Output = Seconds;

    fn div(self, rhs: Ratio<i32>) -> Seconds {
        Seconds(self.0 / rhs)
    }
}
impl Sum for Seconds {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Seconds::ZERO, |a, b| a + b)
    }
}
impl Display for Seconds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}s", self.0)
    }
}

impl AddAssign<Seconds> for Seconds {
    fn add_assign(&mut self, rhs: Seconds) {
        self.0 += rhs.0;
    }
}
impl SubAssign<Seconds> for Seconds {
    fn sub_assign(&mut self, rhs: Seconds) {
        self.0 -= rhs.0;
    }
}
impl MulAssign<Ratio<i32>> for Seconds {
    fn mul_assign(&mut self, rhs: Ratio<i32>) {
        self.0 *= rhs;
    }
}
impl DivAssign<Ratio<i32>> for Seconds {
    fn div_assign(&mut self, rhs: Ratio<i32>) {
        self.0 /= rhs;
    }
}
