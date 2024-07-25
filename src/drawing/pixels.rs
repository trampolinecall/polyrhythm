use std::ops::{Add, Div, Mul, Sub};

use smufl::StaffSpaces;
use web_sys::CanvasRenderingContext2d;

use crate::drawing::STAFF_SPACE_PIXELS;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Pixels(pub f64);

impl From<f64> for Pixels {
    fn from(value: f64) -> Pixels {
        Pixels(value)
    }
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

pub fn move_to(ctx: &CanvasRenderingContext2d, x: Pixels, y: Pixels) {
    ctx.move_to(x.0, y.0);
}
pub fn line_to(ctx: &CanvasRenderingContext2d, x: Pixels, y: Pixels) {
    ctx.line_to(x.0, y.0);
}
pub fn arc(ctx: &CanvasRenderingContext2d, x: Pixels, y: Pixels, rad: Pixels, start_angle: f64, end_angle: f64) {
    ctx.arc(x.0, y.0, rad.0, start_angle, end_angle).unwrap();
}
pub fn fill_text(ctx: &CanvasRenderingContext2d, text: &str, x: Pixels, y: Pixels) {
    ctx.fill_text(text, x.0, y.0).unwrap()
}
