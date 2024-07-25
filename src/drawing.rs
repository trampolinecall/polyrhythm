use num_rational::Ratio;
use num_traits::cast::ToPrimitive;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::{
    drawing::coord::{pixel::STAFF_SPACE_PIXELS, Pixels, Point, StaffSpaces},
    polyrhythm::Polyrhythm,
    rhythm::{NoteDuration, NoteDurationKind, Rhythm},
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
const DEFAULT_SLUR_MIDPOINT_THICKNESS: StaffSpaces = StaffSpaces(0.22);
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
                draw_note(&ctx, font, note.duration, note.tied_to_next, note_pos);
            }
        }
    }
}

fn draw_staff_line(ctx: &CanvasRenderingContext2d, font: &Font, y: Pixels) {
    drawing::line(ctx, Point::new(Pixels(0.0), y), Point::new(Pixels(1000.0), y), "black", font.metadata.engraving_defaults.staff_line_thickness.unwrap_or(DEFAULT_STAFF_LINE_THICKNESS).into());
}

fn draw_rest(ctx: &CanvasRenderingContext2d, font: &Font, duration: NoteDuration, pos: Point<Pixels>) {
    let glyph = match duration.kind {
        NoteDurationKind::Whole => smufl::Glyph::RestWhole,
        NoteDurationKind::Half => smufl::Glyph::RestHalf,
        NoteDurationKind::Quarter => smufl::Glyph::RestQuarter,
        NoteDurationKind::Eigth => smufl::Glyph::Rest8th,
        NoteDurationKind::Sixteenth => smufl::Glyph::Rest16th,
        NoteDurationKind::Nd32 => smufl::Glyph::Rest32nd,
        NoteDurationKind::Nd64 => smufl::Glyph::Rest64th,
        NoteDurationKind::Nd128 => smufl::Glyph::Rest128th,
        NoteDurationKind::Nd256 => smufl::Glyph::Rest256th,
        NoteDurationKind::Nd512 => smufl::Glyph::Rest512th,
        NoteDurationKind::Nd1024 => smufl::Glyph::Rest1024th,
    };
    drawing::draw_glyph(ctx, font, glyph, pos);
}

fn draw_note(ctx: &CanvasRenderingContext2d, font: &Font, duration: NoteDuration, tied_to_next: bool, pos: Point<Pixels>) {
    let notehead = match duration.kind {
        NoteDurationKind::Whole => smufl::Glyph::NoteheadWhole,
        NoteDurationKind::Half => smufl::Glyph::NoteheadHalf,
        _ => smufl::Glyph::NoteheadBlack,
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

        let flag_glyph = match duration.kind {
            NoteDurationKind::Whole => None,
            NoteDurationKind::Half => None,
            NoteDurationKind::Quarter => None,
            NoteDurationKind::Eigth => Some(smufl::Glyph::Flag8thUp),
            NoteDurationKind::Sixteenth => Some(smufl::Glyph::Flag16thUp),
            NoteDurationKind::Nd32 => Some(smufl::Glyph::Flag32ndUp),
            NoteDurationKind::Nd64 => Some(smufl::Glyph::Flag64thUp),
            NoteDurationKind::Nd128 => Some(smufl::Glyph::Flag128thUp),
            NoteDurationKind::Nd256 => Some(smufl::Glyph::Flag256thUp),
            NoteDurationKind::Nd512 => Some(smufl::Glyph::Flag512thUp),
            NoteDurationKind::Nd1024 => Some(smufl::Glyph::Flag1024thUp),
        };

        if let Some(flag_glyph) = flag_glyph {
            drawing::fill_text(ctx, font, &flag_glyph.codepoint().to_string(), pos - notehead_origin + stemstart_offset - Point::new(Pixels(0.0), STEM_LENGTH.into()));
        }
    }

    if tied_to_next {
        draw_slur(ctx, font, pos + Point::new(StaffSpaces(0.0), StaffSpaces(0.1)).into(), pos + Point::new(StaffSpaces(3.0), StaffSpaces(0.1)).into()); // TODO: tie to the next note, not to a hardcoded offset
    }
}

fn draw_slur(ctx: &CanvasRenderingContext2d, font: &Font, start: Point<Pixels>, end: Point<Pixels>) {
    let dx = end.x - start.x;

    let cp1 = Point::new(start.x + dx / 3.0, start.y + Pixels(25.0));
    let cp2 = Point::new(end.x - dx / 3.0, end.y + Pixels(25.0));

    // TODO: do varying line width
    drawing::bezier(ctx, start, cp1, cp2, end, "black", font.metadata.engraving_defaults.slur_midpoint_thickness.unwrap_or(DEFAULT_SLUR_MIDPOINT_THICKNESS).into());
}

struct FlattenedNote {
    time: Ratio<u32>,
    is_rest: bool,
    duration: NoteDuration,
    tied_to_next: bool,
}
fn flatten_rhythm(r: &Rhythm) -> Vec<FlattenedNote> {
    let mut current_time = Ratio::ZERO;

    let mut notes = Vec::new();

    for segment in &r.segments {
        match segment {
            crate::rhythm::RhythmSegment::Note(dur) => {
                notes.push(FlattenedNote { time: current_time, is_rest: false, duration: *dur, tied_to_next: false });
                current_time += segment.duration().to_ratio();
            }
            crate::rhythm::RhythmSegment::TiedNote(durs) => {
                let (last, firsts) = durs.split_last().expect("cannot have 0 notes in tied notes");
                for dur in firsts {
                    notes.push(FlattenedNote { time: current_time, is_rest: false, duration: *dur, tied_to_next: true });
                    current_time += dur.to_duration().to_ratio();
                }
                notes.push(FlattenedNote { time: current_time, is_rest: false, duration: *last, tied_to_next: false });
                current_time += last.to_duration().to_ratio();
            }
            crate::rhythm::RhythmSegment::Rest(dur) => {
                notes.push(FlattenedNote { time: current_time, is_rest: true, duration: *dur, tied_to_next: false });
                current_time += segment.duration().to_ratio();
            }
            crate::rhythm::RhythmSegment::Tuplet { actual, normal, note_duration: _, rhythm, do_not_construct: _ } => {
                for flattened_subnote in flatten_rhythm(rhythm).into_iter() {
                    notes.push(FlattenedNote {
                        time: flattened_subnote.time * Ratio::new(*normal, *actual) + current_time,
                        is_rest: flattened_subnote.is_rest,
                        duration: flattened_subnote.duration,
                        tied_to_next: flattened_subnote.tied_to_next,
                    })
                }
                current_time += segment.duration().to_ratio();
            }
        }
    }

    notes
}
