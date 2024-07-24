use num_rational::Ratio;
use num_traits::cast::ToPrimitive;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::{
    polyrhythm::Polyrhythm,
    rhythm::{NoteDuration, Rhythm},
};

// TODO: do this better (scale so that the shortest note is a comfortable distance from the next?)
const WHOLE_NOTE_SIZE: f64 = 500.0;
// TODO: decide on a better value for this (make this vary between each rhythm based on the highest number of flags on the shortest note?)
const RHYTHM_HEIGHT: f64 = 100.0;

pub fn draw(canvas: &HtmlCanvasElement, polyrhythm: &Polyrhythm) {
    let ctx: CanvasRenderingContext2d = canvas.get_context("2d").expect("could not get canvas context").unwrap().dyn_into().expect("2d canvas context should be CanvasRenderingContext2d");

    // TODO: adjust this
    canvas.set_width(1000);
    canvas.set_height(500);
    ctx.clear_rect(0.0, 0.0, 1000.0, 500.0);

    for (i, rhythm) in polyrhythm.rhythms.iter().enumerate() {
        let y = (i as f64) * RHYTHM_HEIGHT + RHYTHM_HEIGHT / 2.0;
        ctx.begin_path();
        ctx.move_to(0.0, y);
        ctx.line_to(1000.0, y);
        ctx.set_stroke_style(&"black".into());
        ctx.stroke();

        for note in flatten_rhythm(rhythm) {
            let x = note.time.to_f64().unwrap() * WHOLE_NOTE_SIZE;
            if note.is_rest {
                draw_rest(&ctx, note.duration, x, y);
            } else {
                draw_notehead(&ctx, note.duration, x, y);
            }
        }
    }
}

fn draw_rest(ctx: &CanvasRenderingContext2d, duration: NoteDuration, x: f64, y: f64) {
    // TODO: do this actually
    ctx.begin_path();
    ctx.arc(x, y, 2.0, 0.0, std::f64::consts::TAU).unwrap();
    ctx.set_fill_style(&"black".into());
    ctx.fill();
}

fn draw_notehead(ctx: &CanvasRenderingContext2d, duration: NoteDuration, x: f64, y: f64) {
    // TODO: modify this based on the note duration
    ctx.begin_path();
    ctx.arc(x, y, 8.0, 0.0, std::f64::consts::TAU).unwrap();
    ctx.set_fill_style(&"black".into());
    ctx.fill();
}

struct FlattenedNote {
    time: Ratio<u32>,
    is_rest: bool,
    duration: NoteDuration,
}
fn flatten_rhythm(r: &Rhythm) -> Vec<FlattenedNote> {
    let mut current_time = Ratio::ZERO;

    let mut notes = Vec::new();

    for segment in &r.segments {
        match segment {
            crate::rhythm::RhythmSegment::Note(n) => {
                notes.push(FlattenedNote { time: current_time, is_rest: false, duration: n.duration });
                current_time += segment.duration().to_ratio();
            }
            crate::rhythm::RhythmSegment::Rest(duration) => {
                notes.push(FlattenedNote { time: current_time, is_rest: true, duration: *duration });
                current_time += segment.duration().to_ratio();
            }
            crate::rhythm::RhythmSegment::Tuplet { actual, normal, note_duration: _, rhythm, do_not_construct: _ } => {
                let flattened_tuplet_subrhythm_scaled =
                    flatten_rhythm(rhythm).into_iter().map(|note| FlattenedNote { time: note.time * Ratio::new(*normal, *actual), is_rest: note.is_rest, duration: note.duration });
                for flattened_subnote in flattened_tuplet_subrhythm_scaled {
                    notes.push(FlattenedNote { time: flattened_subnote.time + current_time, is_rest: flattened_subnote.is_rest, duration: flattened_subnote.duration })
                }
                current_time += segment.duration().to_ratio();
            }
        }
    }

    notes
}
