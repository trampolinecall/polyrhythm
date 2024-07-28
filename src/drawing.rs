use num_rational::Ratio;
use num_traits::{ConstZero, ToPrimitive};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::{
    drawing::coord::{pixel::STAFF_SPACE_PIXELS, Pixels, Point, StaffSpaces},
    polyrhythm::{self, Polyrhythm},
    rhythm::{NoteDuration, NoteDurationKind, Rhythm},
    time::Time,
    units::WholeNotes,
};

pub use drawing::Font;

mod coord;
#[allow(clippy::module_inception)]
mod drawing;
mod layout;

const STAFF_HEIGHT: Pixels = Pixels(STAFF_SPACE_PIXELS.0 * 4.0);

const DEFAULT_STAFF_LINE_THICKNESS: StaffSpaces = StaffSpaces(1.0 / 8.0);
const DEFAULT_SLUR_MIDPOINT_THICKNESS: StaffSpaces = StaffSpaces(0.22);
const DEFAULT_STEM_THICKNESS: StaffSpaces = StaffSpaces(3.0 / 25.0);
const DEFAULT_BEAT_LINE_THICKNESS: StaffSpaces = StaffSpaces(1.0 / 25.0);
const DEFAULT_CORRESPONDENCE_LINE_THICKNESS: StaffSpaces = StaffSpaces(2.0 / 25.0);

pub fn draw(canvas: &HtmlCanvasElement, font: &Font, polyrhythm: &Polyrhythm) {
    let ctx: CanvasRenderingContext2d = canvas.get_context("2d").expect("could not get canvas context").unwrap().dyn_into().expect("2d canvas context should be CanvasRenderingContext2d");

    let layout_metrics = layout::LayoutMetrics::calculate(polyrhythm);

    drawing::set_canvas_size_and_clear(canvas, &ctx, layout_metrics.canvas_width(), layout_metrics.canvas_height());

    draw_tempo(&ctx, &layout_metrics, font, polyrhythm.tempo);

    let mut rhythm_i = 0;
    if let Some(pulse) = &polyrhythm.pulse {
        draw_rhythm(&ctx, &layout_metrics, font, rhythm_i, pulse);
        draw_pulse(&ctx, &layout_metrics, pulse);
        rhythm_i += 1;
    }

    for line in polyrhythm.rhythms.iter() {
        let original_flattened = polyrhythm::flatten_rhythm(&line.original);
        let original_i = rhythm_i;

        draw_rhythm(&ctx, &layout_metrics, font, rhythm_i, &line.original);

        rhythm_i += 1;

        for approx in &line.approximations {
            draw_rhythm(&ctx, &layout_metrics, font, rhythm_i, approx);

            let approx_error = polyrhythm::score_error(polyrhythm.tempo, &line.original, approx);
            drawing::fill_text(&ctx, font, &format!("error: {}s", approx_error.0.to_f64().unwrap()), layout_metrics.error_text_pos(rhythm_i));

            let approx_flattened = polyrhythm::flatten_rhythm(approx);

            for (original_ev, approx_ev) in original_flattened.iter().zip(approx_flattened) {
                drawing::line(
                    &ctx,
                    layout_metrics.note_position(original_ev.time, original_i),
                    layout_metrics.note_position(approx_ev.time, rhythm_i),
                    "grey",
                    DEFAULT_CORRESPONDENCE_LINE_THICKNESS.into(),
                );
            }

            rhythm_i += 1;
        }
    }
}

fn draw_tempo(ctx: &CanvasRenderingContext2d, layout_metrics: &layout::LayoutMetrics, font: &Font, (dur, bpm): (NoteDuration, u32)) {
    let mut dur_sym = match dur.kind {
        NoteDurationKind::Whole => smufl::Glyph::MetNoteWhole,
        NoteDurationKind::Half => smufl::Glyph::MetNoteHalfUp,
        NoteDurationKind::Quarter => smufl::Glyph::MetNoteQuarterUp,
        NoteDurationKind::Eigth => smufl::Glyph::MetNote8thUp,
        NoteDurationKind::Sixteenth => smufl::Glyph::MetNote16thUp,
        NoteDurationKind::Nd32 => smufl::Glyph::MetNote32ndUp,
        NoteDurationKind::Nd64 => smufl::Glyph::MetNote64thUp,
        NoteDurationKind::Nd128 => smufl::Glyph::MetNote128thUp,
        NoteDurationKind::Nd256 => smufl::Glyph::MetNote256thUp,
        NoteDurationKind::Nd512 => smufl::Glyph::MetNote512thUp,
        NoteDurationKind::Nd1024 => smufl::Glyph::MetNote1024thUp,
    }
    .codepoint()
    .to_string();

    if dur.dotted {
        dur_sym.push(smufl::Glyph::MetAugmentationDot.codepoint())
    }

    drawing::fill_text(ctx, font, &format!("{} = {}", dur_sym, bpm), layout_metrics.tempo_marking_pos());
}

fn draw_rhythm(ctx: &CanvasRenderingContext2d, layout_metrics: &layout::LayoutMetrics, font: &Font, rhythm_index: usize, rhythm: &Rhythm) {
    draw_staff_line(ctx, layout_metrics, font, rhythm_index);
    for note in flatten_rhythm(rhythm) {
        let note_pos = layout_metrics.note_position(note.time, rhythm_index);
        if note.is_rest {
            draw_rest(ctx, font, note.duration, note_pos);
        } else {
            draw_note(ctx, font, note.duration, note.tied_to_next, note_pos);
        }
    }
}

fn draw_pulse(ctx: &CanvasRenderingContext2d, layout_metrics: &layout::LayoutMetrics, pulse: &Rhythm) {
    for note in flatten_rhythm(pulse) {
        let x = layout_metrics.time_to_x(note.time);
        drawing::line(ctx, Point::new(x, Pixels(0.0)), Point::new(x, Pixels(500.0)), "grey", DEFAULT_BEAT_LINE_THICKNESS.into())
    }
}

fn draw_staff_line(ctx: &CanvasRenderingContext2d, layout_metrics: &layout::LayoutMetrics, font: &Font, rhythm_index: usize) {
    let y = layout_metrics.rhythm_index_to_y(rhythm_index);
    drawing::line(
        ctx,
        Point::new(Pixels(0.0), y),
        Point::new(layout_metrics.canvas_width(), y),
        "black",
        font.metadata.engraving_defaults.staff_line_thickness.unwrap_or(DEFAULT_STAFF_LINE_THICKNESS).into(),
    );
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

    if duration.dotted {
        drawing::draw_glyph(ctx, font, smufl::Glyph::AugmentationDot, pos + Point::new(StaffSpaces(1.0), StaffSpaces(-0.5)).into());
        // TODO: adjust x position
    }
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
        draw_tie(ctx, font, pos + Point::new(StaffSpaces(0.0), StaffSpaces(0.1)).into(), pos + Point::new(StaffSpaces(3.0), StaffSpaces(0.1)).into());
        // TODO: adjust offset from notehead
        // TODO: tie to the next note, not to a hardcoded offset
    }

    if duration.dotted {
        drawing::draw_glyph(ctx, font, smufl::Glyph::AugmentationDot, pos + Point::new(StaffSpaces(1.0), StaffSpaces(-0.5)).into());
        // TODO: adjust x position
    }
}

fn draw_tie(ctx: &CanvasRenderingContext2d, font: &Font, start: Point<Pixels>, end: Point<Pixels>) {
    let dx = end.x - start.x;

    let cp1 = Point::new(start.x + dx / 3.0, start.y + Pixels(25.0));
    let cp2 = Point::new(end.x - dx / 3.0, end.y + Pixels(25.0));

    // TODO: do varying line width
    drawing::bezier(ctx, start, cp1, cp2, end, "black", font.metadata.engraving_defaults.slur_midpoint_thickness.unwrap_or(DEFAULT_SLUR_MIDPOINT_THICKNESS).into());
}

struct FlattenedNote {
    time: Time<WholeNotes>,
    is_rest: bool,
    duration: NoteDuration,
    tied_to_next: bool,
}
fn flatten_rhythm(r: &Rhythm) -> Vec<FlattenedNote> {
    let mut current_time = Time::ZERO;

    let mut notes = Vec::new();

    for segment in &r.segments {
        match segment {
            crate::rhythm::RhythmSegment::Note(dur) => {
                notes.push(FlattenedNote { time: current_time, is_rest: false, duration: *dur, tied_to_next: false });
                current_time += segment.duration();
            }
            crate::rhythm::RhythmSegment::TiedNote(durs) => {
                let (last, firsts) = durs.split_last().expect("cannot have 0 notes in tied notes");
                for dur in firsts {
                    notes.push(FlattenedNote { time: current_time, is_rest: false, duration: *dur, tied_to_next: true });
                    current_time += dur.to_duration();
                }
                notes.push(FlattenedNote { time: current_time, is_rest: false, duration: *last, tied_to_next: false });
                current_time += last.to_duration();
            }
            crate::rhythm::RhythmSegment::Rest(dur) => {
                notes.push(FlattenedNote { time: current_time, is_rest: true, duration: *dur, tied_to_next: false });
                current_time += segment.duration();
            }
            crate::rhythm::RhythmSegment::Tuplet { actual, normal, note_duration: _, rhythm, do_not_construct: _ } => {
                for flattened_subnote in flatten_rhythm(rhythm).into_iter() {
                    notes.push(FlattenedNote {
                        time: flattened_subnote.time * Ratio::new(*normal as i32, *actual as i32) + current_time,
                        is_rest: flattened_subnote.is_rest,
                        duration: flattened_subnote.duration,
                        tied_to_next: flattened_subnote.tied_to_next,
                    })
                }
                current_time += segment.duration();
            }
        }
    }

    notes
}
