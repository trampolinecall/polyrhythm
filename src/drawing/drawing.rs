use web_sys::CanvasRenderingContext2d;

use crate::drawing::coord::{Pixels, Point};

pub fn line(ctx: &CanvasRenderingContext2d, p1: Point<Pixels>, p2: Point<Pixels>, color: &str, thickness: Pixels) {
    ctx.set_stroke_style(&color.into());
    ctx.set_line_width(thickness.0);

    ctx.begin_path();
    ctx.move_to(p1.x.0, p1.y.0);
    ctx.line_to(p2.x.0, p2.y.0);
    ctx.stroke();
}

pub fn arc(ctx: &CanvasRenderingContext2d, center: Point<Pixels>, rad: Pixels, start_angle: f64, end_angle: f64) {
    ctx.arc(center.x.0, center.y.0, rad.0, start_angle, end_angle).unwrap();
}

pub fn fill_text(ctx: &CanvasRenderingContext2d, text: &str, pos: Point<Pixels>) {
    ctx.fill_text(text, pos.x.0, pos.y.0).unwrap()
}
pub fn draw_glyph(ctx: &CanvasRenderingContext2d, glyph: smufl::Glyph, pos: Point<Pixels>) {
    fill_text(ctx, &glyph.codepoint().to_string(), pos)
}
