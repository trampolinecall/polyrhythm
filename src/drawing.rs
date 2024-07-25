use num_rational::Ratio;
use num_traits::cast::ToPrimitive;
use smufl::{Coord, StaffSpaces};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::{
    drawing::pixels::Pixels,
    polyrhythm::Polyrhythm,
    rhythm::{NoteDuration, Rhythm},
};

mod pixels;

lazy_static::lazy_static! {
    static ref FONT: String = {
        let x = format!("{}px Bravura", STAFF_HEIGHT_PIXELS.0);
        x
    };
    static ref FONT_METADATA: smufl::Metadata = {
        // TODO: do this better
        let metadata_contents = include_str!("../site/fonts/bravura/redist/bravura_metadata.json");
        smufl::Metadata::from_reader(metadata_contents.as_bytes()).expect("could not parse metadata for font")
    };
}

// TODO: do this better (scale so that the shortest note is a comfortable distance from the next?)
const WHOLE_NOTE_WIDTH: Pixels = Pixels(500.0);
// TODO: decide on a better value for this (make this vary between each rhythm based on the highest number of flags on the shortest note?)
const RHYTHM_HEIGHT: Pixels = Pixels(100.0);
const STAFF_SPACE_PIXELS: Pixels = Pixels(10.0);
const STAFF_HEIGHT_PIXELS: Pixels = Pixels(STAFF_SPACE_PIXELS.0 * 4.0);

pub fn draw(canvas: &HtmlCanvasElement, polyrhythm: &Polyrhythm) {
    let ctx: CanvasRenderingContext2d = canvas.get_context("2d").expect("could not get canvas context").unwrap().dyn_into().expect("2d canvas context should be CanvasRenderingContext2d");

    // TODO: adjust this
    canvas.set_width(1000);
    canvas.set_height(500);
    ctx.clear_rect(0.0, 0.0, 1000.0, 500.0);

    for (i, rhythm) in polyrhythm.rhythms.iter().enumerate() {
        let y = RHYTHM_HEIGHT * (i as f64) + RHYTHM_HEIGHT / 2.0;
        ctx.begin_path();
        pixels::move_to(&ctx, Pixels(0.0), y);
        pixels::line_to(&ctx, Pixels(0.0), y);
        ctx.set_stroke_style(&"black".into());
        ctx.stroke();

        for note in flatten_rhythm(rhythm) {
            let x = WHOLE_NOTE_WIDTH * note.time.to_f64().unwrap();
            if note.is_rest {
                draw_rest(&ctx, note.duration, x, y);
            } else {
                draw_note(&ctx, note.duration, x, y);
            }
        }
    }
}

fn draw_rest(ctx: &CanvasRenderingContext2d, duration: NoteDuration, x: Pixels, y: Pixels) {
    ctx.set_font(&FONT);
    let glyph = if duration == NoteDuration::WHOLE {
        smufl::Glyph::RestWhole
    } else if duration == NoteDuration::HALF {
        smufl::Glyph::RestHalf
    } else if duration == NoteDuration::QUARTER {
        smufl::Glyph::RestQuarter
    } else if duration == NoteDuration::EIGTH {
        smufl::Glyph::Rest8th
    } else if duration == NoteDuration::SIXTEENTH {
        smufl::Glyph::Rest16th
    } else if duration == NoteDuration::THIRTYSECOND {
        smufl::Glyph::Rest32nd
    } else if duration == NoteDuration::SIXTYFOURTH {
        smufl::Glyph::Rest64th
    } else if duration == NoteDuration::ND128 {
        smufl::Glyph::Rest128th
    } else if duration == NoteDuration::ND256 {
        smufl::Glyph::Rest256th
    } else if duration == NoteDuration::ND512 {
        smufl::Glyph::Rest512th
    } else if duration == NoteDuration::ND1024 {
        smufl::Glyph::Rest1024th
    } else {
        panic!("no glyph for rest")
    };
    web_sys::console::log_1(&(FONT.clone()).into());
    pixels::fill_text(&ctx, &glyph.codepoint().to_string(), x, y);
}

fn draw_note(ctx: &CanvasRenderingContext2d, duration: NoteDuration, x: Pixels, y: Pixels) {
    ctx.set_font(&FONT);

    let notehead = if duration == NoteDuration::WHOLE {
        smufl::Glyph::NoteheadWhole
    } else if duration == NoteDuration::HALF {
        smufl::Glyph::NoteheadHalf
    } else {
        smufl::Glyph::NoteheadBlack
    };

    let notehead_anchors = FONT_METADATA.anchors.get(notehead).unwrap();
    let notehead_origin_coord = notehead_anchors.notehead_origin;
    let notehead_origin_x = notehead_origin_coord.map(|c| c.x()).unwrap_or(StaffSpaces(0.0));
    let notehead_origin_y = notehead_origin_coord.map(|c| c.y()).unwrap_or(StaffSpaces(0.0));

    pixels::fill_text(ctx, &notehead.codepoint().to_string(), x - notehead_origin_x.into(), y - notehead_origin_y.into());

    if let Some(stemstart_offset) = notehead_anchors.stem_up_se {
        // draw the stem
        let stem_up_extension_coord = notehead_anchors.stem_up_nw;
        let stem_up_extension_x = stem_up_extension_coord.map(|c| c.x()).unwrap_or(StaffSpaces(0.0));
        let stem_up_extension_y = stem_up_extension_coord.map(|c| c.y()).unwrap_or(StaffSpaces(0.0));

        ctx.begin_path();
        pixels::move_to(ctx, x + stemstart_offset.x().into(), y + stemstart_offset.y().into());
        pixels::line_to(ctx, x + stemstart_offset.x().into() + stem_up_extension_x.into(), y + stemstart_offset.y().into() - StaffSpaces(3.5).into() + stem_up_extension_y.into());
        ctx.stroke();

        let flag_glyph = if duration == NoteDuration::EIGTH {
            Some(smufl::Glyph::Flag8thUp)
        } else if duration == NoteDuration::SIXTEENTH {
            Some(smufl::Glyph::Flag16thUp)
        } else if duration == NoteDuration::THIRTYSECOND {
            Some(smufl::Glyph::Flag32ndUp)
        } else if duration == NoteDuration::SIXTYFOURTH {
            Some(smufl::Glyph::Flag64thUp)
        } else if duration == NoteDuration::ND128 {
            Some(smufl::Glyph::Flag128thUp)
        } else if duration == NoteDuration::ND256 {
            Some(smufl::Glyph::Flag256thUp)
        } else if duration == NoteDuration::ND512 {
            Some(smufl::Glyph::Flag512thUp)
        } else if duration == NoteDuration::ND1024 {
            Some(smufl::Glyph::Flag1024thUp)
        } else {
            None
        };

        if let Some(flag_glyph) = flag_glyph {
            pixels::fill_text(ctx, &flag_glyph.codepoint().to_string(), x - notehead_origin_x.into() + stemstart_offset.x().into(), y - notehead_origin_y.into() - StaffSpaces(3.5).into());
        }
    }
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
