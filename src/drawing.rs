use num_rational::Ratio;
use num_traits::cast::ToPrimitive;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::polyrhythm::Polyrhythm;

pub fn draw(canvas: &HtmlCanvasElement, polyrhythm: &Polyrhythm) {
    let ctx: CanvasRenderingContext2d = canvas.get_context("2d").expect("could not get canvas context").unwrap().dyn_into().expect("2d canvas context should be CanvasRenderingContext2d");

    // TODO: adjust this
    canvas.set_width(1000);
    canvas.set_height(500);
    ctx.clear_rect(0.0, 0.0, 1000.0, 500.0);

    const WHOLE_NOTE_SIZE: f64 = 400.0;
    for (i, rhythm) in polyrhythm.rhythms.iter().enumerate() {
        let y = (i * 100 + 50) as f64;
        ctx.begin_path();
        ctx.move_to(0.0, y);
        ctx.line_to(1000.0, y);
        ctx.set_stroke_style(&"black".into());
        ctx.stroke();

        let mut x: Ratio<u32> = Ratio::ZERO;
        for duration in rhythm.note_durations() {
            ctx.begin_path();
            ctx.arc(x.to_f64().unwrap() * WHOLE_NOTE_SIZE, y, 10.0, 0.0, std::f64::consts::TAU).unwrap();
            ctx.set_fill_style(&"black".into());
            ctx.fill();
            ctx.begin_path();
            ctx.arc(x.to_f64().unwrap() * WHOLE_NOTE_SIZE, y, 10.0, 0.0, std::f64::consts::TAU).unwrap();
            ctx.set_fill_style(&"black".into());
            ctx.stroke();
            ctx.begin_path();
            ctx.move_to(x.to_f64().unwrap() * WHOLE_NOTE_SIZE, y - 100.0);
            ctx.move_to(x.to_f64().unwrap() * WHOLE_NOTE_SIZE, y + 100.0);
            ctx.set_fill_style(&"black".into());
            ctx.stroke();
            web_sys::console::log_1(&"efjaowie".into());
            web_sys::console::log_4(&(x.to_f64().unwrap() * WHOLE_NOTE_SIZE).into(), &y.into(), &0.0.into(), &std::f64::consts::TAU.into());

            x += duration.to_ratio();
        }
    }
}
