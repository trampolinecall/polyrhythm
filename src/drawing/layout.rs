use crate::{
    drawing::coord::{pixel::STAFF_SPACE_PIXELS, Pixels, Point},
    polyrhythm::Polyrhythm,
    rhythm::Rhythm,
    time::{Duration, Time},
    units::WholeNotes,
};
use num_rational::Ratio;
use num_traits::ToPrimitive;

pub struct LayoutMetrics {
    canvas_width: Pixels,
    canvas_height: Pixels,

    error_text_x: Pixels,
    whole_note_width: Pixels,
    rhythm_height: Pixels,
}

const MIN_NOTE_SPACING: Pixels = Pixels(30.0);
const ERROR_DISPLAY_WIDTH: Pixels = Pixels(300.0);
const TEMPO_MARKING_HEIGHT: Pixels = Pixels(80.0);

impl LayoutMetrics {
    pub fn calculate(polyrhythm: &Polyrhythm) -> LayoutMetrics {
        let rhythm_height = STAFF_SPACE_PIXELS * 7.0;

        let all_rhythms = polyrhythm.pulse.iter().chain(polyrhythm.rhythms.iter().flat_map(|rhythm_line| std::iter::once(&rhythm_line.original).chain(rhythm_line.approximations.iter())));

        let whole_note_width = {
            fn flatten_rhythm_to_durations(r: &Rhythm) -> Vec<Duration<WholeNotes>> {
                let mut notes = Vec::new();

                for segment in &r.segments {
                    match segment {
                        crate::rhythm::RhythmSegment::Note(dur) => {
                            notes.push(dur.to_duration());
                        }
                        crate::rhythm::RhythmSegment::TiedNote(durs) => {
                            for dur in durs {
                                notes.push(dur.to_duration());
                            }
                        }
                        crate::rhythm::RhythmSegment::Rest(dur) => {
                            notes.push(dur.to_duration());
                        }
                        crate::rhythm::RhythmSegment::Tuplet { actual, normal, note_duration: _, rhythm, do_not_construct: _ } => {
                            for flattened_subdur in flatten_rhythm_to_durations(rhythm).into_iter() {
                                notes.push(flattened_subdur * Ratio::new(*normal as i32, *actual as i32))
                            }
                        }
                    }
                }

                notes
            }
            let all_durations = all_rhythms.clone().flat_map(flatten_rhythm_to_durations);
            let shortest_duration = all_durations.min().unwrap_or(Duration::WHOLE_NOTE); // just an arbitrary default duration in case there are no notes

            MIN_NOTE_SPACING / shortest_duration.0 .0.to_f64().unwrap()
        };

        let longest_rhythm = all_rhythms.clone().map(|rhy| rhy.duration()).max().unwrap_or(Duration::WHOLE_NOTE); // also another arbitrary default
        let canvas_width = whole_note_width * longest_rhythm.0 .0.to_f64().unwrap() + ERROR_DISPLAY_WIDTH;
        let canvas_height = rhythm_height * (all_rhythms.count() + 1) as f64 + TEMPO_MARKING_HEIGHT;

        let error_text_x = whole_note_width * longest_rhythm.0 .0.to_f64().unwrap();

        LayoutMetrics { canvas_width, canvas_height, error_text_x, whole_note_width, rhythm_height }
    }

    pub fn canvas_width(&self) -> Pixels {
        self.canvas_width
    }

    pub fn canvas_height(&self) -> Pixels {
        self.canvas_height
    }

    pub fn time_to_x(&self, time: Time<WholeNotes>) -> Pixels {
        self.whole_note_width * time.0 .0.to_f64().unwrap()
    }

    pub fn rhythm_index_to_y(&self, index: usize) -> Pixels {
        self.rhythm_height * (index + 1) as f64 + TEMPO_MARKING_HEIGHT
    }

    pub fn note_position(&self, time: Time<WholeNotes>, rhythm_index: usize) -> Point<Pixels> {
        Point::new(self.time_to_x(time), self.rhythm_index_to_y(rhythm_index))
    }

    pub fn error_text_pos(&self, rhythm_i: usize) -> Point<Pixels> {
        Point::new(self.error_text_x, self.rhythm_index_to_y(rhythm_i))
    }

    pub fn tempo_marking_pos(&self) -> Point<Pixels> {
        Point::new(Pixels(0.0), TEMPO_MARKING_HEIGHT)
    }
}
