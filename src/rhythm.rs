use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};

use num_rational::Ratio;

// the u32 is the exponent that makes the note name
// so for example, whole   notes have a duration of NoteDuration(0) because 2^0 = 1
// so for example, half    notes have a duration of NoteDuration(1) because 2^1 = 2
// so for example, quarter notes have a duration of NoteDuration(2) because 2^2 = 4
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct NoteDuration(u32);

impl NoteDuration {
    const WHOLE: NoteDuration = NoteDuration(0);
    const HALF: NoteDuration = NoteDuration(1);
    const QUARTER: NoteDuration = NoteDuration(2);
    const EIGTH: NoteDuration = NoteDuration(3);
    const SIXTEENTH: NoteDuration = NoteDuration(4);
    const THIRTYSECOND: NoteDuration = NoteDuration(5);
    const SIXTYFOURTH: NoteDuration = NoteDuration(6);
    const ONEHUNDREDTWENTYSECOND: NoteDuration = NoteDuration(7);

    pub fn to_duration(self) -> Duration {
        Duration::WHOLE_NOTE * Ratio::new(1, 2u32.pow(self.0))
    }

    pub fn from_number(number: u32) -> Option<NoteDuration> {
        if number.is_power_of_two() {
            number.checked_ilog2().map(NoteDuration)
        } else {
            None
        }
    }
}

// durations are expressed in terms of whole notes
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Duration(Ratio<u32>);

impl Add<Duration> for Duration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Duration {
        Duration(self.0 + rhs.0)
    }
}
impl Sub<Duration> for Duration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Duration {
        Duration(self.0 - rhs.0)
    }
}
impl Mul<Ratio<u32>> for Duration {
    type Output = Duration;

    fn mul(self, rhs: Ratio<u32>) -> Duration {
        Duration(self.0 * rhs)
    }
}
impl Div<Ratio<u32>> for Duration {
    type Output = Duration;

    fn div(self, rhs: Ratio<u32>) -> Duration {
        Duration(self.0 / rhs)
    }
}
impl Sum for Duration {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Duration::ZERO, |a, b| a + b)
    }
}
impl Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0) // TODO: do this better
    }
}

impl Duration {
    // it is safe to use new_raw here because we know that the denominators are not 0
    const ZERO: Duration = Duration(Ratio::new_raw(0, 1));
    const WHOLE_NOTE: Duration = Duration(Ratio::new_raw(1, 1));
    const HALF_NOTE: Duration = Duration(Ratio::new_raw(1, 2));
    const QUARTER_NOTE: Duration = Duration(Ratio::new_raw(1, 4));
    const EIGTH_NOTE: Duration = Duration(Ratio::new_raw(1, 8));
    const SIXTEENTH_NOTE: Duration = Duration(Ratio::new_raw(1, 16));
    const THIRTYSECOND_NOTE: Duration = Duration(Ratio::new_raw(1, 32));
    const SIXTYFOURTH_NOTE: Duration = Duration(Ratio::new_raw(1, 64));
    const ONEHUNDREDTWENTYSECOND_NOTE: Duration = Duration(Ratio::new_raw(1, 128));

    pub(crate) fn to_ratio(self) -> Ratio<u32> {
        self.0
    }
}

pub struct Note {
    pub duration: NoteDuration,
}

pub struct Rhythm {
    pub segments: Vec<RhythmSegment>,
}
pub struct DoNotConstruct(());
pub enum RhythmSegment {
    Note(Note),
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
            RhythmSegment::Note(n) => n.duration.to_duration(),
            RhythmSegment::Rest(d) => d.to_duration(),
            RhythmSegment::Tuplet { actual: _, normal, note_duration, rhythm: _, do_not_construct: _ } => note_duration.to_duration() * Ratio::from_integer(*normal),
        }
    }
}

impl Rhythm {
    fn duration(&self) -> Duration {
        self.segments.iter().map(|s| s.duration()).sum()
    }
}
