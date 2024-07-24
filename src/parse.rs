use lalrpop_util::lalrpop_mod;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlDivElement, HtmlParagraphElement};

use crate::{polyrhythm::Polyrhythm, rhythm::TupletInnerDurationMismatch};

lalrpop_mod!(grammar);

pub enum RhythmError {
    TupletInnerDurationMismatch(TupletInnerDurationMismatch),
    InvalidNoteDuration(u32),
}

impl From<TupletInnerDurationMismatch> for RhythmError {
    fn from(v: TupletInnerDurationMismatch) -> Self {
        Self::TupletInnerDurationMismatch(v)
    }
}

pub fn parse(code: &str) -> Result<Polyrhythm, lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'_>, RhythmError>> {
    grammar::PolyrhythmParser::new().parse(code)
}

pub fn parse_error_to_div(document: &Document, error: lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'_>, RhythmError>) -> HtmlDivElement {
    let div: HtmlDivElement = document.create_element("div").expect("could not create div for error").dyn_into().expect("div should be able to be casted into div");
    let p: HtmlParagraphElement = document.create_element("p").expect("could not create p for error").dyn_into().expect("p should be able to be casted into p");

    let error_text = match error {
        lalrpop_util::ParseError::InvalidToken { location } => format!("invalid token at {location}"),
        lalrpop_util::ParseError::UnrecognizedEof { location, expected } => format!("unexpected eof at {location}; expected {}", expected.join(", ")),
        lalrpop_util::ParseError::UnrecognizedToken { token: (start, token, end), expected } => format!("unexpected token {token} at {start}-{end}; expected {}", expected.join(", ")),
        lalrpop_util::ParseError::ExtraToken { token: (start, token, end) } => format!("extra token {token} at {start}-{end}"),
        lalrpop_util::ParseError::User { error } => match error {
            RhythmError::TupletInnerDurationMismatch(TupletInnerDurationMismatch { actual, expected }) => format!("tuplet inner duration mismatch: expected duration of {expected} but got {actual}"),
            RhythmError::InvalidNoteDuration(duration) => format!("invalid note duration: {duration} (should be a power of 2)"),
        },
    };
    p.set_text_content(Some(&error_text));
    div.replace_children_with_node_1(&p);

    div
}
