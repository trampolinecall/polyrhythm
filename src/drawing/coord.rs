use std::ops::{Add, Div, Mul, Sub};

pub mod pixel {
    use std::ops::{Add, Div, Mul, Sub};

    use crate::drawing::coord::staff_spaces::StaffSpaces;

    pub const STAFF_SPACE_PIXELS: Pixels = Pixels(20.0);

    #[derive(Copy, Clone, PartialEq, PartialOrd)]
    pub struct Pixels(pub f64);

    impl num_traits::Zero for Pixels {
        fn zero() -> Self {
            Pixels(0.0)
        }

        fn is_zero(&self) -> bool {
            *self == Pixels(0.0)
        }
    }
    impl num_traits::ConstZero for Pixels {
        const ZERO: Self = Pixels(0.0);
    }

    impl From<StaffSpaces> for Pixels {
        fn from(value: smufl::StaffSpaces) -> Pixels {
            STAFF_SPACE_PIXELS * value.0
        }
    }
    impl Add<Pixels> for Pixels {
        type Output = Pixels;

        fn add(self, rhs: Pixels) -> Pixels {
            Pixels(self.0 + rhs.0)
        }
    }

    impl Sub<Pixels> for Pixels {
        type Output = Pixels;

        fn sub(self, rhs: Pixels) -> Pixels {
            Pixels(self.0 - rhs.0)
        }
    }

    impl Mul<f64> for Pixels {
        type Output = Pixels;

        fn mul(self, rhs: f64) -> Pixels {
            Pixels(self.0 * rhs)
        }
    }

    impl Div<f64> for Pixels {
        type Output = Pixels;

        fn div(self, rhs: f64) -> Pixels {
            Pixels(self.0 / rhs)
        }
    }
}

pub mod staff_spaces {
    pub use smufl::StaffSpaces;
}

pub use pixel::Pixels;
pub use staff_spaces::StaffSpaces;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: num_traits::ConstZero> Point<T> {
    pub const ZERO: Point<T> = Point { x: T::ZERO, y: T::ZERO };
}
impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

impl From<smufl::Coord> for Point<StaffSpaces> {
    fn from(coord: smufl::Coord) -> Self {
        Point { x: coord.x(), y: coord.y() }
    }
}
impl From<Point<StaffSpaces>> for Point<Pixels> {
    fn from(point: Point<StaffSpaces>) -> Point<Pixels> {
        Point { x: point.x.into(), y: point.y.into() }
    }
}
impl<T: Add<T>> Add<Point<T>> for Point<T> {
    type Output = Point<<T as Add<T>>::Output>;

    fn add(self, rhs: Point<T>) -> Point<<T as Add>::Output> {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T: Sub<T>> Sub<Point<T>> for Point<T> {
    type Output = Point<<T as Sub<T>>::Output>;

    fn sub(self, rhs: Point<T>) -> Point<<T as Sub>::Output> {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<T: Mul<f64>> Mul<f64> for Point<T> {
    type Output = Point<<T as Mul<f64>>::Output>;

    fn mul(self, rhs: f64) -> Point<<T as Mul<f64>>::Output> {
        Point { x: self.x * rhs, y: self.y * rhs }
    }
}

impl<T: Div<f64>> Div<f64> for Point<T> {
    type Output = Point<<T as Div<f64>>::Output>;

    fn div(self, rhs: f64) -> Point<<T as Div<f64>>::Output> {
        Point { x: self.x / rhs, y: self.y / rhs }
    }
}
