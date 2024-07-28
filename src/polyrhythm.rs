use crate::{
    rhythm::{NoteDuration, Rhythm},
    time::Time,
    units::{Seconds, WholeNotes},
};
use num_rational::Ratio;
use num_traits::ConstZero;

pub struct Polyrhythm {
    pub tempo: (NoteDuration, u32),
    pub pulse: Option<Rhythm>,
    pub rhythms: Vec<RhythmLine>,
}

pub struct RhythmLine {
    pub original: Rhythm,
    pub approximations: Vec<Rhythm>,
}

pub fn score_error(tempo: (NoteDuration, u32), original: &Rhythm, approx: &Rhythm) -> Seconds {
    let o = flatten_rhythm(original);
    let a = flatten_rhythm(approx);

    let mut err = o
        .iter()
        .zip(a.iter())
        .map(|(o_ev, a_ev)| {
            let mut diff = (o_ev.time.0.to_seconds(tempo) - a_ev.time.0.to_seconds(tempo)).abs();
            if o_ev.kind != a_ev.kind {
                diff += Seconds(Ratio::from_integer(10));
                diff *= Ratio::from_integer(50);
            }
            diff
        })
        .sum();

    if o.len() != a.len() {
        err += Seconds(Ratio::from_integer(100));
        err *= Ratio::from_integer(100);
    }

    err
}

pub struct Event {
    pub kind: EventKind,
    pub time: Time<WholeNotes>,
}
#[derive(PartialEq, Eq)]
pub enum EventKind {
    Start,
    Stop,
}

pub fn flatten_rhythm(r: &Rhythm) -> Vec<Event> {
    let mut current_time = Time::ZERO;

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
                    events.push(Event { time: flattened_subnote.time * Ratio::new(*normal as i32, *actual as i32) + current_time, kind: flattened_subnote.kind })
                }
                current_time += segment.duration();
            }
        }
    }

    events
}
