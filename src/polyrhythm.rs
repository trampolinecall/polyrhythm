use crate::{
    rhythm::{NoteDuration, Rhythm}, time::Duration,
};
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
    let o = flatten_rhythm(original);
    let a = flatten_rhythm(approx);

    let mut err = o
        .iter()
        .zip(a.iter())
        .map(|(o_ev, a_ev)| {
            let mut diff = (o_ev.time.to_seconds(tempo) - a_ev.time.to_seconds(tempo)).to_f64().unwrap().abs();
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

pub struct Event {
    pub kind: EventKind,
    pub time: Duration,
}
#[derive(PartialEq, Eq)]
pub enum EventKind {
    Start,
    Stop,
}

pub fn flatten_rhythm(r: &Rhythm) -> Vec<Event> {
    let mut current_time = Duration::ZERO;

    let mut events = Vec::new();

    for segment in &r.segments {
        match segment {
            crate::rhythm::RhythmSegment::Note(_) => {
                events.push(Event { time: current_time, kind: EventKind::Start });
                current_time += segment.duration();
            }
            crate::rhythm::RhythmSegment::TiedNote(_) => {
                events.push(Event { time: current_time, kind: EventKind::Start });
                current_time += segment.duration();
            }
            crate::rhythm::RhythmSegment::Rest(_) => {
                events.push(Event { time: current_time, kind: EventKind::Stop });
                current_time += segment.duration();
            }
            crate::rhythm::RhythmSegment::Tuplet { actual, normal, note_duration: _, rhythm, do_not_construct: _ } => {
                for flattened_subnote in flatten_rhythm(rhythm).into_iter() {
                    events.push(Event { time: flattened_subnote.time * Ratio::new(*normal, *actual) + current_time, kind: flattened_subnote.kind })
                }
                current_time += segment.duration();
            }
        }
    }

    events
}
