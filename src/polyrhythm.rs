use crate::rhythm::{NoteDuration, Rhythm};
use num_rational::Ratio;
use num_traits::ToPrimitive;

pub struct Polyrhythm {
    pub tempo: (NoteDuration, u32),
    pub pulse: Option<Rhythm>,
    pub rhythms: Vec<RhythmLine>,
}

pub struct RhythmLine {
    pub original: Rhythm,
    pub approximations: Vec<Rhythm>,
}

pub fn score_error(tempo: (NoteDuration, u32), original: &Rhythm, approx: &Rhythm) -> f64 {
    let duration_multiplier = calculate_duration_multiplier(tempo);

    let o = flatten_rhythm(original);
    let a = flatten_rhythm(approx);

    let mut err = o
        .iter()
        .zip(a.iter())
        .map(|(o_ev, a_ev)| {
            let mut diff = (o_ev.time - a_ev.time).to_f64().unwrap().abs() * duration_multiplier;
            if o_ev.kind != a_ev.kind {
                diff += 10.0;
                diff *= 50.0;
            }
            diff
        })
        .sum();

    if o.len() != a.len() {
        err += 100.0;
        err *= 100.0;
    }

    err
}

fn calculate_duration_multiplier((dur, bpm): (NoteDuration, u32)) -> f64 {
    let whole_notes_per_minute = dur.to_duration().to_ratio() * Ratio::from_integer(bpm);
    let whole_notes_per_second = whole_notes_per_minute / Ratio::from_integer(60);
    let seconds_per_whole_note = Ratio::from_integer(1) / whole_notes_per_second;
    seconds_per_whole_note.to_f64().unwrap()
}

pub struct Event {
    pub kind: EventKind,
    pub time: Ratio<u32>,
}
#[derive(PartialEq, Eq)]
pub enum EventKind {
    Start,
    Stop,
}

pub fn flatten_rhythm(r: &Rhythm) -> Vec<Event> {
    let mut current_time = Ratio::ZERO;

    let mut events = Vec::new();

    for segment in &r.segments {
        match segment {
            crate::rhythm::RhythmSegment::Note(_) => {
                events.push(Event { time: current_time, kind: EventKind::Start });
                current_time += segment.duration().to_ratio();
            }
            crate::rhythm::RhythmSegment::TiedNote(_) => {
                events.push(Event { time: current_time, kind: EventKind::Start });
                current_time += segment.duration().to_ratio();
            }
            crate::rhythm::RhythmSegment::Rest(_) => {
                events.push(Event { time: current_time, kind: EventKind::Stop });
                current_time += segment.duration().to_ratio();
            }
            crate::rhythm::RhythmSegment::Tuplet { actual, normal, note_duration: _, rhythm, do_not_construct: _ } => {
                for flattened_subnote in flatten_rhythm(rhythm).into_iter() {
                    events.push(Event { time: flattened_subnote.time * Ratio::new(*normal, *actual) + current_time, kind: flattened_subnote.kind })
                }
                current_time += segment.duration().to_ratio();
            }
        }
    }

    events
}
