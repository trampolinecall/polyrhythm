use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::drawing::{
    coord::{Pixels, Point},
    STAFF_HEIGHT,
};

#[allow(clippy::manual_non_exhaustive)]
pub struct Font {
    pub music_font_selector: String,
    pub text_font_selector: String,
    pub metadata: smufl::Metadata,
    _dont_construct: (),
}

impl Font {
    pub async fn load_bravura(window: &web_sys::Window) -> Font {
        let fetch: web_sys::Response =
            JsFuture::from(window.fetch_with_str("fonts/bravura/redist/bravura_metadata.json")).await.expect("could not load metadata for font").dyn_into().expect("fetch result should be a response");
        let metadata_contents =
            JsFuture::from(fetch.text().expect("metadata fetch response has no text")).await.expect("could not get text from metadata response").as_string().expect("fetch text should be a string");
        let metadata = smufl::Metadata::from_reader(metadata_contents.as_bytes()).expect("could not parse metadata for font");
        Font { music_font_selector: format!("{}px Bravura", STAFF_HEIGHT.0), text_font_selector: metadata.engraving_defaults.text_font_family.join(", "), metadata, _dont_construct: () }
    }
}

pub fn set_canvas_size_and_clear(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d, width: Pixels, height: Pixels) {
    canvas.set_width(width.0 as u32);
    canvas.set_height(height.0 as u32);
    ctx.clear_rect(0.0, 0.0, width.0, height.0);
}

pub fn line(ctx: &CanvasRenderingContext2d, p1: Point<Pixels>, p2: Point<Pixels>, color: &str, thickness: Pixels) {
    ctx.set_stroke_style(&color.into());
    ctx.set_line_width(thickness.0);

    ctx.begin_path();
    ctx.move_to(p1.x.0, p1.y.0);
    ctx.line_to(p2.x.0, p2.y.0);
    ctx.stroke();
}

// TODO: be able to select music or text
pub fn fill_text(ctx: &CanvasRenderingContext2d, font: &Font, text: &str, pos: Point<Pixels>) {
    ctx.set_font(&font.music_font_selector);
    ctx.fill_text(text, pos.x.0, pos.y.0).unwrap()
}
pub fn draw_glyph(ctx: &CanvasRenderingContext2d, font: &Font, glyph: smufl::Glyph, pos: Point<Pixels>) {
    fill_text(ctx, font, &glyph.codepoint().to_string(), pos)
}

pub fn bezier(ctx: &CanvasRenderingContext2d, start: Point<Pixels>, cp1: Point<Pixels>, cp2: Point<Pixels>, end: Point<Pixels>, color: &str, thickness: Pixels) {
    ctx.set_stroke_style(&color.into());
    ctx.set_line_width(thickness.0);

    ctx.begin_path();
    ctx.move_to(start.x.0, start.y.0);
    ctx.bezier_curve_to(cp1.x.0, cp1.y.0, cp2.x.0, cp2.y.0, end.x.0, end.y.0);
    ctx.stroke();
}
