use num_rational::Ratio;
use num_traits::cast::ToPrimitive;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::{
    drawing::coord::{pixel::STAFF_SPACE_PIXELS, Pixels, Point, StaffSpaces},
    polyrhythm::Polyrhythm,
    rhythm::{NoteDuration, Rhythm},
};

pub use drawing::Font;

mod coord;
#[allow(clippy::module_inception)]
mod drawing;

// TODO: replace canvas with svg?

// TODO: do this better (scale so that the shortest note is a comfortable distance from the next?)
const WHOLE_NOTE_WIDTH: Pixels = Pixels(500.0);
// TODO: decide on a better value for this (make this vary between each rhythm based on the highest number of flags on the shortest note?)
const RHYTHM_HEIGHT: Pixels = Pixels(100.0);
const STAFF_HEIGHT: Pixels = Pixels(STAFF_SPACE_PIXELS.0 * 4.0);

const DEFAULT_STAFF_LINE_THICKNESS: StaffSpaces = StaffSpaces(1.0 / 8.0);
const DEFAULT_STEM_THICKNESS: StaffSpaces = StaffSpaces(3.0 / 25.0);

pub fn draw(canvas: &HtmlCanvasElement, font: &Font, polyrhythm: &Polyrhythm) {
    let ctx: CanvasRenderingContext2d = canvas.get_context("2d").expect("could not get canvas context").unwrap().dyn_into().expect("2d canvas context should be CanvasRenderingContext2d");

    // TODO: adjust this
    canvas.set_width(1000);
    canvas.set_height(500);
    ctx.clear_rect(0.0, 0.0, 1000.0, 500.0);

    for (i, rhythm) in polyrhythm.rhythms.iter().enumerate() {
        let y = RHYTHM_HEIGHT * (i as f64) + RHYTHM_HEIGHT / 2.0;
        draw_staff_line(&ctx, font, y);

        for note in flatten_rhythm(rhythm) {
            let x = WHOLE_NOTE_WIDTH * note.time.to_f64().unwrap();
            let note_pos = Point::new(x, y);
            if note.is_rest {
                draw_rest(&ctx, font, note.duration, note_pos);
            } else {
                draw_note(&ctx, font, note.duration, note_pos);
            }
        }
    }
}

fn draw_staff_line(ctx: &CanvasRenderingContext2d, font: &Font, y: Pixels) {
    drawing::line(ctx, Point::new(Pixels(0.0), y), Point::new(Pixels(1000.0), y), "black", font.metadata.engraving_defaults.staff_line_thickness.unwrap_or(DEFAULT_STAFF_LINE_THICKNESS).into());
}

fn draw_rest(ctx: &CanvasRenderingContext2d, font: &Font, duration: NoteDuration, pos: Point<Pixels>) {
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
    drawing::draw_glyph(ctx, font, glyph, pos);
}

fn draw_note(ctx: &CanvasRenderingContext2d, font: &Font, duration: NoteDuration, pos: Point<Pixels>) {
    let notehead = if duration == NoteDuration::WHOLE {
        smufl::Glyph::NoteheadWhole
    } else if duration == NoteDuration::HALF {
        smufl::Glyph::NoteheadHalf
    } else {
        smufl::Glyph::NoteheadBlack
    };

    let notehead_anchors = font.metadata.anchors.get(notehead).unwrap();
    let notehead_origin: Point<_> = notehead_anchors.notehead_origin.map(Point::from).unwrap_or(Point::new(StaffSpaces(0.0), StaffSpaces(0.0))).into(); // TODO: make Point::ZERO work for staff spaces too

    drawing::draw_glyph(ctx, font, notehead, pos - notehead_origin);

    if let Some(stem_start_offset) = notehead_anchors.stem_up_se {
        let stemstart_offset = Point::<Pixels>::from(Point::<StaffSpaces>::from(stem_start_offset));
        const STEM_LENGTH: StaffSpaces = StaffSpaces(3.5);
        // draw the stem
        let stem_up_extension = notehead_anchors.stem_up_nw.map(Point::<StaffSpaces>::from).map(Point::<Pixels>::from).unwrap_or(Point::ZERO);

        drawing::line(
            ctx,
            pos + stemstart_offset,
            pos + stemstart_offset - Point::new(Pixels(0.0), STEM_LENGTH.into()) + stem_up_extension,
            "black",
            font.metadata.engraving_defaults.stem_thickness.unwrap_or(DEFAULT_STEM_THICKNESS).into(),
        );

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
            drawing::fill_text(ctx, font, &flag_glyph.codepoint().to_string(), pos - notehead_origin + stemstart_offset - Point::new(Pixels(0.0), STEM_LENGTH.into()));
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
