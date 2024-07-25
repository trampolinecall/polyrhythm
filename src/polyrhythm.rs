use crate::rhythm::{Duration, Rhythm};
use num_rational::Ratio;
use num_traits::ToPrimitive;

pub struct Polyrhythm {
    pub pulse: Option<Rhythm>,
    pub rhythms: Vec<RhythmLine>,
}

pub struct RhythmLine {
    pub original: Rhythm,
    pub approximations: Vec<Rhythm>,
}

pub fn score_error(original: &Rhythm, approx: &Rhythm) -> f64 {
    let o = flatten_rhythm(original);
    let a = flatten_rhythm(approx);

    o.into_iter().zip(a).map(|(o_ev, a_ev)| (o_ev.time - a_ev.time).to_f64().unwrap().abs()).sum()
}

pub struct Event {
    pub kind: EventKind,
    pub time: Ratio<u32>,
}
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
