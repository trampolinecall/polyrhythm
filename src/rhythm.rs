use num_rational::Ratio;

use crate::time::Duration;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct NoteDuration {
    pub kind: NoteDurationKind,
    pub dotted: bool,
}
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NoteDurationKind {
    Whole,
    Half,
    Quarter,
    Eigth,
    Sixteenth,
    Nd32,
    Nd64,
    Nd128,
    Nd256,
    Nd512,
    Nd1024,
}

impl NoteDuration {
    pub fn to_duration(self) -> Duration {
        Duration::WHOLE_NOTE * self.kind.to_ratio() * if self.dotted { Ratio::new(3, 2) } else { Ratio::new(1, 1) }
    }
}
impl NoteDurationKind {
    pub fn to_ratio(self) -> Ratio<u32> {
        match self {
            NoteDurationKind::Whole => Ratio::new(1, 1),
            NoteDurationKind::Half => Ratio::new(1, 2),
            NoteDurationKind::Quarter => Ratio::new(1, 4),
            NoteDurationKind::Eigth => Ratio::new(1, 8),
            NoteDurationKind::Sixteenth => Ratio::new(1, 16),
            NoteDurationKind::Nd32 => Ratio::new(1, 32),
            NoteDurationKind::Nd64 => Ratio::new(1, 64),
            NoteDurationKind::Nd128 => Ratio::new(1, 128),
            NoteDurationKind::Nd256 => Ratio::new(1, 256),
            NoteDurationKind::Nd512 => Ratio::new(1, 512),
            NoteDurationKind::Nd1024 => Ratio::new(1, 1024),
        }
    }

    pub fn from_number(number: u32) -> Option<NoteDurationKind> {
        match number {
            1 => Some(NoteDurationKind::Whole),
            2 => Some(NoteDurationKind::Half),
            4 => Some(NoteDurationKind::Quarter),
            8 => Some(NoteDurationKind::Eigth),
            16 => Some(NoteDurationKind::Sixteenth),
            32 => Some(NoteDurationKind::Nd32),
            64 => Some(NoteDurationKind::Nd64),
            128 => Some(NoteDurationKind::Nd128),
            256 => Some(NoteDurationKind::Nd256),
            512 => Some(NoteDurationKind::Nd512),
            1024 => Some(NoteDurationKind::Nd1024),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Rhythm {
    pub segments: Vec<RhythmSegment>,
}
#[derive(Clone)]
pub struct DoNotConstruct(());
#[derive(Clone)]
pub enum RhythmSegment {
    Note(NoteDuration),
    TiedNote(Vec<NoteDuration>),
    Rest(NoteDuration),
    Tuplet {
        // TODO: incomplete tuplets
        // a tuplet where `actual` number of notes are found in the space that there would normally be `normal` notes
        // duration specifies the duration of the notes that `acutal` and `normal` refer to
        // for example, a triplet where 3 eigth notes fit into a space where there would be normally 2 would become Tuplet { actual: 3, normal: 2, duration: NoteDuration::EIGTH, .. }
        actual: u32,
        normal: u32,
        note_duration: NoteDuration,
        rhythm: Box<Rhythm>,
        do_not_construct: DoNotConstruct,
    },
}

pub struct TupletInnerDurationMismatch {
    pub actual: Duration,
    pub expected: Duration,
}

impl RhythmSegment {
    pub fn new_tuplet(actual: u32, normal: u32, note_duration: NoteDuration, rhythm: Rhythm) -> Result<RhythmSegment, TupletInnerDurationMismatch> {
        let actual_inner_duration = rhythm.duration();
        let expected_inner_duration = note_duration.to_duration() * Ratio::from_integer(actual);
        if actual_inner_duration == expected_inner_duration {
            Ok(RhythmSegment::Tuplet { actual, normal, note_duration, rhythm: Box::new(rhythm), do_not_construct: DoNotConstruct(()) })
        } else {
            Err(TupletInnerDurationMismatch { actual: actual_inner_duration, expected: expected_inner_duration })
        }
    }

    pub fn duration(&self) -> Duration {
        match self {
            RhythmSegment::Note(dur) => dur.to_duration(),
            RhythmSegment::TiedNote(durs) => durs.iter().copied().map(NoteDuration::to_duration).sum(),
            RhythmSegment::Rest(dur) => dur.to_duration(),
            RhythmSegment::Tuplet { actual: _, normal, note_duration, rhythm: _, do_not_construct: _ } => note_duration.to_duration() * Ratio::from_integer(*normal),
        }
    }
}

impl Rhythm {
    pub fn duration(&self) -> Duration {
        self.segments.iter().map(|s| s.duration()).sum()
    }
}
